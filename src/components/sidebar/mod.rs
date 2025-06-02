mod view;
use crate::messages::SidebarMessage;

#[derive(Debug, Clone, Default)]
pub enum SideBarState {
    #[default]
    Collapsed,
    Expanded,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum SideBarActiveButton {
    #[default]
    Overview,
    AddDownload,
    PauseAll,
    DeleteAll,
    ResumeAll,
    Settings,
    Shortcuts,
    Import,
    Downloading,
    Paused,
    Finished,
    Trash,
    Failed,
    Null,
}

impl From<SideBarActiveButton> for String {
    fn from(value: SideBarActiveButton) -> Self {
        match value {
            SideBarActiveButton::Downloading => "in progress".to_owned(),
            SideBarActiveButton::Paused => "paused".to_owned(),
            SideBarActiveButton::Finished => "finished".to_owned(),
            SideBarActiveButton::Trash => "trashed".to_owned(),
            _ => "all".to_owned(),
        }
    }
}
#[derive(Debug)]
pub struct SidebarButton<'a> {
    text: &'a str,
    icon: char,
    message: SidebarMessage,
    tooltip: &'a str,
    name: SideBarActiveButton,
}

#[derive(Debug)]
pub struct AtomSidebar<'a> {
    pub active: SideBarActiveButton,
    pub state: SideBarState,
    pub show_dialog: bool,
    buttons_primary: Vec<SidebarButton<'a>>,
    buttons_secondary: Vec<SidebarButton<'a>>,
    button_tertiary: SidebarButton<'a>,
}

impl Default for AtomSidebar<'_> {
    fn default() -> Self {
        let buttons_primary = vec![
            SidebarButton {
                text: "Overview",
                icon: '\u{e944}',
                message: SidebarMessage::GotoHomePage,
                tooltip: "Home",
                name: SideBarActiveButton::Overview,
            },
            SidebarButton {
                text: "Add Download",
                icon: '\u{efc0}',
                message: SidebarMessage::NewDownloadForm,
                tooltip: "Add new download",
                name: SideBarActiveButton::AddDownload,
            },
            SidebarButton {
                text: "Import",
                icon: '\u{ec84}',
                message: SidebarMessage::Import,
                tooltip: "Import file with links",
                name: SideBarActiveButton::Import,
            },
            SidebarButton {
                text: "Settings",
                icon: '\u{ec83}',
                message: SidebarMessage::Settings,
                tooltip: "Settings",
                name: SideBarActiveButton::Settings,
            },
            SidebarButton {
                text: "Shortcuts",
                icon: '\u{ea54}',
                message: SidebarMessage::Shortcuts,
                tooltip: "Shortcuts",
                name: SideBarActiveButton::Shortcuts,
            },
        ];

        let buttons_secondary = vec![
            SidebarButton {
                text: "Pause All",
                icon: '\u{eca5}',
                message: SidebarMessage::PauseAll,
                tooltip: "Pause All Downloads",
                name: SideBarActiveButton::Null,
            },
            SidebarButton {
                text: "Resume All",
                icon: '\u{eca8}',
                message: SidebarMessage::ResumeAll,
                tooltip: "Resume All Downloads",
                name: SideBarActiveButton::Null,
            },
            SidebarButton {
                text: "Delete All",
                icon: '\u{edec}',
                message: SidebarMessage::DeleteConfirm,
                tooltip: "Delete All Downloads",
                name: SideBarActiveButton::Null,
            },
        ];

        let button_tertiary = SidebarButton {
            text: "Collapse",
            icon: '\u{ef1e}',
            message: SidebarMessage::Expand,
            tooltip: "Expand sidebar",
            name: SideBarActiveButton::Null,
        };

        Self {
            active: SideBarActiveButton::Overview,
            state: SideBarState::Collapsed,
            show_dialog: false,
            buttons_primary,
            buttons_secondary,
            button_tertiary,
        }
    }
}

impl AtomSidebar<'_> {
    pub fn new(active: SideBarActiveButton, state: SideBarState) -> Self {
        let buttons = Self::default();
        Self {
            active,
            state,
            show_dialog: false,
            buttons_primary: buttons.buttons_primary,
            buttons_secondary: buttons.buttons_secondary,
            button_tertiary: buttons.button_tertiary,
        }
    }
}
