use crate::{
    components::modal::Modal,
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomTheme},
};
use iced::{
    widget::{
        button, column as col, container, row, scrollable, scrollable::Scrollbar, text, text_input,
        toggler, tooltip, vertical_space, Button, Scrollable, Text, Toggler, Tooltip,
    },
    Alignment, Element, Font,
    Length::{Fill, Shrink},
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

    pub fn round_button<'a, T>(icon: Text<'a, AtomTheme>) -> Button<'a, T, AtomTheme>
    where
        T: std::fmt::Debug + 'a,
    {
        button(
            container(icon.size(12))
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

    pub fn horizontal_separator<'a, M>() -> impl Into<Element<'a, M, AtomTheme>>
    where
        M: Clone + std::fmt::Debug + 'static,
    {
        col![container(vertical_space().height(1))
            .center(Fill)
            .height(1)
            .class(AtomStyleContainer::ListContainer)]
        .height(Shrink)
        .padding(15)
        .align_x(Alignment::Center)
    }

    #[allow(dead_code)]
    pub fn vertical_separator<'a, M>() -> impl Into<Element<'a, M, AtomTheme>>
    where
        M: Clone + std::fmt::Debug + 'static,
    {
        col![container(vertical_space().height(30).width(1))
            .class(AtomStyleContainer::ListItemContainer)
            .width(1)]
        .align_x(iced::Alignment::Center)
        .width(Shrink)
    }

    pub fn tooltip_top<'a, E, M>(content: E, tooltip_text: &'a str) -> Tooltip<'a, M, AtomTheme>
    where
        E: Into<Element<'a, M, AtomTheme>>,
        M: Clone + std::fmt::Debug + 'static,
    {
        tooltip(content, text(tooltip_text).size(12), tooltip::Position::Top)
            .gap(5)
            .padding(10)
            .class(AtomStyleContainer::ToolTipContainer)
    }

    pub fn tooltip_bottom<'a, E, M>(content: E, tooltip_text: &'a str) -> Tooltip<'a, M, AtomTheme>
    where
        E: Into<Element<'a, M, AtomTheme>>,
        M: Clone + std::fmt::Debug + 'static,
    {
        tooltip(
            content,
            text(tooltip_text).size(12),
            tooltip::Position::Bottom,
        )
        .gap(5)
        .padding(10)
        .class(AtomStyleContainer::ToolTipContainer)
    }

    pub fn toggle<'a, M>(
        is_checked: bool,
        message: impl Fn(bool) -> M + 'a,
        label: &'a str,
    ) -> Toggler<'a, M, AtomTheme>
    where
        M: Clone + std::fmt::Debug + 'static,
    {
        toggler(is_checked)
            .label(label)
            .on_toggle(message)
            .spacing(10)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(Shrink)
    }

    pub fn scrollbar<'a, E, M>(content: E, visible: bool) -> Scrollable<'a, M, AtomTheme>
    where
        E: Into<Element<'a, M, AtomTheme>>,
        M: Clone + std::fmt::Debug + 'static,
    {
        let width = if visible { 6 } else { 0 };

        scrollable(content).direction(scrollable::Direction::Vertical(
            Scrollbar::new()
                .margin(0)
                .scroller_width(width)
                .width(width),
        ))
    }
}
