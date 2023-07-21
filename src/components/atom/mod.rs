mod update;
mod view;
use crate::{
    components::download::AtomDownload,
    components::{
        download_state::AtomDownloadStates, form::AtomDownloadForm, import::AtomImport,
        metadata::AtomDownloadMetadata, settings::AtomSettings, sidebar::AtomSidebar,
        titlebar::AtomTitleBar,
    },
    messages::{DownloadsListFilterMessage, Message, SideBarActiveButton, SideBarState},
    utils::helpers::load_tray_icon,
};
use crate::{
    style::Theme,
    utils::helpers::{handle_web_request, listen_for_tray_events},
};
use iced::{executor, Application, Command, Subscription};
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

impl<'a> Application for Atom<'a> {
    type Message = Message;
    type Flags = (AtomSettings, BTreeMap<usize, AtomDownload>, SingleInstance);
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::new(flags.0, flags.1, flags.2), Command::none())
    }

    fn title(&self) -> String {
        "A.T.O.M".to_string()
    }

    fn scale_factor(&self) -> f64 {
        if self.scale_factor < 1.0 {
            1.0
        } else {
            self.scale_factor
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let mut subscriptions: Vec<_> = self
            .downloads
            .iter()
            .map(|(&index, download)| download.subscription(index, &self.settings.cache_dir))
            .collect();

        subscriptions.push(iced_native::subscription::events().map(Message::EventsOccurred));
        // subscriptions.push(iced::window::frames().map(|_| Message::Tick));
        subscriptions.push(handle_web_request(self.should_exit));
        if self.tray.is_some() && !self.should_exit {
            subscriptions.push(listen_for_tray_events());
        }
        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        self.update(message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.view()
    }
}
