mod update;
mod view;
use crate::components::{download::AtomDownload, settings::AtomSettings};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AtomDownloadForm {
    pub url: String,
    pub file_name: String,
    pub size: usize,
    pub sequential: bool,
    pub headers: HashMap<String, String>,
    pub is_valid_url: bool,
    pub header_name: String,
    pub header_value: String,
    pub auto_referer: bool,
}

impl AtomDownloadForm {
    pub fn new(download: AtomDownload, settings: &AtomSettings) -> Self {
        Self {
            url: download.url,
            file_name: format!("{}/{}", settings.downloads_dir, download.file_name),
            size: download.size,
            headers: download.headers,
            sequential: download.size == 0,
            is_valid_url: true,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }
}
