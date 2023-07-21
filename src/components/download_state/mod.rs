use crate::{
    font::{icon, CustomFont},
    messages::{DownloadsListFilterMessage, Message, SideBarActiveButton},
    style::{AtomStyleButton, AtomStyleContainer, Theme},
};
use iced::{
    widget::{button, container, row, text},
    Element, Renderer,
};

#[derive(Debug)]
struct FilterButton<'a> {
    icon: char,
    text: &'a str,
    message: Message,
    state: SideBarActiveButton,
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
                text: "Downloading",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Downloading),
                state: SideBarActiveButton::Downloading,
            },
            FilterButton {
                icon: '\u{eca5}',
                text: "Paused",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Paused),
                state: SideBarActiveButton::Paused,
            },
            FilterButton {
                icon: '\u{f00d}',
                text: "Finished",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Finished),
                state: SideBarActiveButton::Finished,
            },
            FilterButton {
                icon: '\u{ec53}',
                text: "Trash",
                message: Message::DownloadsListFilter(DownloadsListFilterMessage::Deleted),
                state: SideBarActiveButton::Trash,
            },
        ];

        Self {
            download_filter_buttons: df_buttons,
        }
    }
}

impl<'a> AtomDownloadStates<'a> {
    pub fn view(&self, active: &SideBarActiveButton) -> Element<Message, Renderer<Theme>> {
        let df_buttons_row = self.download_filter_buttons.iter().fold(
            row!()
                .spacing(0)
                .padding(0)
                .align_items(iced::Alignment::Center),
            |row, dfb| {
                let mut df_button = button(
                    container(
                        row!()
                            .align_items(iced::Alignment::Center)
                            .spacing(10)
                            .push(icon(dfb.icon, CustomFont::IcoFont).size(16))
                            .push(text(dfb.text).size(16)),
                    )
                    .style(AtomStyleContainer::ButtonContainer)
                    .center_y()
                    .center_x()
                    .width(iced::Length::Fill)
                    .padding(5),
                )
                .width(iced::Length::Fill)
                .on_press(dfb.message.clone());
                if active == &dfb.state {
                    df_button = df_button.style(AtomStyleButton::DownloadFiltersButton);
                } else {
                    df_button = df_button.style(AtomStyleButton::SidebarButton);
                }
                row.push(df_button)
            },
        );

        container(df_buttons_row)
            .width(iced::Length::Fill)
            .style(AtomStyleContainer::ListHeaderContainer)
            .into()
    }
}
