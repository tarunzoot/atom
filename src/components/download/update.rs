use super::{AtomDownload, DownloadMessage};
use crate::{
    components::settings::AtomSettings,
    utils::helpers::{open_file, show_notification},
};
use std::{path::PathBuf, time::SystemTime};
use tracing::warn;

impl AtomDownload {
    pub fn update(&mut self, state: DownloadMessage, settings: &AtomSettings) {
        match state {
            DownloadMessage::SetFileSize(size, file_size) => {
                self.size = size;
                self.downloaded = file_size
            }
            DownloadMessage::Error(error) => {
                self.error = error;
                self.downloading = false;
                warn!("{:#?}", self.error);
                if settings.show_notifications {
                    show_notification(
                        "Download Error",
                        &format!("{} cannot be downloaded", self.file_name),
                        6000,
                    );
                }
            }
            DownloadMessage::DownloadDoneJoining => {
                self.joining = true;
                self.downloading = false;
            }
            DownloadMessage::Finished => {
                self.downloading = false;
                self.joining = false;
                self.download_this_session = 0;
                if self.size < 1 {
                    self.size = self.downloaded;
                }
                if settings.show_notifications {
                    show_notification("Download Complete", &self.file_name, 6000);
                }

                let cache_dir = settings.cache_dir.to_string_lossy().to_string();
                (1..=self.threads).for_each(|i| {
                    let file = format!("{}/{}.atom.{}", cache_dir, self.file_name, i);
                    std::fs::remove_file(file).ok();
                });

                if self.auto_open {
                    let path = PathBuf::from(&self.file_path)
                        .join(&self.file_name)
                        .to_string_lossy()
                        .to_string();
                    open_file(&path);
                }
            }
            DownloadMessage::DownloadProgress(downloaded) => {
                if downloaded > self.downloaded {
                    let chunk_len = downloaded - self.downloaded;
                    self.downloaded = downloaded;
                    self.download_this_session += chunk_len;

                    if self.elapsed_time.is_none() {
                        self.elapsed_time = Some(SystemTime::now());
                    }

                    if let Ok(elapsed) =
                        SystemTime::now().duration_since(self.elapsed_time.unwrap())
                    {
                        let elapsed = elapsed.as_secs_f64();
                        let speed =
                            ((self.download_this_session as f64 * 8.0) / elapsed) / 8000000.0;
                        self.transfer_rate = speed;
                    }
                }
            }
            DownloadMessage::JoiningProgress(bytes) => {
                self.joined_bytes += bytes;
                self.joining = true;
                // self.is_downloading = true;
            }
            DownloadMessage::Downloading => {
                self.downloading = true;
                self.error = String::default();
                self.elapsed_time = Some(SystemTime::now());
                self.download_this_session = 0;
            }
            DownloadMessage::Paused => {
                self.downloading = false;
                self.joining = false;
                self.download_this_session = 0;
            }
            DownloadMessage::DownloadSelected => {
                // return Command::perform(async {}, |_| Message::ShowMetadata(index))
            }
            DownloadMessage::MarkDeleted => {
                self.show_delete_confirm_dialog = true;
            }
            DownloadMessage::RemoveDownload(force) => {
                if !force {
                    self.deleted = true;
                    self.downloading = false;
                    self.show_delete_confirm_dialog = false;
                }
            }
            DownloadMessage::HideDialog => self.show_delete_confirm_dialog = false,
            _ => {}
        }
    }
}
