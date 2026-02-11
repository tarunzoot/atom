use super::{AtomDownload, DownloadType};
use crate::{
    messages::{DownloadMessage, DownloadProperties},
    utils::helpers::{get_content_length, hashmap2headermap, split_file_name, ATOM_USER_AGENT},
};
use iced::{
    futures::stream::{self, Stream},
    Subscription,
};
use reqwest::{
    header::{RANGE, USER_AGENT},
    Client, Method, Response,
};
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};
use tracing::{debug, error, warn};

const BUFFER_SIZE: usize = 1024 * 1000;

#[derive(Debug, Clone)]
struct DownloadSubData {
    index: usize,
    client: Client,
    download: AtomDownload,
    cache_dir: String,
}

impl std::hash::Hash for DownloadSubData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.download.url.hash(state);
    }
}

#[derive(Debug)]
struct SubDownloads {
    response: Response,
    file: BufWriter<File>,
}

#[derive(Debug)]
enum State {
    Starting(Client, AtomDownload, String),
    ThreadedStarting(Client, AtomDownload, String, Vec<String>),
    SequentialDownloading(Response, BufWriter<File>, usize),
    ThreadedDownloading(AtomDownload, Vec<SubDownloads>, String, Vec<String>, usize),
    FileJoining(BufWriter<File>, VecDeque<String>, BufReader<File>, Vec<u8>),
    SequentialFinished,
    ThreadedFinished(String, Vec<String>),
    Wait,
}

impl AtomDownload {
    #[tracing::instrument(name = "Subscription", skip(self))]
    pub fn subscription(
        &self,
        index: usize,
        cache_dir: &Path,
        client: Client,
    ) -> Subscription<DownloadMessage> {
        if !self.downloading && (self.sequential || !self.joining) {
            return Subscription::none();
        }

        debug!(download=?self);
        self.unfold_subscription(client, cache_dir.to_string_lossy().to_string(), index)
    }

    #[tracing::instrument]
    fn download_stream(data: &DownloadSubData) -> impl Stream<Item = DownloadMessage> {
        let state = State::Starting(
            data.client.clone(),
            data.download.clone(),
            data.cache_dir.clone(),
        );

        stream::unfold(state, move |state| async move {
            match state {
                // State::Wait => iced::futures::future::pending().await,
                State::Wait => None,
                // State::SequentialFinished => Some((DownloadMessage::Ignore, State::Wait)),
                State::SequentialFinished => None,

                State::ThreadedFinished(destination_file, files) => {
                    Some(handle_threaded_download_finish(&destination_file, files))
                }

                State::ThreadedDownloading(
                    download,
                    sub_downloads,
                    destination_file,
                    chunk_files,
                    downloaded,
                ) => Some(
                    handle_threaded_downloading(
                        download,
                        sub_downloads,
                        destination_file,
                        chunk_files,
                        downloaded,
                    )
                    .await,
                ),

                State::ThreadedStarting(client, download, destination_file, chunk_files) => Some(
                    handle_threaded_download_starting(
                        download,
                        destination_file,
                        chunk_files,
                        client,
                    )
                    .await,
                ),

                State::SequentialDownloading(response, file, downloaded) => {
                    Some(handle_sequential_downloading(response, file, downloaded).await)
                }

                State::Starting(client, download, cache_dir) => {
                    Some(handle_download_starting(download, client, cache_dir).await)
                }

                State::FileJoining(bw, chunk_files, chunk_files_reader, buffer) => {
                    Some(handle_joining_progress(bw, chunk_files, chunk_files_reader, buffer).await)
                }
            }
        })
    }

    fn unfold_subscription(
        &self,
        client: Client,
        cache_dir: String,
        index: usize,
    ) -> Subscription<DownloadMessage> {
        Subscription::run_with(
            DownloadSubData {
                index,
                download: self.clone(),
                client,
                cache_dir,
            },
            AtomDownload::download_stream,
        )
    }
}

async fn handle_download_starting(
    mut download: AtomDownload,
    client: Client,
    cache_dir: String,
) -> (DownloadMessage, State) {
    let file_path = Path::new(&download.file_path)
        .join(&download.file_name)
        .to_str()
        .unwrap_or_default()
        .to_string();

    let mut options = DownloadProperties {
        content_length: download.size,
        download_type: if download.sequential {
            DownloadType::Sequential
        } else {
            DownloadType::Threaded
        },
        error: "".to_string(),
    };

    if download.downloaded == 0 && download.size == 0 {
        options = get_content_length(client.clone(), &download.url, &download.headers).await;
    }

    if !options.error.is_empty() {
        return (DownloadMessage::Error(options.error), State::Wait);
    }

    match (options.download_type, download.sequential) {
        (DownloadType::Threaded, false) => {
            let cache_dir_path = PathBuf::from(&cache_dir);
            let files: Vec<String> = split_file_name(&download.file_name, download.threads)
                .iter()
                .map(|m| cache_dir_path.join(m).to_string_lossy().to_string())
                .collect();

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
                DownloadMessage::SetFileSize(options.content_length, downloaded_bytes_len as usize),
                State::ThreadedStarting(client, download, file_path, files),
            )
        }
        _ => {
            if let Ok(file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)
            {
                let mut file_size = 0;
                let mut client = client
                    .request(
                        match download.method {
                            super::DownloadMethod::Get => Method::GET,
                            super::DownloadMethod::Post => Method::POST,
                        },
                        &download.url,
                    )
                    .header(USER_AGENT, ATOM_USER_AGENT)
                    .headers(hashmap2headermap(&download.headers));

                if let super::DownloadMethod::Post = download.method {
                    client = client.body(download.request_body);
                }

                download.size = options.content_length;

                if options.content_length != 0 {
                    file_size = file.metadata().unwrap().len() as usize;
                    if file_size >= download.size {
                        return (DownloadMessage::Finished, State::Wait);
                    }
                    client = client.header(
                        RANGE,
                        format!("bytes={}-{}", file_size, options.content_length),
                    );
                }

                if let Ok(response) = client.send().await {
                    (
                        DownloadMessage::SetFileSize(options.content_length, file_size),
                        State::SequentialDownloading(response, BufWriter::new(file), file_size),
                    )
                } else {
                    (
                        DownloadMessage::Error("Failed to initialize download client.".to_string()),
                        State::Wait,
                    )
                }
            } else {
                (
                    DownloadMessage::Error(format!(
                        "Failed to create download file {}.",
                        download.file_name
                    )),
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
) -> (DownloadMessage, State) {
    let chunk = response.chunk().await;

    if !response.status().is_success() {
        return (
            DownloadMessage::Error(format!(
                "The server returned a non-success({}) status code.",
                response.status()
            )),
            State::Wait,
        );
    }

    match chunk {
        Ok(Some(chunk)) => {
            if file.write_all(&chunk[..]).is_err() {
                return (
                    DownloadMessage::Error("error occurred while downloading!".to_string()),
                    State::Wait,
                );
            }
            downloaded += chunk.len();

            (
                DownloadMessage::DownloadProgress(downloaded),
                State::SequentialDownloading(response, file, downloaded),
            )
        }
        Ok(None) => (DownloadMessage::Finished, State::Wait),
        Err(error) => (
            DownloadMessage::Error(format!(
                "Failed to retrieve chunk from the server: {:?}.",
                error
            )),
            State::Wait,
        ),
    }
}

#[tracing::instrument]
async fn handle_threaded_download_starting(
    download: AtomDownload,
    destination_file: String,
    chunk_files: Vec<String>,
    client: Client,
) -> (DownloadMessage, State) {
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
            .append(true)
            .open(f)
        {
            let mut file_len: usize = 0;
            if let Ok(meta) = file.metadata() {
                file_len = meta.len() as usize;
            }
            downloaded += file_len;

            if file_len >= chunk_size || (previous_chunk_start + file_len) >= download.size {
                continue;
            }

            let mut client = client
                .request(
                    match download.method {
                        super::DownloadMethod::Get => Method::GET,
                        super::DownloadMethod::Post => Method::POST,
                    },
                    &download.url,
                )
                // .get(&download.url)
                .header(USER_AGENT, ATOM_USER_AGENT)
                .headers(hashmap2headermap(&download.headers));

            if let super::DownloadMethod::Post = download.method {
                client = client.form(&download.request_body);
            };

            client = client.header(
                RANGE,
                format!(
                    "bytes={}-{}",
                    previous_chunk_start + file_len,
                    if (i + 1) == (download.threads as usize) {
                        download.size
                    } else {
                        previous_chunk_start + chunk_size
                    }
                ),
            );
            open_chunk_files.push(file);

            responses.push(client.send());
        } else {
            return (
                DownloadMessage::Error(format!("Failed to create {} file.", download.file_name)),
                State::Wait,
            );
        }
    }

    for (response, file) in responses.into_iter().zip(open_chunk_files) {
        match response.await {
            Ok(response) => {
                if response.status().is_success() {
                    sub_downloads.push(SubDownloads {
                        response,
                        file: BufWriter::new(file),
                    });
                } else {
                    return (
                        DownloadMessage::Error(format!(
                            "The server returned an error status code ({}) for the requested resource.",
                            response.status()
                        )),
                        State::Wait,
                    );
                }
            }
            Err(e) => {
                return (
                    DownloadMessage::Error(format!("Request failed with error : {}.", e)),
                    State::Wait,
                );
            }
        }
    }

    (
        DownloadMessage::DownloadProgress(downloaded),
        State::ThreadedDownloading(
            download,
            sub_downloads,
            destination_file,
            chunk_files,
            downloaded,
        ),
    )
}

#[tracing::instrument]
async fn handle_threaded_downloading(
    download: AtomDownload,
    sub_downloads: Vec<SubDownloads>,
    destination_file: String,
    chunk_files: Vec<String>,
    mut downloaded: usize,
) -> (DownloadMessage, State) {
    let mut filtered_sub_downloads = vec![];

    for mut sub_download in sub_downloads.into_iter() {
        match sub_download.response.chunk().await {
            Ok(Some(chunk)) => {
                if sub_download.file.write_all(&chunk[..]).is_err() {
                    return (
                        DownloadMessage::Error("Failed to write to chunk file.".to_string()),
                        State::Wait,
                    );
                }
                downloaded += chunk.len();
                filtered_sub_downloads.push(sub_download);
            }
            Ok(None) => {}
            Err(error) => {
                return (
                    DownloadMessage::Error(format!("Download failed : {:?}", error)),
                    State::Wait,
                )
            }
        }
    }

    if filtered_sub_downloads.is_empty() {
        (
            DownloadMessage::DownloadDoneJoining,
            State::ThreadedFinished(destination_file, chunk_files),
        )
    } else {
        (
            DownloadMessage::DownloadProgress(downloaded),
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

#[tracing::instrument]
fn handle_threaded_download_finish(
    destination_file: &str,
    chunk_files: Vec<String>,
) -> (DownloadMessage, State) {
    let error_msg = (
        DownloadMessage::Error("Failed to join downloaded file segments.".to_string()),
        State::Wait,
    );

    let mut chunk_file_names = VecDeque::from(chunk_files);

    match File::create(destination_file) {
        Ok(out) => {
            if let Some(file) = chunk_file_names.pop_front() {
                if let Ok(f) = File::open(&file) {
                    let chunk_file_reader = BufReader::with_capacity(BUFFER_SIZE, f);
                    let buffer = vec![0; BUFFER_SIZE];
                    (
                        DownloadMessage::JoiningProgress(0),
                        State::FileJoining(
                            BufWriter::with_capacity(BUFFER_SIZE, out),
                            chunk_file_names,
                            chunk_file_reader,
                            buffer,
                        ),
                    )
                } else {
                    error!("[ATOM] failed to open : '{file:#?}'");
                    error_msg
                }
            } else {
                error!("ATOM: pop_front from the chunk file names vector returned None");
                error_msg
            }
        }
        Err(error) => {
            error!("[ATOM] : {}", error);
            error_msg
        }
    }
}

#[tracing::instrument(name = "HandleJoiningProgress", skip(buffer))]
async fn handle_joining_progress(
    mut bw: BufWriter<File>,
    mut chunk_files: VecDeque<String>,
    mut chunk_files_reader: BufReader<File>,
    mut buffer: Vec<u8>,
) -> (DownloadMessage, State) {
    let error_msg = (
        DownloadMessage::Error("Failed to join downloaded file segments.".to_string()),
        State::Wait,
    );
    let finished_msg = (DownloadMessage::Finished, State::SequentialFinished);
    let mut copied = 0;

    if let Ok(_read) = chunk_files_reader.read(&mut buffer) {
        if bw.write_all(&buffer[.._read]).is_err() {
            return error_msg;
        }

        copied += _read;

        if _read < BUFFER_SIZE {
            if let Some(first) = chunk_files.pop_front() {
                if let Ok(file) = File::open(&first) {
                    chunk_files_reader = BufReader::with_capacity(BUFFER_SIZE, file);
                } else {
                    return error_msg;
                }
            } else if _read == 0 {
                return finished_msg;
            }
        }
    }

    (
        DownloadMessage::JoiningProgress(copied),
        State::FileJoining(bw, chunk_files, chunk_files_reader, buffer),
    )
}
