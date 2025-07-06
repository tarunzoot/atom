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
    pub auto_open: bool,
    pub auto_remove: bool,
    pub is_mouse_over_heading: bool,
}

impl AtomDownloadForm {
    pub fn new(download: AtomDownload, settings: &AtomSettings) -> Self {
        Self {
            url: download.url,
            #[cfg(target_os = "windows")]
            file_name: format!("{}\\{}", settings.downloads_dir, download.file_name),
            #[cfg(not(target_os = "windows"))]
            file_name: format!("{}/{}", settings.downloads_dir, download.file_name),
            size: download.size,
            headers: download.headers,
            sequential: download.size == 0 || download.sequential,
            is_valid_url: true,
            auto_remove: false,
            is_mouse_over_heading: false,
            ..Default::default()
        }
    }

    pub fn make_download(&self) -> Result<AtomDownload, &str> {
        AtomDownload::new()
            .url(&self.url)
            .auto_set_file_name_path(&self.file_name)
            .file_size(self.size)
            .headers(self.headers.clone())
            .download_type(self.sequential)
            .auto_open(self.auto_open)
            .auto_remove(self.auto_remove)
            .build()
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }
}
