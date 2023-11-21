use super::{Atom, View};
use crate::{
    components::download::{AtomDownload, DownloadType},
    components::form::AtomDownloadForm,
    messages::{
        DownloadMessage, DownloadsListFilterMessage, Message, SideBarActiveButton, SideBarState,
        SidebarMessage, TitleBarMessage,
    },
    utils::helpers::{get_epoch_ms, save_downloads_toml, save_settings_toml},
};
use iced::{keyboard, window, Command, Event};
use std::path::PathBuf;
use tracing::{error, warn};

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
                self.filter_type = DownloadsListFilterMessage::All;
                self.metadata.enabled = false;
            }
            View::Import => todo!(),
        }

        self.view = view;
    }

    pub fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Ignore => {}
            Message::ToggleListViewLayout => match self.settings.list_layout {
                crate::components::settings::ListLayout::ListExtended => {
                    self.settings.list_layout = crate::components::settings::ListLayout::List
                }
                crate::components::settings::ListLayout::List => {
                    self.settings.list_layout =
                        crate::components::settings::ListLayout::ListExtended
                }
            },
            Message::EventsOccurred(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed {
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
                            keyboard::KeyCode::Q => Message::TitleBar(TitleBarMessage::AppExit),
                            keyboard::KeyCode::I => Message::Sidebar(SidebarMessage::Import),
                            keyboard::KeyCode::P => Message::Sidebar(SidebarMessage::PauseAll),
                            keyboard::KeyCode::R => Message::Sidebar(SidebarMessage::ResumeAll),
                            keyboard::KeyCode::H => Message::GotoHomePage,
                            keyboard::KeyCode::D => Message::Sidebar(SidebarMessage::DeleteConfirm),
                            keyboard::KeyCode::Comma => Message::Sidebar(SidebarMessage::Settings),
                            keyboard::KeyCode::K => Message::Sidebar(SidebarMessage::Shortcuts),
                            keyboard::KeyCode::F => {
                                return iced::widget::text_input::focus(
                                    iced::widget::text_input::Id::new("search"),
                                )
                            }
                            _ => Message::Ignore,
                        };

                        return Command::perform(async {}, |_| message);
                    }
                }

                if let Event::Window(window::Event::Resized { width, height: _ }) = event {
                    if width > 1200 {
                        self.scale_factor = 1.20;
                    } else {
                        self.scale_factor = 1.0;
                    }
                }

                if let Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) =
                    event
                {
                    return window::drag();
                }
                if let Event::Window(window::Event::CloseRequested) = event {
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
            Message::TitleBar(message) => match message {
                TitleBarMessage::AppMaximize => {
                    return window::toggle_maximize();
                }
                TitleBarMessage::AppMinimize => {
                    return window::minimize(true);
                }
                TitleBarMessage::AppHide => {
                    if !self.instance.as_ref().unwrap().is_single() {
                        return Command::perform(async {}, |_| {
                            Message::TitleBar(TitleBarMessage::AppExit)
                        });
                    }
                    return window::change_mode(window::Mode::Hidden);
                }
                TitleBarMessage::AppShow => {
                    return window::change_mode(window::Mode::Windowed);
                }
                TitleBarMessage::AppExit => {
                    self.should_exit = true;
                    if !save_settings_toml(&self.settings) {
                        warn!("Error: saving settings failed!");
                    }

                    if !save_downloads_toml(
                        self.downloads.clone().into_values().collect(),
                        &PathBuf::from(&self.settings.config_dir).join("downloads.toml"),
                    ) {
                        warn!("Error: saving downloads failed!");
                    }
                    return window::close();
                }
                TitleBarMessage::SearchDownload(search_text) => {
                    self.titlebar.search_text = search_text
                }
            },
            Message::Import(message) => match message {
                crate::messages::ImportMessage::ClosePane => {
                    return Command::perform(async {}, |_| Message::GotoHomePage)
                }
                crate::messages::ImportMessage::StartImportDownload => {
                    if let Ok(file_contents) = std::fs::read_to_string(&self.import.import_file) {
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
                                                        .file_path(&self.import.download_path)
                                                        .file_name(file_name)
                                                        .file_size(0)
                                                        .download_type(
                                                            if self.import.is_sequential {
                                                                DownloadType::Sequential
                                                            } else {
                                                                DownloadType::Threaded
                                                            },
                                                        )
                                                        .build()
                                                    {
                                                        Ok(atom_download) => {
                                                            let _ = self.update(
                                                                Message::AddNewDownload(
                                                                    atom_download,
                                                                ),
                                                            );
                                                        }

                                                        Err(e) => warn!("Error: {:#?}", e),
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
                _ => return self.import.update(message, &self.settings),
            },
            Message::Metadata(message) => match message {
                crate::messages::MetadataMessage::ClosePane => self.metadata.enabled = false,
                _ => self.metadata.update(message),
            },
            Message::ShowMetadata(index) => {
                self.metadata.enabled = true;
                if let Some(download) = self.downloads.get(&index) {
                    self.metadata.update_info(download);
                }
            }
            Message::DownloadsListFilter(message) => {
                match message {
                    DownloadsListFilterMessage::All => {
                        self.sidebar.active = SideBarActiveButton::Overview;
                    }
                    DownloadsListFilterMessage::Downloading => {
                        self.sidebar.active = SideBarActiveButton::Downloading;
                    }
                    DownloadsListFilterMessage::Paused => {
                        self.sidebar.active = SideBarActiveButton::Paused;
                    }
                    DownloadsListFilterMessage::Finished => {
                        self.sidebar.active = SideBarActiveButton::Finished;
                    }
                    DownloadsListFilterMessage::Deleted => {
                        self.sidebar.active = SideBarActiveButton::Trash;
                    }
                }
                self.filter_type = message;
                self.view = View::Downloads;
                self.metadata.enabled = false;
            }
            Message::Settings(message) => match message {
                crate::messages::SettingsMessage::ClosePane => {
                    self.settings = self.default_settings.clone();
                    let _ = self.update(Message::GotoHomePage);
                }
                crate::messages::SettingsMessage::SaveSettings => {
                    if !save_settings_toml(&self.settings) {
                        warn!("Warning: unable to save settings => {:#?}", self.settings);
                    }
                    self.default_settings = self.settings.clone();
                    self.theme = self.settings.theme.clone().into();
                    self.update_view(View::Downloads);
                }
                _ => return self.settings.update(message),
            },
            Message::Download(state, index) => match state {
                DownloadMessage::DownloadSelected => {
                    return Command::perform(async {}, move |_| Message::ShowMetadata(index));
                }
                DownloadMessage::RemoveDownload(force) => {
                    if force {
                        self.downloads.remove(&index);
                        if self.downloads.is_empty() {
                            self.update_view(View::Downloads);
                        }
                        return Command::perform(async {}, |_| Message::SaveDownloads);
                    } else if let Some(download) = self.downloads.get_mut(&index) {
                        download.update(state, &self.settings);
                    }
                }
                DownloadMessage::Finished => {
                    if let Some(download) = self.downloads.get_mut(&index) {
                        download.update(state, &self.settings);
                    }
                    return Command::perform(async {}, |_| Message::SaveDownloads);
                }
                _ => {
                    if let Some(download) = self.downloads.get_mut(&index) {
                        download.update(state, &self.settings);
                    }
                }
            },
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
                            return Command::batch(vec![
                                Command::perform(async {}, |_| {
                                    Message::TitleBar(TitleBarMessage::AppShow)
                                }),
                                window::gain_focus(),
                            ]);
                        }
                    }
                    Err(e) => warn!("Error: new download from browser, {:#?}", e),
                }
            }
            Message::AddNewDownload(mut new_download) => {
                new_download.threads = self.settings.threads;

                if let Some(existing_download_id) =
                    self.downloads.iter().find_map(|(&index, download)| {
                        if (download.url == new_download.url
                            || (download.file_name == new_download.file_name
                                && download.file_path == new_download.file_path))
                            && !download.is_deleted
                        {
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

                let _ = self.update(Message::GotoHomePage);
                return Command::perform(async {}, |_| Message::SaveDownloads);
            }
            Message::SaveDownloads => {
                if !save_downloads_toml(
                    self.downloads.clone().into_values().collect(),
                    &PathBuf::from(&self.settings.config_dir).join("downloads.toml"),
                ) {
                    warn!("Error: saving downloads failed!");
                }
            }
            Message::GotoHomePage => {
                self.filters.show_confirmation_dialog = false;
                self.view = View::Downloads;
                self.filter_type = DownloadsListFilterMessage::All;
                self.metadata.enabled = false;
                self.sidebar.active = if self.downloads.is_empty() {
                    SideBarActiveButton::AddDownload
                } else {
                    SideBarActiveButton::Overview
                };
            }
            Message::DownloadForm(message) => {
                self.metadata.enabled = false;
                match message {
                    crate::messages::DownloadFormMessage::AddNewDownload => {
                        if let Ok(download) = self.download_form.make_download() {
                            return Command::perform(async {}, |_| {
                                Message::AddNewDownload(download)
                            });
                        }
                    }
                    crate::messages::DownloadFormMessage::ClosePane => {
                        return Command::perform(async {}, |_| Message::GotoHomePage)
                    }
                    _ => return self.download_form.update(message, &self.settings),
                }
                // return self.download_form.update(message, &self.settings);
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
                        if !download.is_downloaded() && !download.is_downloading {
                            download.update(DownloadMessage::Downloading, &self.settings);
                        }
                    });
                    self.sidebar.active = SideBarActiveButton::Overview;
                    self.metadata.enabled = false;
                }
                SidebarMessage::DeleteConfirm => {
                    self.filters.show_confirmation_dialog = true;
                }
                SidebarMessage::DeleteAll => {
                    match self.sidebar.active {
                        SideBarActiveButton::Overview => self.downloads.clear(),
                        SideBarActiveButton::Downloading => {
                            self.downloads.retain(|_, download| {
                                !(download.downloaded < download.size && download.is_downloading)
                                    || download.is_deleted
                            });
                        }
                        SideBarActiveButton::Paused => {
                            self.downloads.retain(|_, download| {
                                download.is_downloading
                                    || download.size <= download.downloaded
                                    || download.is_deleted
                            });
                        }
                        SideBarActiveButton::Finished => {
                            self.downloads.retain(|_, download| {
                                download.downloaded < download.size || download.is_deleted
                            });
                        }
                        SideBarActiveButton::Trash => {
                            self.downloads.retain(|_, download| !download.is_deleted);
                        }
                        _ => {}
                    }

                    if self.downloads.is_empty() {
                        self.filter_type = DownloadsListFilterMessage::All;
                        self.view = View::Downloads;
                        self.sidebar.active = SideBarActiveButton::Overview;
                        self.metadata.enabled = false;
                    }

                    self.filters.show_confirmation_dialog = false;
                }
                SidebarMessage::PauseAll => {
                    self.downloads.iter_mut().for_each(|(&_, download)| {
                        download.update(DownloadMessage::Paused, &self.settings);
                    });
                    self.metadata.enabled = false;
                    self.sidebar.active = SideBarActiveButton::Overview;
                }
                SidebarMessage::Settings => {
                    self.view = View::Settings;
                    self.filter_type = DownloadsListFilterMessage::All;
                    self.metadata.enabled = false;
                    self.sidebar.active = SideBarActiveButton::Settings;
                }
                SidebarMessage::Shortcuts => {
                    self.view = View::Shortcuts;
                    self.filter_type = DownloadsListFilterMessage::All;
                    self.metadata.enabled = false;
                    self.sidebar.active = SideBarActiveButton::Shortcuts;
                }
                SidebarMessage::GotoHomePage => {
                    let _ = self.update(Message::GotoHomePage);
                }
            },
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
                        return Command::perform(async {}, |_| {
                            Message::TitleBar(TitleBarMessage::AppExit)
                        })
                    }
                };
                return Command::batch(vec![
                    Command::perform(async {}, |_| Message::TitleBar(TitleBarMessage::AppShow)),
                    Command::perform(async {}, |_| message),
                ]);
            }
            Message::TrayEvent(id) => {
                if let Some(message) = self.tray_event.get(&id) {
                    let message = message.to_owned();
                    return Command::perform(async {}, |_| message);
                }
                warn!("Warning: unknown tray event id => {id:#?}");
            }
            Message::FontLoaded(result) => {
                if result.is_err() {
                    error!("{result:#?}");
                }
            }
            Message::LoadingComplete => {}
        }
        Command::none()
    }
}
