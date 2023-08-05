use super::AtomSettings;
use crate::messages::{Message, SettingsMessage};
use iced::Command;
use rfd::FileDialog;

impl AtomSettings {
    pub fn update(&mut self, message: SettingsMessage) -> Command<Message> {
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
            SettingsMessage::ThreadsChanged(threads) => {
                self.threads = threads;
            }
            SettingsMessage::NotificationToggle(checked) => self.show_notifications = checked,
            SettingsMessage::QuitActionToggle(checked) => self.quit_action_closes_app = checked,
            SettingsMessage::AutoStartDownloadToggle(checked) => self.auto_start_download = checked,
            SettingsMessage::ClosePane => {}
            SettingsMessage::OpenConfigDir => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer.exe")
                    .arg(&self.config_dir)
                    .spawn()
                    .ok();
            }
            SettingsMessage::SaveSettings => {}
        }

        Command::none()
    }
}
