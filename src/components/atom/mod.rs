mod update;
mod view;
use crate::{
    components::download::AtomDownload,
    components::{
        download_state::AtomDownloadStates, form::AtomDownloadForm, import::AtomImport,
        metadata::AtomDownloadMetadata, settings::AtomSettings, sidebar::AtomSidebar,
        titlebar::AtomTitleBar,
    },
    font::{DEFAULT_APP_FONT, ICOFONT_BYTES, SYMBOLS_BYTES},
    messages::{DownloadsListFilterMessage, Message, SideBarActiveButton, SideBarState},
    style::Theme,
    utils::helpers::{
        get_conf_directory, load_tray_icon, parse_downloads_toml, parse_settings_toml,
        save_settings_toml,
    },
    utils::helpers::{handle_web_request, listen_for_tray_events},
};
use iced::{
    executor, subscription,
    widget::{container, text},
    Application, Command, Length, Subscription,
};
use single_instance::SingleInstance;
use std::{
    collections::{BTreeMap, HashMap},
    fs::create_dir_all,
};
use tray_icon::{
    menu::{Menu, MenuItem},
    TrayIcon, TrayIconBuilder,
};

#[derive(Debug, Clone, Default)]
pub enum View {
    NewDownloadForm,
    Settings,
    Shortcuts,
    #[default]
    Downloads,
    DeleteConfirm,
    Import,
}

pub enum AtomState<'a> {
    Loading,
    Loaded(Atom<'a>),
}

#[derive(Default)]
pub struct Atom<'a> {
    pub view: View,
    pub sidebar: AtomSidebar<'a>,
    pub titlebar: AtomTitleBar,
    pub filters: AtomDownloadStates<'a>,
    pub download_form: AtomDownloadForm,
    pub downloads: BTreeMap<usize, AtomDownload>,
    pub settings: AtomSettings,
    pub metadata: AtomDownloadMetadata,
    pub filter_type: DownloadsListFilterMessage,
    pub import: AtomImport,
    pub should_exit: bool,
    pub instance: Option<SingleInstance>,
    pub tray: Option<TrayIcon>,
    pub tray_event: HashMap<u32, Message>,
    pub scale_factor: f64,
    pub default_settings: AtomSettings,
}

impl<'a> Atom<'a> {
    pub fn new() -> Self {
        // check single instance of application
        let app_instance =
            single_instance::SingleInstance::new("fade9985-845c-4ca3-84b2-8a1b29a6c636")
                .map_err(|_| {
                    log::error!("SingleInstance cannot be initialized!");
                    std::process::exit(-1);
                })
                .unwrap();

        // check if config path can be created or exists
        let config_dir_path = get_conf_directory()
            .map_err(|e| {
                log::error!("{e:#?}");
                std::process::exit(1);
            })
            .unwrap();

        if !config_dir_path.exists() && create_dir_all(&config_dir_path).is_err() {
            log::error!("Error: cannot create config directory `{config_dir_path:#?}`, exiting.");
            std::process::exit(1);
        }

        let settings_path = config_dir_path.join("settings.toml");
        if !settings_path.exists() {
            log::warn!("No settings.toml found, using defaults");
            save_settings_toml(&AtomSettings {
                ..Default::default()
            });
        }

        let settings = parse_settings_toml(&settings_path);
        let downloads_toml_path =
            std::path::PathBuf::from(&settings.config_dir).join("downloads.toml");
        let downloads: BTreeMap<usize, AtomDownload> = parse_downloads_toml(&downloads_toml_path);

        let sidebar = AtomSidebar::new(
            if downloads.is_empty() {
                SideBarActiveButton::AddDownload
            } else {
                SideBarActiveButton::Overview
            },
            if settings.sidebar_collapsed {
                SideBarState::Collapsed
            } else {
                SideBarState::Full
            },
        );

        let (tray_icon, tray_messages) = if !app_instance.is_single() || cfg!(target_os = "linux") {
            (None, HashMap::default())
        } else {
            let tray_menu = Menu::new();
            let menu_items = vec![
                (
                    MenuItem::new("Show App", true, None),
                    Message::TrayMessages(crate::messages::TrayMessage::ShowApp),
                ),
                (
                    MenuItem::new("Add download", true, None),
                    Message::TrayMessages(crate::messages::TrayMessage::AddNewDownload),
                ),
                (
                    MenuItem::new("Settings", true, None),
                    Message::TrayMessages(crate::messages::TrayMessage::Settings),
                ),
                (
                    MenuItem::new("Import Links", true, None),
                    Message::TrayMessages(crate::messages::TrayMessage::Import),
                ),
                (
                    MenuItem::new("Exit", true, None),
                    Message::TrayMessages(crate::messages::TrayMessage::Exit),
                ),
            ];

            let tray_messages = menu_items
                .into_iter()
                .map(|item| {
                    tray_menu.append(&item.0);
                    (item.0.id(), item.1)
                })
                .collect();

            let tray_icon = if let Ok(tray) = TrayIconBuilder::new()
                .with_menu(Box::new(tray_menu))
                .with_tooltip("A.T.O.M Download Manager")
                .with_icon(load_tray_icon(include_bytes!(
                    "../../../resources/images/icon.ico"
                )))
                .build()
            {
                log::debug!("Tray menu enabled!");
                Some(tray)
            } else {
                log::debug!("Tray menu creation failed!");
                None
            };
            (tray_icon, tray_messages)
        };

        Self {
            default_settings: settings.clone(),
            settings,
            sidebar,
            downloads,
            instance: Some(app_instance),
            tray: tray_icon,
            tray_event: tray_messages,
            ..Default::default()
        }
    }
}

impl<'a> Application for AtomState<'a> {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Message>) {
        (
            AtomState::Loading,
            Command::batch(vec![
                iced::font::load(DEFAULT_APP_FONT).map(Message::FontLoaded),
                iced::font::load(ICOFONT_BYTES).map(Message::FontLoaded),
                iced::font::load(SYMBOLS_BYTES).map(Message::FontLoaded),
                Command::perform(async {}, |_| Message::LoadingComplete),
            ]),
        )
    }

    fn title(&self) -> String {
        "A.T.O.M".to_string()
    }

    fn scale_factor(&self) -> f64 {
        match self {
            AtomState::Loading => 1.0,
            AtomState::Loaded(atom) => {
                if atom.scale_factor < 1.0 {
                    1.0
                } else {
                    atom.scale_factor
                }
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        match self {
            AtomState::Loading => Subscription::none(),
            AtomState::Loaded(atom) => {
                let mut subscriptions: Vec<_> = atom
                    .downloads
                    .iter()
                    .map(|(&index, download)| {
                        download.subscription(index, &atom.settings.cache_dir)
                    })
                    .collect();

                subscriptions.push(subscription::events().map(Message::EventsOccurred));
                // subscriptions.push(iced::window::frames().map(|_| Message::Tick));
                subscriptions.push(handle_web_request(atom.should_exit));
                if atom.tray.is_some() && !atom.should_exit {
                    subscriptions.push(listen_for_tray_events());
                }
                Subscription::batch(subscriptions)
            }
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match self {
            AtomState::Loading => {
                if let Message::LoadingComplete = message {
                    *self = AtomState::Loaded(Atom::new());
                }
                iced::Command::none()
            }
            AtomState::Loaded(atom) => atom.update(message),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self {
            AtomState::Loading => container(
                text("loading...")
                    .size(50)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into(),
            AtomState::Loaded(atom) => atom.view(),
        }
    }
}
