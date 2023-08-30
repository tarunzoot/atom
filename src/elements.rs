use crate::{
    font::{icon, CustomFont},
    style::{button::AtomStyleButton, container::AtomStyleContainer, Theme},
};
use iced::{
    widget::{button, container, row},
    Element, Padding, Renderer,
};

pub struct GuiElements;

impl GuiElements {
    pub fn primary_button<'a, T>(
        contents: Vec<impl Into<Element<'a, T, Renderer<Theme>>>>,
    ) -> iced::widget::Button<'a, T, Renderer<Theme>>
    where
        T: std::fmt::Debug + 'a,
    {
        let content_row = contents
            .into_iter()
            .fold(row!(), |row, child| row.push(child))
            .align_items(iced::Alignment::Center)
            .spacing(5);

        button(
            container(content_row)
                .width(iced::Length::Fill)
                .center_x()
                .style(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::from([7, 15, 7, 15]))
    }

    pub fn round_button<'a, T>(icon_code: char) -> iced::widget::Button<'a, T, Renderer<Theme>>
    where
        T: std::fmt::Debug + 'a,
    {
        button(
            container(icon(icon_code, CustomFont::IcoFont).size(12))
                .width(iced::Length::Fill)
                .style(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::from([6, 8, 6, 8]))
        .style(AtomStyleButton::RoundButton)
    }
}
