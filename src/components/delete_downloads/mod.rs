use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::SidebarMessage,
    style::{AtomStyleContainer, Theme},
};
use iced::{
    widget::{column, container, row, text},
    Element, Length, Renderer,
};

pub fn view() -> Element<'static, SidebarMessage, Renderer<Theme>> {
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
                            GuiElements::primary_button(vec![
                                icon('\u{ec53}', CustomFont::IcoFont),
                                text("delete"),
                            ])
                            .width(Length::Fixed(170.0))
                            .on_press(SidebarMessage::DeleteAll),
                        )
                        .push(
                            GuiElements::primary_button(vec![
                                icon('\u{eede}', CustomFont::IcoFont),
                                text("cancel"),
                            ])
                            .width(Length::Fixed(170.0))
                            .on_press(SidebarMessage::GotoHomePage),
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
