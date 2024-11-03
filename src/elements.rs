use crate::{
    components::modal::Modal,
    font::{icon, CustomFont},
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomTheme},
};
use iced::{
    widget::{button, column as col, container, row, text, text_input, Button},
    Alignment, Element, Font,
    Length::Shrink,
    Padding,
};

pub struct GuiElements;

impl GuiElements {
    pub fn modal<'a, T, U>(
        base: T,
        body: impl Into<Element<'a, U, AtomTheme>>,
        actions: impl Into<Element<'a, U, AtomTheme>>,
        on_blur: U,
    ) -> Element<'a, U, AtomTheme>
    where
        T: Into<Element<'a, U, AtomTheme>>,
        U: Clone + std::fmt::Debug + 'static,
    {
        Modal::new(
            base,
            container(
                col!()
                    .align_x(Alignment::Center)
                    .spacing(30)
                    .push(GuiElements::panel_title("Confirmation Dialog"))
                    .push(
                        row!().align_y(Alignment::Center).spacing(5).push(body),
                        // .push(text(confirmation_text).size(24)),
                    )
                    .push(actions)
                    .padding(Padding::new(20.0).top(0)),
            )
            .class(AtomStyleContainer::ToolTipContainer)
            .padding(Padding::from([0, 30]))
            .width(Shrink),
        )
        .on_blur(on_blur)
        .into()
    }

    pub fn primary_button<'a, T>(
        contents: Vec<impl Into<Element<'a, T, AtomTheme>>>,
    ) -> Button<'a, T, AtomTheme>
    where
        T: std::fmt::Debug + 'a,
    {
        let content_row = contents
            .into_iter()
            .fold(row!(), |row, child| row.push(child))
            .align_y(Alignment::Center)
            .spacing(5);

        button(
            container(content_row)
                .width(Shrink)
                .center(Shrink)
                .class(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::new(15.0).top(7).bottom(7))
    }

    pub fn round_button<'a, T>(icon_code: char) -> Button<'a, T, AtomTheme>
    where
        T: std::fmt::Debug + 'a,
    {
        button(
            container(icon(icon_code, CustomFont::IcoFont).size(12))
                .width(Shrink)
                .class(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::new(8.0).top(6).bottom(6))
        .class(AtomStyleButton::RoundButton)
    }

    pub fn round_text_button<'a, T, U>(content: U) -> Button<'a, T, AtomTheme>
    where
        T: std::fmt::Debug + 'a,
        U: std::fmt::Display + 'a + iced::advanced::text::IntoFragment<'a>,
    {
        button(
            container(text(content).size(10).font(Font::with_name("DM Mono")))
                .width(Shrink)
                .center(Shrink)
                .class(AtomStyleContainer::ButtonContainer),
        )
        .padding(Padding::from([6, 9]))
        .class(AtomStyleButton::RoundButton)
    }

    pub fn panel_title<M>(panel_title: &str) -> impl Into<Element<M, AtomTheme>>
    where
        M: Clone + std::fmt::Debug + 'static,
    {
        container(text(panel_title))
            .class(AtomStyleContainer::LogoContainer)
            .padding(Padding::new(30.0).top(10).bottom(10))
    }

    pub fn text_input_icon<Font>(icon: char, font: Font, text_size: u16) -> text_input::Icon<Font> {
        text_input::Icon {
            font,
            code_point: icon,
            size: Some(iced::Pixels(text_size.into())),
            spacing: 5.0,
            side: text_input::Side::Right,
        }
    }
}
