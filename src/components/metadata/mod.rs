mod subscription;
mod view;
use crate::{components::download::AtomDownload, messages::MetadataMessage};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Default)]
pub struct AtomDownloadMetadata {
    pub enabled: bool,
    pub url: String,
    pub extension: String,
    pub file_path: String,
    pub size: usize,
    pub checksums: HashMap<String, String>,
    is_calculating_checksum: bool,
}

impl AtomDownloadMetadata {
    pub fn update(&mut self, message: MetadataMessage) {
        match message {
            MetadataMessage::PreviewFile => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer.exe")
                    .arg(&self.file_path)
                    .spawn()
                    .ok();
                #[cfg(target_os = "macos")]
                std::process::Command::new("open")
                    .arg(&self.file_path)
                    .spawn()
                    .ok();
                #[cfg(target_os = "linux")]
                std::process::Command::new("xdg-open")
                    .arg(&self.file_path)
                    .spawn()
                    .ok();
            }
            MetadataMessage::DeleteFile => {
                std::fs::remove_file(&self.file_path).ok();
            }
            MetadataMessage::Checksum(checksum, url) => {
                self.is_calculating_checksum = false;
                if let Some(key) = self.checksums.get_mut(&url) {
                    *key = checksum;
                } else {
                    self.checksums.insert(url, checksum);
                }
            }
            MetadataMessage::CalculateChecksum => self.is_calculating_checksum = true,
            _ => {}
        };
    }

    pub fn update_info(&mut self, download: &AtomDownload) {
        if let Some(extension) = Path::new(&download.get_file_name()).extension() {
            self.extension = extension.to_string_lossy().to_string();
            self.file_path = Path::new(&download.file_path)
                .join(download.get_file_name())
                .to_string_lossy()
                .to_string();
        }
        self.url = download.get_url();
        self.size = download.get_download_size();
        self.is_calculating_checksum = false;
    }
}
