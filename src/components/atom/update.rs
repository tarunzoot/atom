use super::{Atom, View};
use crate::{
    components::download::{AtomDownload, DownloadType},
    components::form::AtomDownloadForm,
    messages::{
        DownloadStateMessage, DownloadsFilterListMessage, Message, SideBarActiveButton,
        SideBarState, SidebarMessage,
    },
    utils::helpers::{get_epoch_ms, save_downloads_toml, save_settings_toml},
};
use iced::{keyboard, window, Command};
use std::path::PathBuf;

impl<'a> Atom<'a> {
    fn update_view(&mut self, view: View) {
        match view {
            View::NewDownloadForm => {
                self.sidebar.active = SideBarActiveButton::AddDownload;
                self.metadata.enabled = false;
            }
            View::Settings => todo!(),
            View::Shortcuts => todo!(),
            View::Downloads => {
                self.sidebar.active = SideBarActiveButton::Overview;
                self.filter_type = DownloadsFilterListMessage::All;
                self.metadata.enabled = false;
            }
            View::DeleteConfirm => todo!(),
            View::Import => todo!(),
        }

        self.view = view;
    }

    pub fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Ignore => {}
            Message::EventsOccurred(event) => {
                if let iced_native::Event::Keyboard(iced_native::keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                }) = event
                {
                    if keyboard::KeyCode::Tab == key_code {
                        if modifiers.shift() {
                            return iced::widget::focus_previous();
                        } else {
                            return iced::widget::focus_next();
                        }
                    }

                    if modifiers.control() || modifiers.command() {
                        let message = match key_code {
                            keyboard::KeyCode::N => {
                                Message::Sidebar(SidebarMessage::NewDownloadForm)
                            }
                            keyboard::KeyCode::Q => Message::AppExit,
                            keyboard::KeyCode::I => Message::Sidebar(SidebarMessage::Import),
                            keyboard::KeyCode::P => Message::Sidebar(SidebarMessage::PauseAll),
                            keyboard::KeyCode::R => Message::Sidebar(SidebarMessage::ResumeAll),
                            keyboard::KeyCode::H => Message::GotoHomePage,
                            keyboard::KeyCode::D => Message::Sidebar(SidebarMessage::DeleteConfirm),
                            keyboard::KeyCode::Comma => Message::Sidebar(SidebarMessage::Settings),
                            keyboard::KeyCode::K => Message::Sidebar(SidebarMessage::Shortcuts),
                            _ => Message::Ignore,
                        };

                        return Command::perform(async {}, |_| message);
                    }
                }

                if let iced_native::Event::Window(iced_native::window::Event::Resized {
                    width,
                    height: _,
                }) = event
                {
                    if width > 1010 {
                        self.scale_factor = 1.20;
                    } else {
                        self.scale_factor = 1.0;
                    }
                }

                if let iced_native::Event::Mouse(iced_native::mouse::Event::ButtonPressed(
                    iced_native::mouse::Button::Left,
                )) = event
                {
                    return window::drag();
                }
                if let iced_native::Event::Window(iced_native::window::Event::CloseRequested) =
                    event
                {
                    if !save_settings_toml(&self.settings) {
                        eprintln!("Error: saving settings failed!");
                    }

                    if !save_downloads_toml(
                        self.downloads.clone().into_values().collect(),
                        &PathBuf::from(&self.settings.config_dir).join("downloads.toml"),
                    ) {
                        eprintln!("Error: saving downloads failed!");
                    }
                    self.should_exit = true;
                }
            }
            Message::AppMaximize => {
                if self.scale_factor > 1.0 {
                    self.scale_factor = 1.0;
                } else {
                    self.scale_factor = 1.20;
                }
                return window::toggle_maximize();
            }
            Message::AppMinimize => {
                return window::minimize(true);
            }
            Message::AppHide => {
                if !self.instance.as_ref().unwrap().is_single() {
                    return Command::perform(async {}, |_| Message::AppExit);
                }
                return window::change_mode(iced_native::window::Mode::Hidden);
            }
            Message::AppShow => {
                return window::change_mode(iced_native::window::Mode::Windowed);
            }
            Message::AppExit => {
                self.should_exit = true;
                if !save_settings_toml(&self.settings) {
                    log::warn!("Error: saving settings failed!");
                }

                if !save_downloads_toml(
                    self.downloads.clone().into_values().collect(),
                    &PathBuf::from(&self.settings.config_dir).join("downloads.toml"),
                ) {
                    log::warn!("Error: saving downloads failed!");
                }
                return window::close();
            }
            Message::SearchDownload(search_text) => {
                self.titlebar.search_text = search_text;
            }
            Message::Import(message) => {
                return self.import.update(message, &self.settings);
            }
            Message::SaveSettings(settings) => {
                if !save_settings_toml(&settings) {
                    log::warn!("Warning: unable to save settings => {settings:#?}");
                }
                self.settings = settings;
                self.update_view(View::Downloads);
            }
            Message::OpenConfigDir => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer.exe")
                    .arg(&self.settings.config_dir)
                    .spawn()
                    .ok();
            }
            Message::PreviewFile(file_path) => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer.exe")
                    .arg(file_path)
                    .spawn()
                    .ok();
                #[cfg(target_os = "macos")]
                std::process::Command::new("open")
                    .arg(file_path)
                    .spawn()
                    .ok();
                #[cfg(target_os = "linux")]
                std::process::Command::new("xdg-open")
                    .arg(file_path)
                    .spawn()
                    .ok();
            }
            Message::DeleteFile(file_path) => {
                std::fs::remove_file(file_path).ok();
            }
            Message::ClosePreview => {
                self.metadata.enabled = false;
            }
            Message::DownloadItemSelected(index) => {
                self.metadata.enabled = true;
                if let Some(download) = self.downloads.get(&index) {
                    self.metadata.update(download);
                }
            }
            Message::FilterList(message) => {
                match message {
                    DownloadsFilterListMessage::All => {
                        self.sidebar.active = SideBarActiveButton::Overview;
                    }
                    DownloadsFilterListMessage::Downloading => {
                        self.sidebar.active = SideBarActiveButton::Downloading;
                    }
                    DownloadsFilterListMessage::Paused => {
                        self.sidebar.active = SideBarActiveButton::Paused;
                    }
                    DownloadsFilterListMessage::Finished => {
                        self.sidebar.active = SideBarActiveButton::Finished;
                    }
                    DownloadsFilterListMessage::Deleted => {
                        self.sidebar.active = SideBarActiveButton::Trash;
                    }
                }
                self.filter_type = message;
                self.view = View::Downloads;
                self.metadata.enabled = false;
            }
            Message::Settings(message) => {
                self.settings.update(message);
            }
            Message::DownloadState(state, index) => {
                if let Some(download) = self.downloads.get_mut(&index) {
                    download.update(state, &self.settings);
                }
                return Command::perform(async {}, |_| Message::SaveDownloads);
            }
            Message::MarkDownloadDeleted(index) => {
                self.downloads.get_mut(&index).unwrap().set_deleted(true);
                if self.downloads.is_empty() {
                    self.update_view(View::Downloads);
                }
                return Command::perform(async {}, |_| Message::SaveDownloads);
            }
            Message::RemoveDownload(index) => {
                self.downloads.remove(&index);
                if self.downloads.is_empty() {
                    self.update_view(View::Downloads);
                }
                return Command::perform(async {}, |_| Message::SaveDownloads);
            }
            Message::NewDownloadReceivedFromBrowser(json) => {
                let download = AtomDownload::new()
                    .headers(json.headers)
                    .url(json.url)
                    .file_name(json.file_name)
                    .file_size(json.size)
                    .file_path(&self.settings.downloads_dir)
                    .download_type(if json.size == 0 {
                        DownloadType::Sequential
                    } else {
                        DownloadType::Threaded
                    });

                match download.build() {
                    Ok(atom_download) => {
                        if self.settings.auto_start_download {
                            return Command::perform(async {}, |_| {
                                Message::AddNewDownload(atom_download)
                            });
                        } else {
                            self.download_form =
                                AtomDownloadForm::new(atom_download, &self.settings);
                            self.update_view(View::NewDownloadForm);
                        }
                    }
                    Err(e) => log::warn!("Error: new download from browser, {:#?}", e),
                }
            }
            Message::AddNewDownload(mut new_download) => {
                new_download.threads = self.settings.threads;

                if let Some(existing_download_id) =
                    self.downloads.iter().find_map(|(&index, download)| {
                        if download.url == new_download.url && !download.is_deleted {
                            Some(index)
                        } else {
                            None
                        }
                    })
                {
                    if let Some(existing_download) = self.downloads.get_mut(&existing_download_id) {
                        let file_path = PathBuf::from(&existing_download.file_path)
                            .join(&existing_download.file_name);
                        if !file_path.exists() {
                            *existing_download = new_download;
                            existing_download.is_downloading = true;
                        }
                    }
                } else {
                    self.downloads.insert(get_epoch_ms(), new_download);
                }

                self.sidebar.active = SideBarActiveButton::Overview;
                self.view = View::Downloads;
                return Command::perform(async {}, |_| Message::SaveDownloads);
            }
            Message::SaveDownloads => {
                if !save_downloads_toml(
                    self.downloads.clone().into_values().collect(),
                    &PathBuf::from(&self.settings.config_dir).join("downloads.toml"),
                ) {
                    log::warn!("Error: saving downloads failed!");
                }
            }
            Message::GotoHomePage => {
                self.view = View::Downloads;
                self.filter_type = DownloadsFilterListMessage::All;
                self.metadata.enabled = false;
                self.sidebar.active = if self.downloads.is_empty() {
                    SideBarActiveButton::AddDownload
                } else {
                    SideBarActiveButton::Overview
                };
            }
            Message::DownloadForm(message) => {
                self.metadata.enabled = false;
                return self.download_form.update(message, &self.settings);
            }
            Message::Sidebar(message) => match message {
                SidebarMessage::NewDownloadForm => {
                    self.update_view(View::NewDownloadForm);
                    self.download_form.reset();
                }
                SidebarMessage::Expand => {
                    self.sidebar.state = SideBarState::Full;
                    self.settings.sidebar_collapsed = false;
                }
                SidebarMessage::Collapse => {
                    self.sidebar.state = SideBarState::Collapsed;
                    self.settings.sidebar_collapsed = true;
                }
                SidebarMessage::Import => {
                    self.sidebar.active = SideBarActiveButton::Import;
                    self.view = View::Import;
                    self.metadata.enabled = false;
                }
                SidebarMessage::ResumeAll => {
                    self.downloads.iter_mut().for_each(|(&_, download)| {
                        if !download.is_downloaded() {
                            download.update(DownloadStateMessage::Downloading, &self.settings)
                        }
                    });
                    self.sidebar.active = SideBarActiveButton::ResumeAll;
                    self.metadata.enabled = false;
                }
                SidebarMessage::DeleteConfirm => {
                    self.view = View::DeleteConfirm;
                    self.sidebar.active = SideBarActiveButton::DeleteAll;
                }
                SidebarMessage::DeleteAll => {
                    self.downloads.clear();
                    self.view = View::Downloads;
                    self.filter_type = DownloadsFilterListMessage::All;
                    self.metadata.enabled = false;
                    self.sidebar.active = SideBarActiveButton::Overview;
                }
                SidebarMessage::PauseAll => {
                    self.downloads.iter_mut().for_each(|(&_, download)| {
                        download.update(DownloadStateMessage::Paused, &self.settings)
                    });
                    self.metadata.enabled = false;
                    self.sidebar.active = SideBarActiveButton::PauseAll;
                }
                SidebarMessage::Settings => {
                    self.view = View::Settings;
                    self.filter_type = DownloadsFilterListMessage::All;
                    self.metadata.enabled = false;
                    self.sidebar.active = SideBarActiveButton::Settings;
                }
                SidebarMessage::Shortcuts => {
                    self.view = View::Shortcuts;
                    self.filter_type = DownloadsFilterListMessage::All;
                    self.metadata.enabled = false;
                    self.sidebar.active = SideBarActiveButton::Shortcuts;
                }
            },
            Message::StartImportDownload(import) => {
                if let Ok(file_contents) = std::fs::read_to_string(&import.import_file) {
                    file_contents
                        .split('\n')
                        .filter(|f| !f.trim().is_empty())
                        .for_each(|link| {
                            let link = link
                                .strip_suffix("\r\n")
                                .or(link.strip_suffix('\r'))
                                .or(link.strip_suffix('\n'))
                                .unwrap_or(link);

                            if let Ok(url) = reqwest::Url::parse(link) {
                                if let Some(file_name) = url.path_segments() {
                                    if let Some(file_name) = file_name.last() {
                                        urlencoding::decode(file_name)
                                            .map(|file_name| {
                                                match AtomDownload::new()
                                                    .url(link)
                                                    .file_path(&import.download_path)
                                                    .file_name(file_name)
                                                    .file_size(0)
                                                    .download_type(if import.is_sequential {
                                                        DownloadType::Sequential
                                                    } else {
                                                        DownloadType::Threaded
                                                    })
                                                    .build()
                                                {
                                                    Ok(atom_download) => {
                                                        let _ = self.update(
                                                            Message::AddNewDownload(atom_download),
                                                        );
                                                    }

                                                    Err(e) => log::warn!("Error: {:#?}", e),
                                                }
                                            })
                                            .ok();
                                    }
                                }
                            }

                            std::thread::sleep(std::time::Duration::from_millis(2));
                        });
                    return Command::perform(async {}, |_| Message::SaveDownloads);
                }
            }
            Message::TrayMessages(tray_message) => {
                let message = match tray_message {
                    crate::messages::TrayMessage::ShowApp => Message::Ignore,
                    crate::messages::TrayMessage::AddNewDownload => {
                        Message::Sidebar(SidebarMessage::NewDownloadForm)
                    }
                    crate::messages::TrayMessage::Settings => {
                        Message::Sidebar(SidebarMessage::Settings)
                    }
                    crate::messages::TrayMessage::Import => {
                        Message::Sidebar(SidebarMessage::Import)
                    }
                    crate::messages::TrayMessage::Exit => {
                        return Command::perform(async {}, |_| Message::AppExit)
                    }
                };
                return Command::batch(vec![
                    Command::perform(async {}, |_| Message::AppShow),
                    Command::perform(async {}, |_| message),
                ]);
            }
            Message::TrayEvent(id) => {
                if let Some(message) = self.tray_event.get(&id) {
                    let message = message.to_owned();
                    return Command::perform(async {}, |_| message);
                }
                log::warn!("Warning: unknown tray event id => {id}");
            } // Message::Tick => {
              //     // self.preview.update_width();
              // }
        }
        Command::none()
    }
}
