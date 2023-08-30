use super::{AtomDownload, DownloadType};
use crate::{
    messages::{DownloadMessage, DownloadProperties, Message},
    utils::helpers::{get_content_length, hashmap2headermap, split_file_name, ATOM_USER_AGENT},
};
use iced::Subscription;
use reqwest::{
    header::{RANGE, USER_AGENT},
    Client, Response,
};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

struct SubDownloads {
    response: Response,
    file: BufWriter<File>,
}

enum State {
    Starting(Client, AtomDownload, String),
    ThreadedStarting(Client, AtomDownload, String, Vec<String>),
    SequentialDownloading(Response, BufWriter<File>, usize),
    ThreadedDownloading(AtomDownload, Vec<SubDownloads>, String, Vec<String>, usize),
    FileJoining(BufWriter<File>, Vec<BufReader<File>>, usize),
    SequentialFinished,
    ThreadedFinished(String, Vec<String>),
    Wait,
}

impl AtomDownload {
    pub fn subscription(&self, index: usize, cache_dir: &Path) -> Subscription<Message> {
        if !self.is_downloading && (self.is_sequential || !self.is_joining) {
            return Subscription::none();
        }

        let client_builder = reqwest::ClientBuilder::new();
        let client = if let Ok(client) = client_builder
            .danger_accept_invalid_certs(true)
            .referer(true)
            .build()
        {
            client
        } else {
            return Subscription::none();
        };

        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&self.url[..]);
        let sha256_digest = sha256_hasher.finalize();

        iced::subscription::unfold(
            sha256_digest,
            State::Starting(
                client,
                self.clone(),
                cache_dir.to_string_lossy().to_string(),
            ),
            move |state| async move {
                match state {
                    State::Wait => iced::futures::future::pending().await,
                    State::SequentialFinished => (
                        Message::Download(DownloadMessage::Finished, index),
                        State::Wait,
                    ),
                    State::ThreadedFinished(destination_file, files) => {
                        handle_threaded_download_finish(&destination_file, &files, index)
                    }
                    State::ThreadedDownloading(
                        download,
                        sub_downloads,
                        destination_file,
                        chunk_files,
                        downloaded,
                    ) => {
                        handle_threaded_downloading(
                            download,
                            sub_downloads,
                            destination_file,
                            chunk_files,
                            downloaded,
                            index,
                        )
                        .await
                    }
                    State::ThreadedStarting(client, download, destination_file, chunk_files) => {
                        handle_threaded_download_starting(
                            download,
                            destination_file,
                            chunk_files,
                            client,
                            index,
                        )
                        .await
                    }
                    State::SequentialDownloading(response, file, downloaded) => {
                        handle_sequential_downloading(response, file, downloaded, index).await
                    }
                    State::Starting(client, download, cache_dir) => {
                        handle_download_starting(download, client, cache_dir, index).await
                    }
                    State::FileJoining(bw, chunk_files, index) => {
                        handle_joining_progress(bw, chunk_files, index).await
                    }
                }
            },
        )
    }
}

async fn handle_download_starting(
    mut download: AtomDownload,
    client: Client,
    cache_dir: String,
    index: usize,
) -> (Message, State) {
    let file_path = Path::new(&download.file_path)
        .join(&download.file_name)
        .to_str()
        .unwrap_or_default()
        .to_string();

    let options = if download.is_sequential && download.size > 0 {
        DownloadProperties {
            content_length: download.size,
            download_type: DownloadType::Sequential,
            error: "".to_string(),
        }
    } else if download.downloaded > 0 && !download.is_sequential && download.size > 0 {
        DownloadProperties {
            content_length: download.size,
            download_type: DownloadType::Threaded,
            error: "".to_string(),
        }
    } else {
        get_content_length(&download.url, &download.headers).await
    };

    if !options.error.is_empty() {
        return (
            Message::Download(DownloadMessage::Error(options.error), index),
            State::Wait,
        );
    }

    match (options.download_type, download.is_sequential) {
        (DownloadType::Threaded, false) => {
            #[cfg(target_os = "windows")]
            let cache_file_path = format!(
                "{}\\{}",
                cache_dir,
                std::path::PathBuf::from(&download.file_name)
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
            );
            #[cfg(not(target_os = "windows"))]
            let cache_file_path = format!("{}/{}", cache_dir, &download.file_name);

            let threads = download.threads;
            let files = split_file_name(&cache_file_path, threads);
            let downloaded_bytes_len = files.iter().fold(0, |size, file_path| {
                if let Ok(file) = std::fs::OpenOptions::new()
                    .create(false)
                    .write(false)
                    .append(true)
                    .open(file_path)
                {
                    return size + file.metadata().map_err(|_| Some(0)).unwrap().len();
                }
                size
            });

            download.size = options.content_length;
            (
                Message::Download(
                    DownloadMessage::SetFileSize(
                        options.content_length,
                        downloaded_bytes_len as usize,
                    ),
                    index,
                ),
                State::ThreadedStarting(client, download, file_path, files),
            )
        }
        _ => {
            if let Ok(file) = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&file_path)
            {
                let mut file_size = 0;
                let mut client = client
                    .get(&download.url)
                    .header(USER_AGENT, ATOM_USER_AGENT)
                    .headers(hashmap2headermap(&download.headers));

                download.size = options.content_length;

                if options.content_length != 0 {
                    file_size = file.metadata().unwrap().len() as usize;
                    client = client.header(
                        "Range",
                        format!("bytes={}-{}", file_size, options.content_length),
                    );
                }

                if let Ok(response) = client.send().await {
                    (
                        Message::Download(
                            DownloadMessage::SetFileSize(options.content_length, file_size),
                            index,
                        ),
                        State::SequentialDownloading(response, BufWriter::new(file), file_size),
                    )
                } else {
                    (
                        Message::Download(
                            DownloadMessage::Error("failed to create download client!".to_string()),
                            index,
                        ),
                        State::Wait,
                    )
                }
            } else {
                (
                    Message::Download(
                        DownloadMessage::Error(format!("failed to create {}!", download.file_name)),
                        index,
                    ),
                    State::Wait,
                )
            }
        }
    }
}

async fn handle_sequential_downloading(
    mut response: Response,
    mut file: BufWriter<File>,
    mut downloaded: usize,
    index: usize,
) -> (Message, State) {
    match response.chunk().await {
        Ok(Some(chunk)) => {
            if file.write_all(&chunk[..]).is_err() {
                return (
                    Message::Download(
                        DownloadMessage::Error("error occurred while downloading!".to_string()),
                        index,
                    ),
                    State::Wait,
                );
            }
            downloaded += chunk.len();

            (
                Message::Download(DownloadMessage::DownloadProgress(downloaded), index),
                State::SequentialDownloading(response, file, downloaded),
            )
        }
        Ok(None) => (
            Message::Download(DownloadMessage::Finished, index),
            State::Wait,
        ),
        Err(error) => (
            Message::Download(
                DownloadMessage::Error(format!("download error : {:?}", error)),
                index,
            ),
            State::Wait,
        ),
    }
}

async fn handle_threaded_download_starting(
    download: AtomDownload,
    destination_file: String,
    chunk_files: Vec<String>,
    client: Client,
    index: usize,
) -> (Message, State) {
    let mut sub_downloads: Vec<SubDownloads> = vec![];
    let mut downloaded = 0;
    let chunk_size = download.size / (download.threads as usize);
    let mut previous_chunk_start;

    let mut responses = vec![];
    let mut open_chunk_files = vec![];

    for (i, f) in chunk_files.iter().enumerate() {
        previous_chunk_start = (i * chunk_size) + i;
        if let Ok(file) = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(f)
        {
            let file_len = file.metadata().unwrap().len() as usize;
            let client = client
                .get(&download.url)
                .header(USER_AGENT, ATOM_USER_AGENT)
                .headers(hashmap2headermap(&download.headers))
                .header(
                    RANGE,
                    format!(
                        "bytes={}-{}",
                        previous_chunk_start + file_len,
                        if previous_chunk_start + chunk_size >= download.size {
                            download.size
                        } else {
                            previous_chunk_start + chunk_size
                        }
                    ),
                );
            downloaded += file_len;
            open_chunk_files.push(file);
            responses.push(client.send());
        } else {
            return (
                Message::Download(
                    DownloadMessage::Error(format!(
                        "Error: failed to create {}!",
                        download.file_name
                    )),
                    index,
                ),
                State::Wait,
            );
        }
    }

    for (response, file) in responses.into_iter().zip(open_chunk_files) {
        if let Ok(response) = response.await {
            sub_downloads.push(SubDownloads {
                response,
                file: BufWriter::new(file),
            });
        } else {
            return (
                Message::Download(
                    DownloadMessage::Error(format!(
                        "failed to create request for {}!",
                        download.file_name
                    )),
                    index,
                ),
                State::Wait,
            );
        }
    }

    (
        Message::Download(DownloadMessage::DownloadProgress(downloaded), index),
        State::ThreadedDownloading(
            download,
            sub_downloads,
            destination_file,
            chunk_files,
            downloaded,
        ),
    )
}

async fn handle_threaded_downloading(
    download: AtomDownload,
    sub_downloads: Vec<SubDownloads>,
    destination_file: String,
    chunk_files: Vec<String>,
    mut downloaded: usize,
    index: usize,
) -> (Message, State) {
    let mut filtered_sub_downloads = vec![];

    for mut sub_download in sub_downloads.into_iter() {
        match sub_download.response.chunk().await {
            Ok(Some(chunk)) => {
                if sub_download.file.write_all(&chunk[..]).is_err() {
                    return (
                        Message::Download(
                            DownloadMessage::Error("writing to chunk file failed!".to_string()),
                            index,
                        ),
                        State::Wait,
                    );
                }
                downloaded += chunk.len();
                filtered_sub_downloads.push(sub_download);
            }
            Ok(None) => {}
            Err(error) => {
                return (
                    Message::Download(
                        DownloadMessage::Error(format!("download error : {:?}", error)),
                        index,
                    ),
                    State::Wait,
                )
            }
        }
    }

    if filtered_sub_downloads.is_empty() {
        (
            Message::Download(DownloadMessage::DownloadDoneJoining, index),
            State::ThreadedFinished(destination_file, chunk_files),
        )
    } else {
        (
            Message::Download(DownloadMessage::DownloadProgress(downloaded), index),
            State::ThreadedDownloading(
                download,
                filtered_sub_downloads,
                destination_file,
                chunk_files,
                downloaded,
            ),
        )
    }
}

fn handle_threaded_download_finish(
    destination_file: &str,
    chunk_files: &Vec<String>,
    index: usize,
) -> (Message, State) {
    let error_msg = (
        Message::Download(
            DownloadMessage::Error("Error in joining file!".to_string()),
            index,
        ),
        State::Wait,
    );
    match File::create(destination_file) {
        Ok(out) => {
            let mut chunk_file_handles = vec![];
            for file in chunk_files {
                if let Ok(f) = File::open(file) {
                    chunk_file_handles.push(BufReader::new(f));
                } else {
                    log::error!("[ATOM] : opening {} failed!", file);
                    return error_msg;
                }
            }

            (
                Message::Download(DownloadMessage::JoiningProgress(0), index),
                State::FileJoining(BufWriter::new(out), chunk_file_handles, 0),
            )
        }
        Err(error) => {
            log::error!("[ATOM] : {}", error);
            error_msg
        }
    }
}

async fn handle_joining_progress(
    mut bw: BufWriter<File>,
    mut chunk_files: Vec<BufReader<File>>,
    index: usize,
) -> (Message, State) {
    let error_msg = (
        Message::Download(
            DownloadMessage::Error("Error in joining file!".to_string()),
            index,
        ),
        State::Wait,
    );

    let buffer_len = 10000000;
    let mut copied = 0;

    if chunk_files.is_empty() {
        return (
            Message::Download(DownloadMessage::Finished, index),
            State::SequentialFinished,
        );
    }

    let mut br = chunk_files.remove(0);
    let mut buffer = vec![0; buffer_len];
    while let Ok(_read) = br.read(&mut buffer) {
        if bw.write_all(&buffer[.._read]).is_err() {
            return error_msg;
        }
        copied += _read;

        if _read < buffer_len {
            break;
        }
    }

    (
        Message::Download(DownloadMessage::JoiningProgress(copied), index),
        State::FileJoining(bw, chunk_files, index),
    )
}
