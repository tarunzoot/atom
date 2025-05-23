use std::collections::BTreeMap;

use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::{DownloadsListFilterMessage, Message, SideBarActiveButton},
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomTheme},
};
use iced::{
    widget::{button, column as col, container, horizontal_space, row, text, tooltip},
    Alignment, Element, Font,
    Length::{Fill, Fixed, Shrink},
    Padding, Renderer,
};

use super::{download::AtomDownload, settings::ListLayout};

#[derive(Debug)]
struct FilterButton<'a> {
    icon: char,
    text: &'a str,
    message: Message,
    state: SideBarActiveButton,
    tooltip: Option<&'a str>,
}

#[derive(Debug)]
pub struct AtomDownloadStatesFilterBar<'a> {
    download_filter_buttons: Vec<FilterButton<'a>>,
    pub show_confirmation_dialog: bool,
}

impl Default for AtomDownloadStatesFilterBar<'_> {
    fn default() -> Self {
        let df_buttons = vec![
            FilterButton {
                icon: '\u{ef74}',
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
                tooltip: Some(
                    "Delete all downloads based on the current view (All, Paused, Trash etc...)",
                ),
            },
        ];

        Self {
            download_filter_buttons: df_buttons,
            show_confirmation_dialog: false,
        }
    }
}

impl AtomDownloadStatesFilterBar<'_> {
    pub fn view(
        &self,
        active: &SideBarActiveButton,
        downloads: &BTreeMap<usize, AtomDownload>,
        layout: &ListLayout,
        icons_only: bool,
        text_in_search_box: bool,
    ) -> Element<Message, AtomTheme, Renderer> {
        let count_downloading = downloads
            .iter()
            .filter(|f| f.1.downloading && !f.1.deleted)
            .count();
        let count_paused = downloads
            .iter()
            .filter(|f| !f.1.is_downloaded() && !f.1.is_downloading() && !f.1.deleted)
            .count();
        let count_deleted = downloads.iter().filter(|f| f.1.deleted).count();
        let count_finished = downloads
            .iter()
            .filter(|f| f.1.is_downloaded() && !f.1.deleted)
            .count();

        let df_buttons_row = self.download_filter_buttons.iter().fold(
            row!()
                .spacing(0)
                .padding(0)
                .align_y(iced::Alignment::Center),
            |mut row, dfb| {
                let btn_icon = if dfb.text.is_empty() {
                    match layout {
                        ListLayout::ListExtended => '\u{efa2}',
                        ListLayout::List => '\u{e90b}',
                    }
                } else {
                    dfb.icon
                };

                let mut btn_content = row!()
                    .padding(Padding::from([10, 15]))
                    .align_y(Alignment::Center)
                    .spacing(5)
                    .push(icon(btn_icon, CustomFont::IcoFont).size(12));

                if !dfb.text.is_empty() && !icons_only {
                    btn_content = btn_content.push(text(dfb.text).size(12).font(Font {
                        family: iced::font::Family::Name("Lexend Deca"),
                        weight: iced::font::Weight::Black,
                        ..Default::default()
                    }));
                }

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

                let mut active_bar = container(text(".").width(Fixed(1.0)))
                    .padding(0)
                    .height(Fixed(3.0))
                    .width(Fixed(30.0));

                if active == &dfb.state && dfb.tooltip.is_none() {
                    active_bar = active_bar.class(AtomStyleContainer::MenuBarActiveContainer);
                } else {
                    active_bar = active_bar.class(AtomStyleContainer::MenuBarInActiveContainer);
                }

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
                .width(Shrink)
                .class(AtomStyleButton::SidebarButton)
                .on_press(dfb.message.clone());

                if matches!(dfb.state, SideBarActiveButton::PauseAll) {
                    row = row.push(horizontal_space().width(Fill));
                }

                if let Some(tooltip_text) = dfb.tooltip {
                    row.push(
                        tooltip(
                            df_button,
                            text(tooltip_text).size(10),
                            tooltip::Position::Top,
                        )
                        .gap(5)
                        .padding(10)
                        .class(AtomStyleContainer::ToolTipContainer),
                    )
                } else {
                    row.push(df_button)
                }
            },
        );

        if self.show_confirmation_dialog {
            let conf_string = format!(
                "Are you sure you want to delete {} downloads?",
                if text_in_search_box {
                    "filtered".into()
                } else {
                    <SideBarActiveButton as Into<String>>::into(active.to_owned())
                }
            );
            GuiElements::modal(
                container(df_buttons_row)
                    .padding(0)
                    .center(Shrink)
                    .height(Shrink)
                    .width(Fill)
                    .class(AtomStyleContainer::ListHeaderContainer),
                text(conf_string).size(24),
                row!()
                    .spacing(10)
                    .align_y(Alignment::Center)
                    .push(
                        GuiElements::primary_button(vec![
                            icon('\u{ec53}', CustomFont::IcoFont),
                            text("delete"),
                        ])
                        .width(Fixed(170.0))
                        .on_press(Message::Sidebar(crate::messages::SidebarMessage::DeleteAll)),
                    )
                    .push(
                        GuiElements::primary_button(vec![
                            icon('\u{eedd}', CustomFont::IcoFont),
                            text("cancel"),
                        ])
                        .width(Fixed(170.0))
                        .on_press(Message::GotoHomePage),
                    ),
                Message::GotoHomePage,
            )
        } else {
            container(df_buttons_row)
                .padding(0)
                .center(Shrink)
                .height(Shrink)
                .width(Fill)
                .class(AtomStyleContainer::ListHeaderContainer)
                .into()
        }
    }
}
