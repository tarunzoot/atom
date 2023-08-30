use std::collections::BTreeMap;

use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::{DownloadsListFilterMessage, Message, SideBarActiveButton},
    style::{button::AtomStyleButton, container::AtomStyleContainer, Theme},
};
use iced::{
    widget::{button, column as col, container, horizontal_space, row, text, tooltip},
    Element, Length, Padding, Renderer,
};

use super::download::AtomDownload;

#[derive(Debug)]
struct FilterButton<'a> {
    icon: char,
    text: &'a str,
    message: Message,
    state: SideBarActiveButton,
    tooltip: Option<&'a str>,
}

#[derive(Debug)]
pub struct AtomDownloadStates<'a> {
    download_filter_buttons: Vec<FilterButton<'a>>,
}

impl<'a> Default for AtomDownloadStates<'a> {
    fn default() -> Self {
        let df_buttons = vec![
            FilterButton {
                icon: '\u{eee5}',
                text: "All",
                message: Message::GotoHomePage,
                state: SideBarActiveButton::Overview,
                tooltip: None,
            },
            FilterButton {
                icon: '\u{eee5}',
                text: "Downloading",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Downloading),
                state: SideBarActiveButton::Downloading,
                tooltip: None,
            },
            FilterButton {
                icon: '\u{eca5}',
                text: "Paused",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Paused),
                state: SideBarActiveButton::Paused,
                tooltip: None,
            },
            FilterButton {
                icon: '\u{f00d}',
                text: "Finished",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Finished),
                state: SideBarActiveButton::Finished,
                tooltip: None,
            },
            FilterButton {
                icon: '\u{ec53}',
                text: "Trash",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Deleted),
                state: SideBarActiveButton::Trash,
                tooltip: None,
            },
            FilterButton {
                text: "Pause",
                icon: '\u{eca5}',
                message: Message::Sidebar(crate::messages::SidebarMessage::PauseAll),
                state: SideBarActiveButton::PauseAll,
                tooltip: Some("Pause all downloads"),
            },
            FilterButton {
                text: "Resume",
                icon: '\u{eca8}',
                message: Message::Sidebar(crate::messages::SidebarMessage::ResumeAll),
                state: SideBarActiveButton::ResumeAll,
                tooltip: Some("Resume all downloads"),
            },
            FilterButton {
                text: "Delete",
                icon: '\u{edec}',
                message: Message::Sidebar(crate::messages::SidebarMessage::DeleteConfirm),
                state: SideBarActiveButton::DeleteAll,
                tooltip: Some("Delete all downloads"),
            },
        ];

        Self {
            download_filter_buttons: df_buttons,
        }
    }
}

impl<'a> AtomDownloadStates<'a> {
    pub fn view(
        &self,
        active: &SideBarActiveButton,
        downloads: &BTreeMap<usize, AtomDownload>,
    ) -> Element<Message, Renderer<Theme>> {
        let count_downloading = downloads
            .iter()
            .filter(|f| f.1.is_downloading && !f.1.is_deleted)
            .count();
        let count_paused = downloads
            .iter()
            .filter(|f| !f.1.is_downloaded() && !f.1.is_downloading() && !f.1.is_deleted)
            .count();
        let count_deleted = downloads.iter().filter(|f| f.1.is_deleted).count();
        let count_finished = downloads
            .iter()
            .filter(|f| f.1.is_downloaded() && !f.1.is_deleted)
            .count();

        let df_buttons_row = self.download_filter_buttons.iter().fold(
            row!()
                .spacing(0)
                .padding(0)
                .align_items(iced::Alignment::Center),
            |mut row, dfb| {
                let mut btn_content = row!()
                    .padding(Padding::from([8, 12]))
                    .align_items(iced::Alignment::Center)
                    .spacing(5)
                    .push(icon(dfb.icon, CustomFont::IcoFont).size(12))
                    .push(text(dfb.text).size(12));

                if dfb.tooltip.is_none() {
                    btn_content =
                        btn_content.push(GuiElements::round_text_button(match dfb.state {
                            SideBarActiveButton::Downloading => count_downloading,
                            SideBarActiveButton::Paused => count_paused,
                            SideBarActiveButton::Finished => count_finished,
                            SideBarActiveButton::Trash => count_deleted,
                            _ => downloads.len(),
                        }));
                }

                let mut active_bar = container(text(".").width(iced::Length::Fixed(1.0)))
                    .padding(0)
                    .height(Length::Fixed(3.0))
                    .width(Length::Fixed(30.0));

                if active == &dfb.state {
                    active_bar = active_bar.style(AtomStyleContainer::MenuBarActiveContainer);
                } else {
                    active_bar = active_bar.style(AtomStyleContainer::MenuBarInActiveContainer);
                }

                let df_button = button(
                    container(
                        col!()
                            .padding(0)
                            .spacing(0)
                            .align_items(iced::Alignment::Center)
                            .push(active_bar)
                            .push(btn_content),
                    )
                    .style(AtomStyleContainer::ButtonContainer)
                    .center_y()
                    .center_x()
                    .width(iced::Length::Fill)
                    .padding(0),
                )
                .padding(0)
                .width(iced::Length::Shrink)
                .style(AtomStyleButton::SidebarButton)
                .on_press(dfb.message.clone());

                if matches!(dfb.state, SideBarActiveButton::PauseAll) {
                    row = row.push(horizontal_space(Length::Fill));
                }

                if let Some(tooltip_text) = dfb.tooltip {
                    row.push(
                        tooltip(df_button, tooltip_text, tooltip::Position::Top)
                            .size(10)
                            .gap(5)
                            .padding(10)
                            .style(AtomStyleContainer::ToolTipContainer),
                    )
                } else {
                    row.push(df_button)
                }
            },
        );

        container(df_buttons_row)
            .padding(0)
            .width(iced::Length::Fill)
            .style(AtomStyleContainer::ListHeaderContainer)
            .into()
    }
}
