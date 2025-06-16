use std::collections::BTreeMap;

use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::{DownloadsListFilterMessage, Message},
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomTheme},
};
use iced::{
    widget::{button, column as col, container, row, text},
    Alignment, Element, Font,
    Length::{Fill, Shrink},
    Padding, Renderer,
};

use super::{download::AtomDownload, sidebar::SideBarActiveButton};

#[derive(Debug, Clone)]
struct FilterButton<'a> {
    icon: char,
    icon_font: CustomFont,
    text: &'a str,
    message: Message,
    state: SideBarActiveButton,
    tooltip: &'a str,
}

#[derive(Debug)]
pub struct AtomDownloadStatesFilterBar<'a> {
    download_filter_buttons: Vec<FilterButton<'a>>,
}

impl Default for AtomDownloadStatesFilterBar<'_> {
    fn default() -> Self {
        let df_buttons = vec![
            FilterButton {
                icon: '\u{ef74}',
                icon_font: CustomFont::IcoFont,
                text: "All",
                message: Message::GotoHomePage,
                state: SideBarActiveButton::Overview,
                tooltip: "All Downloads",
            },
            FilterButton {
                icon: '\u{eee5}',
                icon_font: CustomFont::IcoFont,
                text: "Downloading",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Downloading),
                state: SideBarActiveButton::Downloading,
                tooltip: "Downloading",
            },
            FilterButton {
                icon: '\u{eca5}',
                icon_font: CustomFont::IcoFont,
                text: "Paused",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Paused),
                state: SideBarActiveButton::Paused,
                tooltip: "Paused",
            },
            FilterButton {
                icon: '\u{f00d}',
                icon_font: CustomFont::IcoFont,
                text: "Finished",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Finished),
                state: SideBarActiveButton::Finished,
                tooltip: "Finished",
            },
            FilterButton {
                icon: '\u{ec53}',
                icon_font: CustomFont::IcoFont,
                text: "Trash",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Deleted),
                state: SideBarActiveButton::Trash,
                tooltip: "Trashed",
            },
            FilterButton {
                text: "Failed",
                icon_font: CustomFont::Symbols,
                icon: '\u{f0164}',
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Failed),
                state: SideBarActiveButton::Failed,
                tooltip: "Failed",
            },
        ];

        Self {
            download_filter_buttons: df_buttons,
        }
    }
}

impl AtomDownloadStatesFilterBar<'_> {
    pub fn view(
        &self,
        active: &SideBarActiveButton,
        downloads: &BTreeMap<usize, AtomDownload>,
        icons_only: bool,
    ) -> Element<Message, AtomTheme, Renderer> {
        let mut count_downloading = 0;
        let mut count_paused = 0;
        let mut count_deleted = 0;
        let mut count_finished = 0;
        let mut count_failed = 0;

        downloads.iter().for_each(|f| {
            if f.1.deleted {
                count_deleted += 1;
            } else if f.1.downloading || f.1.joining {
                count_downloading += 1;
            } else if !f.1.error.is_empty() {
                count_failed += 1;
            } else if !f.1.is_downloaded() && !f.1.is_downloading() {
                count_paused += 1;
            } else if f.1.is_downloaded() {
                count_finished += 1;
            }
        });

        let df_buttons_row = self.download_filter_buttons.iter().fold(
            row!()
                .spacing(0)
                .padding(0)
                .align_y(iced::Alignment::Center),
            |row, dfb| {
                let mut btn_content = row!()
                    .padding(Padding::from([10, 15]))
                    .align_y(Alignment::Center)
                    .spacing(5)
                    .push(icon(dfb.icon, dfb.icon_font.clone()).size(12));

                if !dfb.text.is_empty() && !icons_only {
                    btn_content = btn_content.push(text(dfb.text).size(12).font(Font {
                        family: iced::font::Family::Name("Lexend Deca"),
                        weight: iced::font::Weight::Black,
                        ..Default::default()
                    }));
                }

                btn_content = btn_content.push(GuiElements::round_text_button(match dfb.state {
                    SideBarActiveButton::Downloading => count_downloading,
                    SideBarActiveButton::Paused => count_paused,
                    SideBarActiveButton::Finished => count_finished,
                    SideBarActiveButton::Trash => count_deleted,
                    SideBarActiveButton::Failed => count_failed,
                    _ => downloads.len(),
                }));

                let active_bar = container(text(".").width(1))
                    .padding(0)
                    .height(3)
                    .width(30)
                    .class(if active == &dfb.state {
                        AtomStyleContainer::MenuBarActiveContainer
                    } else {
                        AtomStyleContainer::MenuBarInActiveContainer
                    });

                let df_button = button(
                    container(
                        col!()
                            .padding(0)
                            .spacing(0)
                            .align_x(Alignment::Center)
                            .push(active_bar)
                            .push(btn_content),
                    )
                    .class(AtomStyleContainer::ButtonContainer)
                    .center(Shrink)
                    .width(Shrink)
                    .padding(0),
                )
                .padding(match dfb.state {
                    SideBarActiveButton::PauseAll
                    | SideBarActiveButton::DeleteAll
                    | SideBarActiveButton::ResumeAll => 5,
                    SideBarActiveButton::Overview if dfb.text.is_empty() => 5,
                    _ => 0,
                })
                .width(Fill)
                .class(AtomStyleButton::SidebarButton)
                .on_press(dfb.message.clone());

                if icons_only {
                    row.push(GuiElements::tooltip_top(df_button, dfb.tooltip))
                } else {
                    row.push(df_button)
                }
            },
        );

        container(df_buttons_row)
            .padding(0)
            .center(Shrink)
            .height(Shrink)
            .width(Fill)
            .class(AtomStyleContainer::ListHeaderContainer)
            .into()
    }
}
