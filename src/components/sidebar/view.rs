use super::AtomSidebar;
use crate::{
    font::{icon, CustomFont},
    messages::{SideBarState, SidebarMessage},
    style::{button::AtomStyleButton, container::AtomStyleContainer, Theme},
};
use iced::{
    widget::{button, column as col, container, row, text, tooltip, vertical_space},
    Element, Length, Renderer,
};

impl<'a> AtomSidebar<'a> {
    pub fn get_sidebar_button(&self) -> Element<SidebarMessage, Theme, Renderer> {
        let icon_size = 20;
        let button_padding = 0;

        self.menu_buttons
            .iter()
            .fold(
                col!().height(iced::Length::FillPortion(2)),
                |mut col, mb| {
                    let (mb_icon, mb_tooltip) =
                        if matches!(self.state, SideBarState::Full) && mb.text == "Collapse" {
                            ('\u{eabf}', "Collapse sidebar")
                        } else {
                            (mb.icon, mb.tooltip)
                        };

                    let mut mbtn_bar = container(text("").width(iced::Length::Fixed(0.0)))
                        .padding(0)
                        .height(Length::Fixed(25.0))
                        .width(Length::Fixed(5.0));

                    if self.active == mb.name {
                        mbtn_bar = mbtn_bar.style(AtomStyleContainer::MenuBarActiveContainer);
                    } else {
                        mbtn_bar = mbtn_bar.style(AtomStyleContainer::MenuBarInActiveContainer);
                    }
                    let mut content_row = row!()
                        .align_items(iced::Alignment::Center)
                        .spacing(0)
                        .push(mbtn_bar)
                        .push(
                            container(icon(mb_icon, CustomFont::IcoFont).size(icon_size))
                                .padding(iced::Padding::from([15, 20, 15, 15]))
                                .style(AtomStyleContainer::ButtonContainer),
                        );

                    if !matches!(self.state, SideBarState::Collapsed) {
                        content_row = content_row.push(text(mb.text)).width(iced::Length::Fill);
                    }

                    if mb.text == "Collapse" || mb.text == "Expand" {
                        col = col.push(vertical_space().height(Length::Fill));
                    }

                    let mut mbtn = button(
                        container(content_row)
                            .style(AtomStyleContainer::ButtonContainer)
                            .center_y()
                            // .width(iced::Length::Fill)
                            .padding(button_padding),
                    )
                    .width(if matches!(self.state, SideBarState::Collapsed) {
                        iced::Length::Shrink
                    } else {
                        iced::Length::Fill
                    })
                    .padding(1)
                    .on_press(
                        if mb.text == "Collapse" && matches!(self.state, SideBarState::Full) {
                            SidebarMessage::Collapse
                        } else {
                            mb.message.to_owned()
                        },
                    );

                    if self.active == mb.name {
                        mbtn = mbtn.style(AtomStyleButton::SidebarButtonActive);
                    } else {
                        mbtn = mbtn.style(AtomStyleButton::SidebarButton);
                    }

                    col.push(
                        tooltip(mbtn, text(mb_tooltip).size(12), tooltip::Position::Right)
                            .gap(10)
                            .padding(10)
                            .style(AtomStyleContainer::ToolTipContainer),
                    )
                },
            )
            .into()
    }

    pub fn view(&self) -> Element<SidebarMessage, Theme, Renderer> {
        let menu_buttons = self.get_sidebar_button();

        container(
            col!()
                .width(if matches!(self.state, SideBarState::Collapsed) {
                    iced::Length::Shrink
                } else {
                    iced::Length::Fill
                })
                .height(iced::Length::Fill)
                .spacing(10)
                .push(
                    container(
                        col!()
                            .spacing(10)
                            .push(col!().spacing(2).push(menu_buttons)),
                    )
                    .center_x()
                    .padding(0)
                    .height(iced::Length::Fill)
                    .style(AtomStyleContainer::ListContainer),
                ),
        )
        .center_x()
        .width(if matches!(self.state, SideBarState::Collapsed) {
            iced::Length::Shrink
        } else {
            iced::Length::Fixed(200.0)
        })
        .height(iced::Length::Fill)
        .into()
    }
}
