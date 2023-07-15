mod update;
mod view;
use crate::{
    components::download::AtomDownload,
    components::{
        download_state::AtomDownloadStates, form::AtomDownloadForm, import::AtomImport,
        metadata::AtomDownloadMetadata, settings::AtomSettings, sidebar::AtomSidebar,
        titlebar::AtomTitleBar,
    },
    messages::{DownloadsFilterListMessage, Message, SideBarActiveButton, SideBarState},
    utils::helpers::load_tray_icon,
};
use single_instance::SingleInstance;
use std::collections::{BTreeMap, HashMap};
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
    pub filter_type: DownloadsFilterListMessage,
    pub import: AtomImport,
    pub should_exit: bool,
    pub instance: Option<SingleInstance>,
    pub tray: Option<TrayIcon>,
    pub tray_event: HashMap<u32, Message>,
    pub scale_factor: f64,
}

impl<'a> Atom<'a> {
    pub fn new(
        settings: AtomSettings,
        downloads: BTreeMap<usize, AtomDownload>,
        app_instance: SingleInstance,
    ) -> Self {
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
