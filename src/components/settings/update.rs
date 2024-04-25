use super::AtomSettings;
use crate::messages::{Message, SettingsMessage};
use iced::Command;

impl AtomSettings {
    pub fn update(&mut self, message: SettingsMessage) -> Command<Message> {
        match message {
            SettingsMessage::ClearCacheClicked => {
                std::fs::remove_dir_all(&self.cache_dir).ok();
                std::fs::create_dir_all(&self.cache_dir).ok();
            }
            SettingsMessage::BrowseDownloadsDirClicked => {
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .pick_folder()
                            .await
                            .map(|file| file.path().to_owned())
                    },
                    |path| Message::Settings(SettingsMessage::DownloadDirSelected(path)),
                );
            }
            SettingsMessage::DownloadDirSelected(folder) => {
                if let Some(folder) = folder {
                    self.downloads_dir = folder.to_str().unwrap_or("").to_string();
                }
            }
            SettingsMessage::ThemeChanged(theme) => self.theme = theme,
            SettingsMessage::ListLayoutChanged(layout) => self.list_layout = layout.into(),
            SettingsMessage::NewDownloadPositionChanged(pos) => self.new_download_pos = pos,
            SettingsMessage::ScalingChanged(scaling) => self.scaling = scaling,
            SettingsMessage::ThreadsChanged(threads) => self.threads = threads,
            SettingsMessage::NotificationToggle(checked) => self.show_notifications = checked,
            SettingsMessage::QuitActionToggle(checked) => self.quit_action_closes_app = checked,
            SettingsMessage::MaximizedActionToggle(checked) => self.maximized = checked,
            SettingsMessage::AutoStartDownloadToggle(checked) => self.auto_start_download = checked,
            SettingsMessage::ListBackgroundToggle(checked) => self.stretch_list_view = checked,
            SettingsMessage::NewDownloadNotificationToggle(checked) => {
                self.new_download_notification = checked
            }
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
