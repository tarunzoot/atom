use crate::{
    font::{icon, CustomFont},
    messages::{Message, SidebarMessage},
    style::{AtomStyleContainer, Theme},
    utils::helpers::{atom_button, ButtonType},
};
use iced::{
    widget::{column, container, row, text},
    Element, Length, Renderer,
};

pub fn view() -> Element<'static, Message, Renderer<Theme>> {
    container(
        column!()
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .push(text("Are you sure you want to delete all downloads?"))
            .push(
                container(
                    row!()
                        .spacing(20)
                        .width(Length::Shrink)
                        .align_items(iced::Alignment::Center)
                        .push(
                            atom_button(
                                ButtonType::IconWithText,
                                vec![icon('\u{ec53}', CustomFont::IcoFont), text("delete")],
                            )
                            .width(Length::Fixed(170.0))
                            .on_press(Message::Sidebar(SidebarMessage::DeleteAll)),
                        )
                        .push(
                            atom_button(
                                ButtonType::IconWithText,
                                vec![icon('\u{eede}', CustomFont::IcoFont), text("cancel")],
                            )
                            .width(Length::Fixed(170.0))
                            .on_press(Message::GotoHomePage),
                        ),
                )
                .style(AtomStyleContainer::Transparent)
                .center_x()
                .center_y()
                .width(Length::Fill),
            ),
    )
    .padding(15)
    .center_x()
    .center_y()
    .style(AtomStyleContainer::ListContainer)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
