mod subscription;
mod update;
mod view;
use crate::messages::DownloadMessage;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, time::SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadMethod {
    Get,
    Post,
}

#[derive(Debug)]
pub enum DownloadType {
    Sequential,
    Threaded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomDownload {
    pub url: String,
    pub method: DownloadMethod,
    pub file_path: String,
    pub file_name: String,
    pub downloaded: usize,
    #[serde(skip_deserializing, skip_serializing)]
    pub download_this_session: usize,
    pub size: usize,
    pub downloading: bool,
    pub threads: u8,
    pub error: String,
    pub deleted: bool,
    pub sequential: bool,
    pub added: String,
    pub headers: HashMap<String, String>,
    pub request_body: String,
    pub transfer_rate: f64,
    pub eta: f64,
    pub auto_open: bool,
    #[serde(skip_deserializing, skip_serializing)]
    pub joined_bytes: usize,
    #[serde(skip_deserializing, skip_serializing)]
    pub elapsed_time: Option<SystemTime>,
    #[serde(skip_deserializing, skip_serializing)]
    pub joining: bool,
    #[serde(skip_deserializing, skip_serializing)]
    pub show_delete_confirm_dialog: bool,
}

impl Default for AtomDownload {
    fn default() -> Self {
        Self {
            url: String::default(),
            method: DownloadMethod::Get,
            file_path: String::default(),
            file_name: String::default(),
            downloaded: 0,
            download_this_session: 0,
            size: 0,
            downloading: true,
            threads: 6,
            error: String::default(),
            deleted: false,
            sequential: false,
            added: chrono::Local::now().date_naive().to_string(),
            headers: HashMap::default(),
            request_body: String::default(),
            transfer_rate: 0.0,
            eta: 0.0,
            elapsed_time: Some(SystemTime::now()),
            joined_bytes: 0,
            joining: false,
            show_delete_confirm_dialog: false,
            auto_open: false,
        }
    }
}

impl AtomDownload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url<T: Into<String>>(mut self, url: T) -> Self {
        self.url = url.into();
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn auto_open(mut self, open: bool) -> Self {
        self.auto_open = open;
        self
    }

    pub fn file_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn file_name<T: Into<String>>(mut self, file_name: T) -> Self {
        self.file_name = file_name.into();
        self
    }

    pub fn file_path<T: Into<String>>(mut self, file_path: T) -> Self {
        self.file_path = file_path.into();
        self
    }

    pub fn download_type(mut self, sequential: bool) -> Self {
        self.sequential = sequential;
        self
    }

    pub fn request_body(mut self, body: String) -> Self {
        self.request_body = body;
        self.method = DownloadMethod::Post;
        self
    }

    pub fn auto_set_file_name_path(mut self, full_path: &str) -> Self {
        let full_path = std::path::PathBuf::from(full_path);
        self.file_name = full_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        self.file_path = full_path
            .parent()
            .unwrap_or_else(|| Path::new(&full_path))
            .to_str()
            .unwrap_or_default()
            .to_string();
        self
    }

    pub fn build<'a>(mut self) -> Result<Self, &'a str> {
        if self.file_name.is_empty() {
            if let Ok(url) = reqwest::Url::parse(&self.url) {
                if let Some(mut file_name) = url.path_segments() {
                    if let Some(file_name) = file_name.next_back() {
                        urlencoding::decode(file_name)
                            .map(|file_name| {
                                self.file_name = file_name.into();
                            })
                            .ok();
                    }
                }
            }
        }

        if self.file_name.is_empty() || self.file_path.is_empty() {
            Err("AtomDownload has empty filename or path!")
        } else {
            Ok(self)
        }
    }

    pub fn get_url(&self) -> String {
        self.url.to_string()
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.to_string()
    }

    pub fn get_download_size(&self) -> usize {
        self.size
    }

    pub fn is_downloaded(&self) -> bool {
        self.downloaded >= self.size || self.deleted
    }

    pub fn is_downloading(&self) -> bool {
        self.downloading
    }
}
