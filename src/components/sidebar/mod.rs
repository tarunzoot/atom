mod view;
use crate::messages::{Message, SideBarActiveButton, SideBarState, SidebarMessage};

#[derive(Debug)]
struct SidebarButton<'a> {
    text: &'a str,
    icon: char,
    message: Message,
    tooltip: &'a str,
    name: SideBarActiveButton,
}

#[derive(Debug)]
pub struct AtomSidebar<'a> {
    pub active: SideBarActiveButton,
    pub state: SideBarState,
    menu_buttons: Vec<SidebarButton<'a>>,
}

impl<'a> Default for AtomSidebar<'a> {
    fn default() -> Self {
        let menu_buttons = vec![
            SidebarButton {
                text: "Overview",
                icon: '\u{e944}',
                message: Message::GotoHomePage,
                tooltip: "Home",
                name: SideBarActiveButton::Overview,
            },
            SidebarButton {
                text: "Add Download",
                icon: '\u{efc0}',
                message: Message::Sidebar(SidebarMessage::NewDownloadForm),
                tooltip: "Add new download",
                name: SideBarActiveButton::AddDownload,
            },
            SidebarButton {
                text: "Pause All",
                icon: '\u{eca5}',
                message: Message::Sidebar(SidebarMessage::PauseAll),
                tooltip: "Pause all downloads",
                name: SideBarActiveButton::PauseAll,
            },
            SidebarButton {
                text: "Resume All",
                icon: '\u{eca8}',
                message: Message::Sidebar(SidebarMessage::ResumeAll),
                tooltip: "Resume all downloads",
                name: SideBarActiveButton::ResumeAll,
            },
            SidebarButton {
                text: "Delete All",
                icon: '\u{edec}',
                message: Message::Sidebar(SidebarMessage::DeleteConfirm),
                tooltip: "Delete all downloads",
                name: SideBarActiveButton::DeleteAll,
            },
            SidebarButton {
                text: "Import",
                icon: '\u{ec84}',
                message: Message::Sidebar(SidebarMessage::Import),
                tooltip: "Import file with links",
                name: SideBarActiveButton::Import,
            },
            SidebarButton {
                text: "Settings",
                icon: '\u{ec83}',
                message: Message::Sidebar(SidebarMessage::Settings),
                tooltip: "Settings",
                name: SideBarActiveButton::Settings,
            },
            SidebarButton {
                text: "Shortcuts",
                icon: '\u{ea54}',
                message: Message::Sidebar(SidebarMessage::Shortcuts),
                tooltip: "Shortcuts",
                name: SideBarActiveButton::Shortcuts,
            },
            SidebarButton {
                text: "Collapse",
                icon: '\u{ef1e}',
                message: Message::Sidebar(SidebarMessage::Expand),
                tooltip: "Expand sidebar",
                name: SideBarActiveButton::Null,
            },
        ];

        Self {
            active: SideBarActiveButton::Overview,
            state: SideBarState::Collapsed,
            menu_buttons,
        }
    }
}

impl<'a> AtomSidebar<'a> {
    pub fn new(active: SideBarActiveButton, state: SideBarState) -> Self {
        Self {
            active,
            state,
            menu_buttons: Self::default().menu_buttons,
        }
    }
}
