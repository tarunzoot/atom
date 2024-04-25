use crate::{
    components::modal::Modal,
    font::{icon, CustomFont},
    style::{button::AtomStyleButton, container::AtomStyleContainer, Theme},
};
use iced::{
    widget::{button, column as col, container, row, text},
    Alignment, Element, Font, Length, Padding, Renderer,
};

pub struct GuiElements;

impl GuiElements {
    pub fn modal<'a, T, U>(
        base: T,
        confirmation_text: &str,
        actions: impl Into<Element<'a, U, Theme, Renderer>>,
        on_blur: U,
    ) -> Element<'a, U, Theme, Renderer>
    where
        T: Into<Element<'a, U, Theme, Renderer>>,
        U: Clone + std::fmt::Debug + 'static,
    {
        Modal::new(
            base,
            container(
                col!()
                    .align_items(Alignment::Center)
                    .spacing(30)
                    .push(
                        container(text("Confirmation Dialog"))
                            .style(AtomStyleContainer::LogoContainer)
                            .padding(Padding::from([10, 30, 10, 30])),
                    )
                    .push(
                        row!()
                            .align_items(Alignment::Center)
                            .spacing(5)
                            .push(icon('\u{efca}', CustomFont::IcoFont).size(24))
                            .push(text(confirmation_text).size(24)),
                    )
                    .push(actions)
                    .padding(Padding::from([0, 20, 20, 20])),
            )
            .style(AtomStyleContainer::ToolTipContainer)
            .padding(Padding::from([0, 30]))
            .width(Length::Shrink),
        )
        .on_blur(on_blur)
        .into()
    }

    pub fn primary_button<'a, T>(
        contents: Vec<impl Into<Element<'a, T, Theme, Renderer>>>,
    ) -> iced::widget::Button<'a, T, Theme, Renderer>
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
                .width(iced::Length::Shrink)
                .center_x()
                .style(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::from([7, 15, 7, 15]))
    }

    pub fn round_button<'a, T>(icon_code: char) -> iced::widget::Button<'a, T, Theme, Renderer>
    where
        T: std::fmt::Debug + 'a,
    {
        button(
            container(icon(icon_code, CustomFont::IcoFont).size(12))
                .width(iced::Length::Shrink)
                .style(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::from([6, 8, 6, 8]))
        .style(AtomStyleButton::RoundButton)
    }

    pub fn round_text_button<'a, T, U>(content: U) -> iced::widget::Button<'a, T, Theme, Renderer>
    where
        T: std::fmt::Debug + 'a,
        U: std::fmt::Display + 'a,
    {
        button(
            container(text(content).size(10).font(Font::with_name("DM Mono")))
                .width(iced::Length::Shrink)
                .style(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::from([6, 9]))
        .style(AtomStyleButton::RoundButton)
    }
}
