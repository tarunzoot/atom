use super::AtomDownloadForm;
use crate::{
    components::settings::AtomSettings,
    messages::{DownloadFormMessage, Message},
};
use iced::Task as Command;

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
                                    if !file_name.is_empty() {
                                        self.file_name =
                                            format!("{}{}", settings.downloads_dir, file_name)
                                    } else {
                                        self.file_name = String::default()
                                    }
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
            DownloadFormMessage::EditHeaderValue(header_name, header_value) => {
                if let Some(hv) = self.headers.get_mut(&header_name) {
                    *hv = header_value;
                }
            }
            DownloadFormMessage::DeleteHeader(header_name) => {
                self.headers.remove(&header_name);
            }
            DownloadFormMessage::AutoOpen(open) => self.auto_open = open,
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
            DownloadFormMessage::ImportHeaders => {
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .pick_file()
                            .await
                            .map(|file| file.path().to_owned())
                    },
                    |path| Message::DownloadForm(DownloadFormMessage::HeaderFilePath(path)),
                );
            }
            DownloadFormMessage::FileSavePathChanged(save_as_path) => {
                if let Some(path) = save_as_path {
                    self.file_name = path.to_str().unwrap_or_default().to_string()
                }
            }
            DownloadFormMessage::HeaderFilePath(file_path) => {
                if let Some(file_path) = file_path {
                    if let Ok(content) = std::fs::read_to_string(file_path) {
                        let headers = content.split('\n').filter(|f| !f.is_empty());
                        for header in headers {
                            let mut header_splitted = header.trim().split(':');
                            let header_name = header_splitted.next().unwrap_or("").trim();
                            let header_value = header_splitted.next().unwrap_or("").trim();

                            if !header_name.is_empty() {
                                self.headers
                                    .insert(header_name.to_owned(), header_value.to_owned());
                            }
                        }
                    }
                }
            }
            DownloadFormMessage::ClosePane => {}
            DownloadFormMessage::AddNewDownload => {}
        }
        Command::none()
    }
}
