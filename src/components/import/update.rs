use super::AtomImport;
use crate::{
    components::settings::AtomSettings,
    // styles::style::{AtomInputDisabled, AtomToggler},
    messages::{ImportMessage, Message},
};
use iced::Command;
use rfd::FileDialog;

impl AtomImport {
    pub fn update(&mut self, message: ImportMessage, settings: &AtomSettings) -> Command<Message> {
        match message {
            ImportMessage::DownloadTypeToggled(checked) => self.is_sequential = checked,
            ImportMessage::ImportFileClicked => {
                if let Some(file) = FileDialog::new()
                    .add_filter("text", &["txt", "*.*"])
                    .set_directory("/")
                    .pick_file()
                {
                    self.import_file = file.to_str().unwrap_or("").to_string();
                }
            }
            ImportMessage::DownloadFolderSelectClicked => {
                let downloads_dir = settings.downloads_dir.clone();
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_directory(downloads_dir)
                            .pick_folder()
                            .await
                            .map(|file| file.path().to_owned())
                    },
                    |path| Message::Import(ImportMessage::DownloadFolder(path)),
                );
            }
            ImportMessage::DownloadFolder(Some(folder)) => {
                self.download_path = folder.to_str().unwrap_or_default().to_owned();
            }
            _ => {}
        }

        Command::none()
    }
}
