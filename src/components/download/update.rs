use super::{AtomDownload, DownloadStateMessage};
use crate::components::settings::AtomSettings;
use std::time::SystemTime;

impl AtomDownload {
    pub fn update(&mut self, state: DownloadStateMessage, settings: &AtomSettings) {
        match state {
            DownloadStateMessage::SetFileSize(size, file_size) => {
                self.size = size;
                self.downloaded = file_size
            }
            DownloadStateMessage::Error(error) => {
                self.error = error;
                if settings.show_notifications
                    && notify_rust::Notification::new()
                        .summary("A.T.O.M")
                        .subtitle("Download Error")
                        .auto_icon()
                        .body(&format!("{} cannot be downloaded", self.file_name))
                        .icon("atom")
                        .timeout(notify_rust::Timeout::Milliseconds(6000))
                        .show()
                        .is_err()
                {
                    log::debug!("[ATOM] error notification failed!");
                }
            }
            DownloadStateMessage::DownloadDoneJoining => {
                self.is_joining = true;
                self.is_downloading = false;
            }
            DownloadStateMessage::Finished => {
                self.is_downloading = false;
                self.is_joining = false;
                self.download_this_session = 0;
                if settings.show_notifications
                    && notify_rust::Notification::new()
                        .summary("A.T.O.M")
                        .subtitle("Download Complete")
                        .auto_icon()
                        .body(&self.file_name)
                        .icon("atom")
                        .timeout(notify_rust::Timeout::Milliseconds(6000))
                        .show()
                        .is_err()
                {
                    log::debug!("[ATOM] : download notification failed!");
                }

                let cache_dir = settings.cache_dir.to_string_lossy().to_string();
                (1..=self.threads).for_each(|i| {
                    let file = format!("{}/{}.atom.{}", cache_dir, self.file_name, i);
                    std::fs::remove_file(file).ok();
                });
            }
            DownloadStateMessage::DownloadProgress(downloaded) => {
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
            DownloadStateMessage::JoiningProgress(bytes) => {
                self.joined_bytes += bytes;
                self.is_joining = true;
                // self.is_downloading = true;
            }
            DownloadStateMessage::Downloading => {
                self.is_downloading = true;
                self.error = String::default();
                self.elapsed_time = Some(SystemTime::now());
            }
            DownloadStateMessage::Paused => {
                self.is_downloading = false;
                self.is_joining = false;
                self.download_this_session = 0;
            }
        }
    }
}
