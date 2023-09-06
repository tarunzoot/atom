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
        message_confirm_positive: U,
        message_confirm_negative: U,
    ) -> Element<'a, U, Renderer<Theme>>
    where
        T: Into<Element<'a, U, Renderer<Theme>>>,
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
                    .push(
                        row!()
                            .spacing(10)
                            .align_items(Alignment::Center)
                            .push(
                                GuiElements::primary_button(vec![
                                    icon('\u{ec53}', CustomFont::IcoFont),
                                    text("delete"),
                                ])
                                .width(Length::Fixed(170.0))
                                .on_press(message_confirm_positive),
                            )
                            .push(
                                GuiElements::primary_button(vec![
                                    icon('\u{eede}', CustomFont::IcoFont),
                                    text("cancel"),
                                ])
                                .width(Length::Fixed(170.0))
                                .on_press(message_confirm_negative.clone()),
                            ),
                    )
                    .padding(Padding::from([0, 20, 20, 20])),
            )
            .style(AtomStyleContainer::ToolTipContainer)
            .padding(Padding::from([0, 30]))
            .width(Length::Shrink),
        )
        .on_blur(message_confirm_negative)
        .into()
    }

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

    pub fn round_text_button<'a, T, U>(content: U) -> iced::widget::Button<'a, T, Renderer<Theme>>
    where
        T: std::fmt::Debug + 'a,
        U: std::fmt::Display + 'a,
    {
        button(
            container(text(content).size(10).font(Font::with_name("DM Mono")))
                .width(iced::Length::Fill)
                .style(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::from([6, 9]))
        .style(AtomStyleButton::RoundButton)
    }
}
