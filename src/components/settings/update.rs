use super::AtomSettings;
use crate::messages::{Message, SettingsMessage};
use iced::Task as Command;

impl AtomSettings {
    pub fn update(&mut self, message: SettingsMessage) -> Command<Message> {
        match message {
            SettingsMessage::ClearCacheClicked(force) => {
                if force {
                    std::fs::remove_dir_all(&self.cache_dir).ok();
                    std::fs::create_dir_all(&self.cache_dir).ok();
                    self.show_confirm_dialog = false;
                } else {
                    self.show_confirm_dialog = true;
                }
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
            SettingsMessage::DownloadDirSelected(Some(folder)) => {
                self.downloads_dir = folder.to_str().unwrap_or("").to_string();
            }
            SettingsMessage::ThemeChanged(theme) => self.theme = theme,
            SettingsMessage::ListLayoutChanged(layout) => self.list_layout = layout.into(),
            SettingsMessage::NewDownloadPositionChanged(pos) => self.new_download_pos = pos,
            SettingsMessage::ScalingChanged(scaling) => self.scaling = scaling,
            SettingsMessage::TextSizeChanged(text_size) => self.font_size = text_size,
            SettingsMessage::ThreadsChanged(threads) => self.threads = threads,
            SettingsMessage::NotificationToggle(checked) => self.show_notifications = checked,
            SettingsMessage::QuitActionToggle(checked) => self.minimize_to_tray = checked,
            SettingsMessage::MaximizedActionToggle(checked) => self.maximized = checked,
            SettingsMessage::AutoStartDownloadToggle(checked) => self.auto_start_download = checked,
            SettingsMessage::ListBackgroundToggle(checked) => self.stretch_list_view = checked,
            SettingsMessage::AlwaysShowPreviewPaneToggle(checked) => {
                self.metadata_always_enabled = checked
            }
            SettingsMessage::ScrollbarsVisible(checked) => self.scrollbars_visible = checked,
            SettingsMessage::ClosePane => {}
            SettingsMessage::HideDialog => {
                self.show_confirm_dialog = false;
                self.reset_settings = false;
            }
            SettingsMessage::OpenConfigDir => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer.exe")
                    .arg(&self.config_dir)
                    .spawn()
                    .ok();
            }
            SettingsMessage::ResetSettings(force) => {
                if !force {
                    self.show_confirm_dialog = true;
                    self.reset_settings = true;
                }
            }
            _ => {}
        }

        Command::none()
    }
}
