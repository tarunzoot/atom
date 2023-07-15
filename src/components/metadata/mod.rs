mod view;
use crate::components::download::AtomDownload;
use std::path::Path;

#[derive(Debug, Default)]
pub struct AtomDownloadMetadata {
    pub enabled: bool,
    pub url: String,
    pub extension: String,
    pub file_path: String,
    pub size: usize,
}

impl AtomDownloadMetadata {
    pub fn update(&mut self, download: &AtomDownload) {
        if let Some(extension) = Path::new(&download.get_file_name()).extension() {
            self.extension = extension.to_string_lossy().to_string();
            self.file_path = Path::new(&download.file_path)
                .join(download.get_file_name())
                .to_string_lossy()
                .to_string();
        }
        self.url = download.get_url();
        self.size = download.get_download_size();
    }
}
