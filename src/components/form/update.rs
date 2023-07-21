use super::AtomDownloadForm;
use crate::{
    components::settings::AtomSettings,
    messages::{DownloadFormMessage, Message},
};
use iced::Command;

impl AtomDownloadForm {
    pub fn update(
        &mut self,
        message: DownloadFormMessage,
        settings: &AtomSettings,
    ) -> Command<Message> {
        match message {
            DownloadFormMessage::UrlChange(url) => {
                self.url = url;
                if let Ok(url) = reqwest::Url::parse(&self.url) {
                    if let Some(file_name) = url.path_segments() {
                        if let Some(file_name) = file_name.last() {
                            urlencoding::decode(file_name)
                                .map(|file_name| {
                                    self.file_name =
                                        format!("{}{}", settings.downloads_dir, file_name)
                                })
                                .ok();
                        }
                    }
                    self.is_valid_url = true;
                } else {
                    self.is_valid_url = false;
                }
            }
            DownloadFormMessage::DownloadSequentially(checked) => self.sequential = checked,
            DownloadFormMessage::AddHeader => {
                if !self.header_name.is_empty() {
                    self.headers
                        .insert(self.header_name.clone(), self.header_value.clone());
                }
            }
            DownloadFormMessage::AddHeaderName(header_name) => self.header_name = header_name,
            DownloadFormMessage::AddHeaderValue(header_value) => self.header_value = header_value,
            DownloadFormMessage::EditHeader(header_name) => {
                if self.headers.contains_key(&header_name) {
                    self.header_value = self.headers.get(&header_name).unwrap().to_string();
                    self.header_name = header_name;
                }
            }
            DownloadFormMessage::DeleteHeader(header_name) => {
                self.headers.remove(&header_name);
            }
            DownloadFormMessage::AutoReferer(checked) => {
                if !self.url.is_empty() {
                    self.auto_referer = checked;
                    if checked {
                        if let Ok(parsed_url) = reqwest::Url::parse(&self.url) {
                            if parsed_url.host().is_some() {
                                self.headers.insert(
                                    "referer".to_string(),
                                    format!(
                                        "{}://{}/",
                                        parsed_url.scheme(),
                                        parsed_url.host().unwrap()
                                    ),
                                );
                            }
                        }
                    } else {
                        self.headers.remove("referer");
                    }
                }
            }
            DownloadFormMessage::BrowseSaveAsFolder => {
                let downloads_dir = settings.downloads_dir.clone();
                let file_name = self.file_name.clone();
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_directory(downloads_dir)
                            .set_file_name(
                                std::path::PathBuf::from(file_name)
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_str()
                                    .unwrap_or_default(),
                            )
                            .save_file()
                            .await
                            .map(|file| file.path().to_owned())
                    },
                    |path| Message::DownloadForm(DownloadFormMessage::FileSavePathChanged(path)),
                );
            }
            DownloadFormMessage::FileSavePathChanged(save_as_path) => {
                if let Some(path) = save_as_path {
                    self.file_name = path.to_str().unwrap_or_default().to_string()
                }
            }
            DownloadFormMessage::ClosePane => {}
            DownloadFormMessage::AddNewDownload => {}
        }
        Command::none()
    }
}
