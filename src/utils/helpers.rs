use crate::{
    components::{
        download::{AtomDownload, DownloadType},
        settings::AtomSettings,
    },
    messages::{DownloadProperties, Message},
    utils::json_from_browser::JSONFromBrowser,
};
use iced::futures::stream::unfold;
use iced::Subscription;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, ACCEPT_RANGES, CONTENT_LENGTH, USER_AGENT},
    Client, Method,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    io::{prelude::*, BufReader, Write},
    net::TcpListener,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{debug, warn};
use tray_icon::menu::MenuEvent;

pub const ATOM_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_2_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.72 Safari/537.36";
pub const ATOM_INPUT_DEFAULT_PADDING: u16 = 6;
pub const ATOM_SOCKET_ADDRESS: &'static str = "127.0.0.1:2866";

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlDownloads {
    pub downloads: Vec<AtomDownload>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlSettings {
    pub settings: AtomSettings,
}

pub fn get_epoch_ms() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as usize
}

pub fn get_formatted_time(time: u64) -> String {
    if time < 60 {
        format!("{:0>2} second(s)", time)
    } else if time < 3600 {
        format!("{:0>2}:{:0>2} min(s)", time / 60, time % 60)
    } else if time < (3600 * 24) {
        format!("{:0>2}:{:0>2} hour(s)", time / 3600, (time % 3600) / 60)
    } else {
        format!("{} day(s)", time / (3600 * 24))
    }
}

pub fn get_file_type(file_type: &str) -> String {
    match &file_type.to_lowercase()[..] {
        "jpg" | "jpeg" | "png" | "tiff" | "gif" | "webp" => "Image",
        "js" | "json" | "html" | "css" | "jsx" | "gulp" | "php" | "sass" | "scss" => {
            "Web Development"
        }
        "ttf" | "woff" | "woff2" | "otf" | "eot" => "Font File",
        "mp4" | "mkv" | "webm" | "ts" | "mov" | "avi" | "wmv" | "flv" | "f4v" | "swf" | "mpeg" => {
            "Video"
        }
        "ini" | "conf" | "toml" | "lock" | "xml" | "xhtml" | "xshtml" | "plist" => {
            "Configuration File"
        }
        "py" | "pyd" | "pyc" | "rs" | "java" | "vue" | "sh" | "bat" | "cmd" | "go" | "vim"
        | "c" | "cpp" | "h" | "hpp" => "Code File",
        "url" | "link" | "desktop" => "Shortcut File",
        "pdf" => "PDF File",
        "zip" | "gz" | "7z" | "rar" | "tar.gz" | "tar.xz" | "xz" => "Compressed Archive",
        "deb" | "exe" | "msi" | "rpm" => "Package Archive",
        "psd" => "Photoshop Project",
        "sql" | "csv" | "db" | "tsv" => "Database File",
        "txt" => "Text File",
        "mp3" | "wma" | "flac" | "midi" | "opus" | "m3u" | "ogg" | "oga" | "m4a" => "Audio",
        "md" => "ReadMe File",
        _ => "Unknown File",
    }
    .to_string()
}

pub fn get_relative_file_size(size: usize) -> String {
    if size < 1024 {
        format!("{} Bytes", size)
    } else if size < (1024 * 1024) {
        format!("{} KB", size / (1024))
    } else if size < (1024 * 1024 * 1024) {
        format!("{} MB", size / (1024 * 1024))
    } else {
        format!("{} GB", size / (1024 * 1024 * 1024))
    }
}

pub fn hashmap2headermap(headers: &HashMap<String, String>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    for entry in headers {
        if let (Ok(name), Ok(value)) = (
            HeaderName::from_lowercase(entry.0.to_lowercase().as_bytes()),
            HeaderValue::from_str(entry.1),
        ) {
            header_map.insert(name, value);
        }
    }
    header_map
}

pub async fn get_content_length(
    client: Client,
    link: &str,
    headers: &HashMap<String, String>,
) -> DownloadProperties {
    let mut size = DownloadProperties {
        content_length: 0,
        download_type: DownloadType::Sequential,
        error: "".to_string(),
    };

    match client
        .request(Method::HEAD, link)
        .header(USER_AGENT, ATOM_USER_AGENT)
        // .header("Referer", referrer)
        .headers(hashmap2headermap(headers))
        .send()
        .await
    {
        Ok(response) => {
            if !response.status().is_success() {
                size.error = "Error, unable to get content length!".to_string();
            } else {
                let headers = response.headers();
                match (headers.get(ACCEPT_RANGES), headers.get(CONTENT_LENGTH)) {
                    // todo:
                    // accept-ranges may be missing
                    (Some(_), Some(cl)) => {
                        let cl = cl
                            .to_str()
                            .unwrap_or_default()
                            .parse::<u64>()
                            .unwrap_or_default();

                        size.content_length = cl as usize;
                        size.download_type = DownloadType::Threaded;
                    }
                    (None, Some(cl)) => {
                        let cl = cl
                            .to_str()
                            .unwrap_or_default()
                            .parse::<u64>()
                            .unwrap_or_default();

                        size.content_length = cl as usize;
                        size.download_type = DownloadType::Sequential;
                    }
                    (_, _) => {
                        size.content_length = 0;
                        size.download_type = DownloadType::Sequential;
                    }
                }
            }
        }
        Err(_) => {
            size.content_length = 0;
            size.download_type = DownloadType::Sequential;
        }
    }

    size
}

/**
 * get user's downloads directory, if param is passed that would be appended to the download path
 */
pub fn get_downloads_directory(file_name: &str) -> String {
    let downloads_dir = if let Some(downloads_dir) = directories::UserDirs::new() {
        downloads_dir
            .download_dir()
            .unwrap_or_else(|| Path::new("./"))
            .to_owned()
    } else {
        Path::new("./").to_owned()
    };

    let file_path = downloads_dir
        .join(file_name)
        .to_str()
        .unwrap_or("./")
        .to_string();

    file_path
}

/**
 * get user's cache dir for storing temporary files
 */
pub fn get_conf_directory<'a>() -> Result<PathBuf, &'a str> {
    directories::BaseDirs::new().map_or_else(
        || Err("basedir failed (get_conf_directory)!"),
        |basedirs| Ok(basedirs.cache_dir().join("atom")),
    )
}

/**
 * opens specified file according to the OS
 */
pub fn open_file(file: &str) {
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer.exe")
        .arg(file)
        .spawn()
        .ok();
    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(file).spawn().ok();
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(file)
        .spawn()
        .ok();
}

/**
 * show desktop notification
 */
pub fn show_notification(subtitle: &str, body: &str, timeout: u32) {
    if notify_rust::Notification::new()
        .summary("A.T.O.M")
        .subtitle(subtitle)
        .auto_icon()
        .body(body)
        .icon("atom")
        .timeout(notify_rust::Timeout::Milliseconds(timeout))
        .show()
        .is_err()
    {
        debug!(
            "[ATOM] : notification error: subtitle: {subtitle}, body: {body}, timeout: {timeout}!"
        );
    }
}

pub fn split_file_name(file_name: &str, split_count: u8) -> Vec<String> {
    (1..=split_count)
        .map(|index| format!("{}.atom.{}", file_name, index))
        .collect()
}

pub fn save_settings_toml(settings: &AtomSettings) -> bool {
    let toml_settings = TomlSettings {
        settings: settings.to_owned(),
    };

    if let Ok(serialized) = toml::to_string(&toml_settings) {
        let path = PathBuf::from(&toml_settings.settings.config_dir);
        if !path.exists() && std::fs::create_dir_all(&path).is_err() {
            return false;
        }
        let path = path.join("settings.toml");

        if std::fs::write(path, serialized).is_err() {
            return false;
        }
    }
    true
}

pub fn save_downloads_toml(downloads: Vec<AtomDownload>, toml_path: &PathBuf) -> bool {
    let toml_downloads = TomlDownloads { downloads };

    if let Ok(serialized) = toml::to_string(&toml_downloads) {
        if std::fs::write(toml_path, serialized).is_err() {
            return false;
        }
    }
    true
}

pub fn parse_settings_toml(settings_path: &PathBuf) -> AtomSettings {
    std::fs::read_to_string(settings_path).map_or_else(
        |_| AtomSettings::default(),
        |contents| {
            toml::from_str::<TomlSettings>(&contents).map_or_else(
                |_| AtomSettings::default(),
                |toml_settings| toml_settings.settings,
            )
        },
    )
}

pub fn parse_downloads_toml(downloads_file_path: &PathBuf) -> BTreeMap<usize, AtomDownload> {
    let mut downloads: BTreeMap<usize, AtomDownload> = BTreeMap::new();

    if let Ok(contents) = std::fs::read_to_string(downloads_file_path) {
        if let Ok(deserialized) = toml::from_str::<TomlDownloads>(&contents) {
            deserialized
                .downloads
                .into_iter()
                .enumerate()
                .for_each(|(index, download)| {
                    downloads.insert(get_epoch_ms() + index, download);
                });
        }
    }

    downloads
}

pub fn handle_web_request() -> Subscription<Message> {
    #[derive(Debug)]
    enum RequestStates {
        Start,
        Listen(TcpListener),
    }

    Subscription::run_with_id(
        10000000,
        unfold(RequestStates::Start, move |state| async move {
            match state {
                RequestStates::Start => {
                    let message = TcpListener::bind(ATOM_SOCKET_ADDRESS).map_or_else(
                        |e| {
                            warn!("Error: TcpListener::bind(failed) ({e:#?})");
                            Some((Message::StatusBar("TcpListener failed, capturing downloads from the browser will not work".to_string()), RequestStates::Start))
                        },
                        |listener| Some((Message::StatusBar("Capturing downloads from the browser enabled".to_string()), RequestStates::Listen(listener))),
                    );
                    return message;
                }
                RequestStates::Listen(listener) => listener.accept().map_or_else(
                    |err| {
                        std::net::TcpStream::connect(ATOM_SOCKET_ADDRESS).ok();
                        warn!("TCP Error: {:#?}", err);
                        Some((Message::StatusBar("TcpListener failed, capturing downloads from the browser will not work".to_string()), RequestStates::Start))
                    },
                    |(stream, _)| {
                        let start_message = Some((Message::Ignore, RequestStates::Start));

                        let mut stream = stream;
                        let buf_reader = BufReader::new(&mut stream);
                        let http_request: Vec<_> = buf_reader
                            .lines()
                            .map(|result| result.unwrap())
                            .take_while(|line| !line.ends_with("<END>"))
                            .collect();

                        let response = "HTTP/1.1 200 OK\r\n\r\n";
                        stream.write_all(response.as_bytes()).ok();

                        if http_request.is_empty() {
                            return start_message;
                        }

                        let json = http_request.last().unwrap();
                        let json = serde_json::from_str::<JSONFromBrowser>(json);
                        if json.is_err() {
                            warn!("TCP JSON error : {:?}", json);
                            return start_message;
                        }

                        let json = json.unwrap();
                        if json.file_name.is_empty() || json.url.is_empty() {
                            return start_message;
                        }

                        Some((
                            Message::NewDownloadReceivedFromBrowser(json),
                            RequestStates::Listen(listener),
                        ))
                    },
                ),
            }
        }),
    )
}

pub fn load_tray_icon(image_data: &[u8]) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(image_data)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

pub fn listen_for_tray_events() -> Subscription<Message> {
    Subscription::run_with_id(
        1001,
        unfold("", move |_| async move {
            if let Ok(event) = MenuEvent::receiver().try_recv() {
                Some((Message::TrayEvent(event.id), ""))
            } else {
                // None
                Some((Message::Ignore, ""))
            }
        }),
    )
}
