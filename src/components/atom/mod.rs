mod update;
mod view;
use crate::{
    components::{
        download::AtomDownload, download_state::AtomDownloadStatesFilterBar,
        form::AtomDownloadForm, import::AtomImport, metadata::AtomDownloadMetadata,
        settings::AtomSettings, sidebar::AtomSidebar, titlebar::AtomTitleBar,
    },
    messages::{DownloadsListFilterMessage, Message, SideBarActiveButton, SideBarState},
    style::AtomTheme,
    utils::helpers::{
        get_conf_directory, parse_downloads_toml, parse_settings_toml, save_settings_toml,
        ATOM_ICON,
    },
};
use iced::window::Id;
use reqwest::Client;
use single_instance::SingleInstance;
use std::{
    collections::{BTreeMap, HashMap},
    fs::create_dir_all,
};
use tracing::{debug, error, warn};
use tray_icon::{
    menu::{Menu, MenuId, MenuItem},
    TrayIcon, TrayIconBuilder,
};

#[derive(Debug, Clone, Default)]
pub enum View {
    NewDownloadForm,
    Settings,
    Shortcuts,
    #[default]
    Downloads,
    Import,
}

impl From<View> for String {
    fn from(value: View) -> Self {
        match value {
            View::NewDownloadForm => "New Download Add Form View",
            View::Settings => "Settings View",
            View::Shortcuts => "Shortcuts View",
            View::Downloads => "Downloads List View",
            View::Import => "Import Links View",
        }
        .to_string()
    }
}

#[derive(Default)]
pub struct Atom<'a> {
    pub client: Client,
    pub view: View,
    pub sidebar: AtomSidebar<'a>,
    pub titlebar: AtomTitleBar,
    pub download_state_filter_bar: AtomDownloadStatesFilterBar<'a>,
    pub download_form: AtomDownloadForm,
    pub downloads: BTreeMap<usize, AtomDownload>,
    pub settings: AtomSettings,
    pub phantom_settings: AtomSettings,
    pub metadata: AtomDownloadMetadata,
    pub filter_type: DownloadsListFilterMessage,
    pub import: AtomImport,
    pub instance: Option<SingleInstance>,
    pub tray: Option<TrayIcon>,
    pub tray_event: HashMap<MenuId, Message>,
    pub theme: AtomTheme,
    pub should_exit: bool,
    pub window_dimensions: (u32, u32), // width x height
    pub status_bar_message: String,
    pub alt_pressed: bool,
    pub mouse_over_titlebar: bool,
    pub windows: BTreeMap<Id, (&'a str, AtomDownloadForm)>,
}

impl Atom<'_> {
    pub fn new() -> Self {
        // check single instance of application
        let app_instance =
            single_instance::SingleInstance::new("fade9985-845c-4ca3-84b2-8a1b29a6c636")
                .map_err(|_| {
                    error!("SingleInstance cannot be initialized!");
                    std::process::exit(-1);
                })
                .unwrap();

        let client_builder = reqwest::ClientBuilder::new();

        let client = client_builder
            .danger_accept_invalid_certs(true)
            .brotli(true)
            .gzip(true)
            .deflate(true)
            .zstd(true)
            .referer(true)
            .build()
            .expect("Error: cannot create download client.");

        // check if config path can be created or exists
        let config_dir_path = get_conf_directory()
            .map_err(|e| {
                error!("{e:#?}");
                std::process::exit(1);
            })
            .unwrap();

        if !config_dir_path.exists() && create_dir_all(&config_dir_path).is_err() {
            error!("Error: cannot create config directory `{config_dir_path:#?}`, exiting.");
            std::process::exit(1);
        }

        let settings_path = config_dir_path.join("settings.toml");
        if !settings_path.exists() {
            warn!("No settings.toml found, using defaults");
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

        let (tray_icon, tray_messages) = Atom::load_system_tray(app_instance.is_single());

        Self {
            client,
            theme: settings.theme.clone().into(),
            phantom_settings: settings.clone(),
            settings,
            sidebar,
            downloads,
            instance: Some(app_instance),
            tray: tray_icon,
            tray_event: tray_messages,
            status_bar_message: String::from("App loaded"),
            alt_pressed: false,
            mouse_over_titlebar: false,
            ..Default::default()
        }
    }

    fn load_tray_icon(image_data: &[u8]) -> tray_icon::Icon {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::load_from_memory(image_data)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
    }

    fn load_system_tray(is_single: bool) -> (Option<TrayIcon>, HashMap<MenuId, Message>) {
        if !is_single || cfg!(target_os = "linux") {
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
                    (
                        tray_menu.append(&item.0).is_err(),
                        (item.0.id().to_owned(), item.1),
                    )
                })
                .filter(|f| {
                    if f.0 {
                        warn!("Error: tray item id:{:?}, message: {:?}", (f.1).0, (f.1).1);
                        false
                    } else {
                        true
                    }
                })
                .map(|item| item.1)
                .collect();

            let tray_icon = if let Ok(tray) = TrayIconBuilder::new()
                .with_menu(Box::new(tray_menu))
                .with_tooltip("A.T.O.M Download Manager")
                .with_icon(Self::load_tray_icon(ATOM_ICON))
                .build()
            {
                debug!("Tray menu enabled!");
                Some(tray)
            } else {
                warn!("Tray menu creation failed!");
                None
            };
            (tray_icon, tray_messages)
        }
    }
}
