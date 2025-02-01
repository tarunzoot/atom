use super::AtomSidebar;
use crate::{
    font::{icon, CustomFont},
    messages::{SideBarState, SidebarMessage},
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomTheme},
};
use iced::{
    widget::{button, column as col, container, row, text, tooltip, vertical_space},
    Alignment, Element,
    Length::{Fill, FillPortion, Fixed, Shrink},
    Padding, Renderer,
};

impl AtomSidebar<'_> {
    pub fn get_sidebar_button(&self) -> Element<SidebarMessage, AtomTheme, Renderer> {
        let icon_size = 20;
        let button_padding = 0;

        self.menu_buttons
            .iter()
            .fold(col!().spacing(10).height(FillPortion(2)), |mut col, mb| {
                let (mb_icon, mb_tooltip) =
                    if matches!(self.state, SideBarState::Full) && mb.text == "Collapse" {
                        ('\u{eabf}', "Collapse sidebar")
                    } else {
                        (mb.icon, mb.tooltip)
                    };

                let mut mbtn_bar = container(text("").width(Fixed(0.0)))
                    .padding(0)
                    .height(Fixed(25.0))
                    .width(Fixed(5.0));

                if self.active == mb.name {
                    mbtn_bar = mbtn_bar.class(AtomStyleContainer::MenuBarActiveContainer);
                } else {
                    mbtn_bar = mbtn_bar.class(AtomStyleContainer::MenuBarInActiveContainer);
                }
                let mut content_row = row!()
                    .align_y(Alignment::Center)
                    .spacing(0)
                    .push(mbtn_bar)
                    .push(
                        container(icon(mb_icon, CustomFont::IcoFont).size(icon_size))
                            .padding(Padding::new(15.0).right(20))
                            .class(AtomStyleContainer::ButtonContainer),
                    );

                if !matches!(self.state, SideBarState::Collapsed) {
                    content_row = content_row.push(text(mb.text)).width(Fill);
                }

                if mb.text == "Collapse" || mb.text == "Expand" {
                    col = col.push(vertical_space().height(Fill));
                }

                let mut mbtn = button(
                    container(content_row)
                        .class(AtomStyleContainer::ButtonContainer)
                        .center_x(Shrink)
                        .center_y(Shrink)
                        // .width(iced::Fill)
                        .padding(button_padding),
                )
                .width(if matches!(self.state, SideBarState::Collapsed) {
                    Shrink
                } else {
                    Fill
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
                    mbtn = mbtn.class(AtomStyleButton::SidebarButtonActive);
                } else {
                    mbtn = mbtn.class(AtomStyleButton::SidebarButton);
                }

                col.push(
                    tooltip(mbtn, text(mb_tooltip).size(12), tooltip::Position::Right)
                        .gap(10)
                        .padding(10)
                        .class(AtomStyleContainer::ToolTipContainer),
                )
            })
            .into()
    }

    pub fn view(&self) -> Element<SidebarMessage, AtomTheme, Renderer> {
        let menu_buttons = self.get_sidebar_button();

        container(
            col!()
                .width(if matches!(self.state, SideBarState::Collapsed) {
                    Shrink
                } else {
                    Fill
                })
                .height(Fill)
                .spacing(10)
                .push(
                    container(
                        col!()
                            .spacing(10)
                            .push(col!().spacing(2).push(menu_buttons)),
                    )
                    .center_x(Shrink)
                    .center_y(Shrink)
                    .padding(0)
                    .height(Shrink)
                    .class(AtomStyleContainer::ListContainer),
                ),
        )
        .center_x(Shrink)
        .center_y(Shrink)
        .width(if matches!(self.state, SideBarState::Collapsed) {
            Shrink
        } else {
            Fixed(200.0)
        })
        .height(Fill)
        .into()
    }
}
