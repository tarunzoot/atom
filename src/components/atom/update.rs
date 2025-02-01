use super::{Atom, View};
use crate::{
    components::{
        download::AtomDownload,
        form::AtomDownloadForm,
        settings::{AtomSettings, ListLayout},
    },
    messages::{
        DownloadMessage, DownloadsListFilterMessage, Message, SideBarActiveButton, SideBarState,
        SidebarMessage, TitleBarMessage,
    },
    utils::helpers::{
        get_current_time_in_millis, save_downloads_toml, save_settings_toml, show_notification,
        ATOM_ICON, ATOM_SOCKET_ADDRESS,
    },
};
use iced::{
    keyboard::{self, key::Named},
    window::{self, settings::PlatformSpecific, Settings},
    Event, Size, Task as Command,
};
use std::path::PathBuf;
use tracing::{debug, error, warn};

impl Atom<'_> {
    fn update_view(&mut self, view: View) {
        match view {
            View::NewDownloadForm => {
                self.sidebar.active = SideBarActiveButton::AddDownload;
            }
            View::Settings => {
                self.sidebar.active = SideBarActiveButton::Settings;
            }
            View::Shortcuts => {
                self.sidebar.active = SideBarActiveButton::Shortcuts;
            }
            View::Downloads => {
                self.sidebar.active = SideBarActiveButton::Overview;
                self.filter_type = DownloadsListFilterMessage::All;
            }
            View::Import => {
                self.sidebar.active = SideBarActiveButton::Import;
            }
        }

        self.metadata.enabled = false;
        self.status_bar_message = "Ready".to_string();
        self.view = view;
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Ignore => {}
            Message::MainWindow(id) => {
                self.windows
                    .insert(id, ("main", AtomDownloadForm::default()));
            }
            Message::WindowOpened(id, download) => {
                let window = AtomDownloadForm::new(download.unwrap(), &self.settings);
                self.windows.insert(id, ("", window));
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    return iced::exit();
                }

                return window::close(id);
            }
            Message::StatusBar(message) => self.status_bar_message = message,
            Message::EventsOccurred((ref event, window_id)) => {
                if let Event::Keyboard(keyboard::Event::KeyReleased {
                    modifiers: _, key, ..
                }) = event
                {
                    if keyboard::Key::Named(Named::Alt) == *key {
                        self.alt_pressed = false;
                        self.status_bar_message = "Ready".to_string();
                    }
                }

                if let Event::Keyboard(keyboard::Event::KeyPressed { modifiers, key, .. }) = event {
                    if keyboard::Key::Named(Named::Alt) == *key {
                        self.alt_pressed = true;
                        self.status_bar_message = "Alt + Drag window to move".to_string();
                    }

                    if keyboard::Key::Named(Named::Tab) == *key {
                        if modifiers.shift() {
                            return iced::widget::focus_previous();
                        }
                        return iced::widget::focus_next();
                    }

                    if modifiers.control() || modifiers.command() {
                        let message = match key.as_ref() {
                            keyboard::Key::Character("n") => {
                                Message::Sidebar(SidebarMessage::NewDownloadForm)
                            }
                            keyboard::Key::Character("q") => {
                                Message::TitleBar(TitleBarMessage::AppExit)
                            }
                            keyboard::Key::Character("i") => {
                                Message::Sidebar(SidebarMessage::Import)
                            }
                            keyboard::Key::Character("p") => {
                                Message::Sidebar(SidebarMessage::PauseAll)
                            }
                            keyboard::Key::Character("r") => {
                                Message::Sidebar(SidebarMessage::ResumeAll)
                            }
                            keyboard::Key::Character("h") => Message::GotoHomePage,
                            keyboard::Key::Character("d") => {
                                Message::Sidebar(SidebarMessage::DeleteConfirm)
                            }
                            keyboard::Key::Character(",") => {
                                Message::Sidebar(SidebarMessage::Settings)
                            }
                            keyboard::Key::Character("k") => {
                                Message::Sidebar(SidebarMessage::Shortcuts)
                            }
                            keyboard::Key::Character("t") => {
                                self.phantom_settings.theme = match self.theme {
                                    crate::style::AtomTheme::Default => {
                                        crate::style::AtomTheme::Tangerine
                                    }
                                    crate::style::AtomTheme::Tangerine => {
                                        crate::style::AtomTheme::Light
                                    }
                                    crate::style::AtomTheme::Light => crate::style::AtomTheme::Hari,
                                    crate::style::AtomTheme::Hari => {
                                        crate::style::AtomTheme::Default
                                    }
                                }
                                .into();

                                return Command::done(Message::Settings(
                                    crate::messages::SettingsMessage::SaveSettings(false),
                                ));
                            }
                            keyboard::Key::Character("l") => {
                                if matches!(self.settings.list_layout, ListLayout::ListExtended) {
                                    self.phantom_settings.list_layout = ListLayout::List;
                                } else {
                                    self.phantom_settings.list_layout = ListLayout::ListExtended;
                                }
                                return Command::done(Message::Settings(
                                    crate::messages::SettingsMessage::SaveSettings(true),
                                ));
                            }
                            keyboard::Key::Character("f") => {
                                return iced::widget::text_input::focus(
                                    iced::widget::text_input::Id::new("search"),
                                )
                            }
                            keyboard::Key::Character("g") => {
                                self.titlebar.search_text.clear();
                                return Command::done(Message::Ignore);
                            }
                            _ => Message::Ignore,
                        };

                        return Command::done(message);
                    }
                }

                if let Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) =
                    event
                {
                    if let Some(window) = self.windows.get(&window_id) {
                        if window.0 == "main" {
                            if self.alt_pressed || self.mouse_on_titlebar {
                                return window::drag(window_id);
                            }
                        } else {
                            return window::drag(window_id);
                        }
                    }
                }

                if let Event::Window(window::Event::CloseRequested) = event {
                    return Command::done(Message::TitleBar(TitleBarMessage::AppExit));
                }
            }
            Message::WindowResized((window_id, size)) => {
                if let Some(window) = self.windows.get(&window_id) {
                    if window.0 == "main" {
                        self.window_dimensions = (size.width as u32, size.height as u32);
                    }
                }
            }
            Message::TitleBar(message) => match message {
                TitleBarMessage::MouseOnTitlebar(on_titlebar) => {
                    self.mouse_on_titlebar = on_titlebar
                }
                TitleBarMessage::AppMaximize => {
                    return window::get_oldest().and_then(window::toggle_maximize);
                }
                TitleBarMessage::AppMinimize => {
                    return window::get_oldest().and_then(|id| window::minimize(id, true));
                }
                TitleBarMessage::AppHide => {
                    if !self.instance.as_ref().unwrap().is_single() {
                        return Command::done(Message::TitleBar(TitleBarMessage::AppExit));
                    }
                    return window::get_oldest()
                        .and_then(|id| window::change_mode(id, window::Mode::Hidden));
                }
                TitleBarMessage::AppShow => {
                    return window::get_oldest()
                        .and_then(|id| window::change_mode(id, window::Mode::Windowed));
                }
                TitleBarMessage::AppExit => {
                    if !save_settings_toml(&self.settings) {
                        warn!("Error: saving settings failed!");
                    }

                    if !save_downloads_toml(
                        self.downloads.clone().into_values().collect(),
                        &PathBuf::from(&self.settings.config_dir).join("downloads.toml"),
                    ) {
                        warn!("Error: saving downloads failed!");
                    }

                    self.should_exit = true;
                    std::net::TcpStream::connect(ATOM_SOCKET_ADDRESS).map_or_else(
                        |e| {
                            warn!("Error: TcpStream::connect(failed), error({e:#?})");
                        },
                        |_| {
                            debug!("Info: TcpStream::connect(success)");
                        },
                    );

                    // return window::get_oldest().and_then(window::close);
                    return iced::exit();
                }
                TitleBarMessage::SearchDownload(search_text) => {
                    self.titlebar.search_text = search_text.to_ascii_lowercase();
                }
            },
            Message::Import(message) => match message {
                crate::messages::ImportMessage::ClosePane => {
                    return Command::done(Message::GotoHomePage)
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
                                                        .download_type(self.import.is_sequential)
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
                        return Command::done(Message::SaveDownloads);
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
                    self.phantom_settings = self.settings.clone();
                    let _ = self.update(Message::GotoHomePage);
                }
                crate::messages::SettingsMessage::ResetSettings(force) => {
                    if force {
                        let settings = AtomSettings::default();
                        self.settings = settings.clone();
                        self.phantom_settings = settings;
                    } else {
                        return self.phantom_settings.update(message);
                    }
                }
                crate::messages::SettingsMessage::SaveSettings(update_view) => {
                    self.settings = self.phantom_settings.clone();
                    if !save_settings_toml(&self.settings) {
                        warn!("Warning: unable to save settings => {:#?}", self.settings);
                    }

                    self.theme = self.settings.theme.clone().into();
                    if update_view {
                        self.update_view(View::Downloads);
                    }
                }
                _ => return self.phantom_settings.update(message),
            },
            Message::Download(state, index) => match state {
                DownloadMessage::DownloadSelected => {
                    return Command::done(Message::ShowMetadata(index));
                }
                DownloadMessage::RemoveDownload(force) => {
                    if force {
                        if let Some(download) = self.downloads.remove(&index) {
                            if !download.is_downloaded() || download.deleted {
                                if download.sequential {
                                    let path =
                                        PathBuf::from(download.file_path).join(download.file_name);
                                    if let Err(e) = std::fs::remove_file(&path) {
                                        warn!("Error deleting file {path:#?} : {e:#?}");
                                    }
                                } else {
                                    let path = PathBuf::from(&self.settings.cache_dir)
                                        .join(&download.file_name);
                                    for i in 1..=download.threads {
                                        let path = path.with_file_name(format!(
                                            "{}.atom.{}",
                                            download.file_name, i
                                        ));
                                        if let Err(e) = std::fs::remove_file(&path) {
                                            warn!("Error deleting file {path:#?} : {e:#?}");
                                        }
                                    }
                                }
                            }
                        }

                        if self.downloads.is_empty() {
                            self.update_view(View::Downloads);
                        }
                        return Command::done(Message::SaveDownloads);
                    } else if let Some(download) = self.downloads.get_mut(&index) {
                        download.update(state, &self.settings);
                    }
                }
                DownloadMessage::Finished => {
                    if let Some(download) = self.downloads.get_mut(&index) {
                        download.update(state, &self.settings);
                    }
                    return Command::done(Message::SaveDownloads);
                }
                _ => {
                    if let Some(download) = self.downloads.get_mut(&index) {
                        download.update(state, &self.settings);
                    }
                }
            },
            Message::NewDownloadReceivedFromBrowser(json) => {
                self.status_bar_message = "Adding new download to the list".to_string();
                let mut download = AtomDownload::new()
                    .headers(json.headers)
                    .url(json.url)
                    .file_name(json.file_name)
                    .file_size(json.size)
                    .file_path(&self.settings.downloads_dir)
                    .download_type(json.sequential);

                if json.method == "POST" {
                    download = download.request_body(json.body);
                }

                match download.build() {
                    Ok(atom_download) => {
                        if self.settings.new_download_notification {
                            show_notification(
                                "New download received from browser",
                                atom_download.file_name.clone().as_str(),
                                2000,
                            );
                        }

                        if self.settings.auto_start_download {
                            return Command::done(Message::AddNewDownload(atom_download));
                        } else {
                            #[cfg(target_os = "windows")]
                            let platform_specific_settings = PlatformSpecific {
                                undecorated_shadow: true,
                                ..Default::default()
                            };
                            #[cfg(not(target_os = "windows"))]
                            let platform_specific_settings = PlatformSpecific::default();

                            let (_id, open) = window::open(Settings {
                                position: window::Position::Centered,
                                visible: true,
                                resizable: false,
                                decorations: false,
                                transparent: false,
                                level: window::Level::AlwaysOnTop,
                                exit_on_close_request: true,
                                size: Size {
                                    width: 800.0,
                                    height: 680.0,
                                },
                                platform_specific: platform_specific_settings,
                                icon: Some(
                                    iced::window::icon::from_file_data(ATOM_ICON, None).unwrap(),
                                ),
                                ..Default::default()
                            });
                            return open.map(move |id| {
                                Message::WindowOpened(id, Some(atom_download.clone()))
                            });
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
                            && !download.deleted
                        {
                            Some(index)
                        } else {
                            None
                        }
                    })
                {
                    let mut existing_download =
                        self.downloads.remove(&existing_download_id).unwrap();
                    existing_download.downloading = true;

                    if let Some(entry) = self.downloads.first_key_value() {
                        self.downloads.insert(entry.0 - 1, existing_download);
                    } else {
                        self.downloads
                            .insert(get_current_time_in_millis(), existing_download);
                    }
                } else {
                    match (
                        self.downloads.first_key_value(),
                        &self.settings.new_download_pos[..],
                    ) {
                        (Some(entry), "First") => {
                            self.downloads.insert(entry.0 - 1, new_download);
                        }
                        _ => {
                            self.downloads
                                .insert(get_current_time_in_millis(), new_download);
                        }
                    };
                }

                let _ = self.update(Message::GotoHomePage);
                self.status_bar_message = "Added new download to the list".to_string();
                return Command::done(Message::SaveDownloads);
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
                self.status_bar_message = "Main View".to_string();
                self.download_state_filter_bar.show_confirmation_dialog = false;
                self.view = View::Downloads;
                self.filter_type = DownloadsListFilterMessage::All;
                self.metadata.enabled = false;
                self.sidebar.active = if self.downloads.is_empty() {
                    SideBarActiveButton::AddDownload
                } else {
                    SideBarActiveButton::Overview
                };
            }
            Message::DownloadForm(message, window_id) => {
                self.metadata.enabled = false;
                if window_id.is_some() {
                    if let Some(window) = self.windows.get_mut(&window_id.unwrap()) {
                        match message {
                            crate::messages::DownloadFormMessage::Minimize => {
                                return window::minimize(window_id.unwrap(), true);
                            }
                            crate::messages::DownloadFormMessage::AddNewDownload => {
                                let window_id = window_id.unwrap();
                                if let Some(window) = self.windows.get_mut(&window_id) {
                                    if let Ok(download) = window.1.make_download() {
                                        return Command::batch([
                                            Command::done(Message::AddNewDownload(download)),
                                            Command::done(Message::WindowClosed(window_id)),
                                        ]);
                                    }
                                }
                            }
                            crate::messages::DownloadFormMessage::ClosePane => {
                                let window_id = window_id.unwrap();
                                return Command::done(Message::WindowClosed(window_id));
                            }
                            _ => {
                                return window
                                    .1
                                    .update(message, &self.settings)
                                    .map(move |message| Message::DownloadForm(message, window_id))
                            }
                        }
                    }
                } else {
                    match message {
                        crate::messages::DownloadFormMessage::AddNewDownload => {
                            if let Ok(download) = self.download_form.make_download() {
                                return Command::done(Message::AddNewDownload(download));
                            }
                        }
                        crate::messages::DownloadFormMessage::ClosePane => {
                            return Command::done(Message::GotoHomePage);
                        }
                        _ => {
                            return self
                                .download_form
                                .update(message, &self.settings)
                                .map(move |message| Message::DownloadForm(message, window_id))
                        }
                    }
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
                    self.update_view(View::Import);
                }
                SidebarMessage::ResumeAll => {
                    self.downloads.iter_mut().for_each(|(&_, download)| {
                        if !download.is_downloaded() && !download.downloading {
                            download.update(DownloadMessage::Downloading, &self.settings);
                        }
                    });
                    self.sidebar.active = SideBarActiveButton::Overview;
                    self.metadata.enabled = false;
                }
                SidebarMessage::DeleteConfirm => {
                    self.download_state_filter_bar.show_confirmation_dialog = true;
                }
                SidebarMessage::DeleteAll => {
                    match self.sidebar.active {
                        SideBarActiveButton::Overview => {
                            if !self.titlebar.search_text.is_empty() {
                                self.downloads.retain(|_, download| {
                                    !download.deleted
                                        && !download
                                            .get_file_name()
                                            .to_lowercase()
                                            .contains(&self.titlebar.search_text)
                                });
                            } else {
                                self.downloads.clear()
                            }
                        }
                        SideBarActiveButton::Downloading => {
                            self.downloads.retain(|_, download| {
                                !(download.downloaded < download.size && download.downloading)
                                    || download.deleted
                            });
                        }
                        SideBarActiveButton::Paused => {
                            self.downloads.retain(|_, download| {
                                download.downloading
                                    || download.size <= download.downloaded
                                    || download.deleted
                            });
                        }
                        SideBarActiveButton::Finished => {
                            self.downloads.retain(|_, download| {
                                download.downloaded < download.size || download.deleted
                            });
                        }
                        SideBarActiveButton::Trash => {
                            self.downloads.retain(|_, download| !download.deleted);
                        }
                        _ => {}
                    }

                    if self.downloads.is_empty() {
                        self.filter_type = DownloadsListFilterMessage::All;
                        self.view = View::Downloads;
                        self.sidebar.active = SideBarActiveButton::Overview;
                        self.metadata.enabled = false;
                    }

                    self.download_state_filter_bar.show_confirmation_dialog = false;
                }
                SidebarMessage::PauseAll => {
                    self.downloads.iter_mut().for_each(|(&_, download)| {
                        download.update(DownloadMessage::Paused, &self.settings);
                    });
                    self.metadata.enabled = false;
                    self.sidebar.active = SideBarActiveButton::Overview;
                }
                SidebarMessage::Settings => {
                    self.filter_type = DownloadsListFilterMessage::All;
                    self.update_view(View::Settings);
                }
                SidebarMessage::Shortcuts => {
                    self.filter_type = DownloadsListFilterMessage::All;
                    self.update_view(View::Shortcuts);
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
                        return Command::done(Message::TitleBar(TitleBarMessage::AppExit))
                    }
                };
                return Command::batch(vec![
                    Command::done(Message::TitleBar(TitleBarMessage::AppShow)),
                    Command::done(message),
                ]);
            }
            Message::TrayEvent(id) => {
                if let Some(message) = self.tray_event.get(&id) {
                    let message = message.to_owned();
                    return Command::done(message);
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
