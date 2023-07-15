use super::AtomSettings;
use crate::messages::SettingsMessage;
use rfd::FileDialog;

impl AtomSettings {
    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::ClearCacheClicked => {
                std::fs::remove_dir_all(&self.cache_dir).ok();
                std::fs::create_dir_all(&self.cache_dir).ok();
            }
            SettingsMessage::BrowseDownloadsDirClicked => {
                if let Some(folder) = FileDialog::new().pick_folder() {
                    self.downloads_dir = folder.to_str().unwrap_or("").to_string();
                }
            }
            // SettingsMessage::BrowseCacheDirClicked => {
            //     if let Ok(Response::Okay(folder_path)) = nfd2::open_pick_folder(None) {
            //         self.cache_dir = folder_path.to_str().unwrap_or("").to_string();
            //     }
            // }
            SettingsMessage::ThreadsChanged(threads) => {
                self.threads = threads;
            }
            SettingsMessage::NotificationToggle(checked) => self.show_notifications = checked,
            SettingsMessage::QuitActionToggle(checked) => self.quit_action_closes_app = checked,
            SettingsMessage::AutoStartDownloadToggle(checked) => self.auto_start_download = checked,
        }
    }
}
