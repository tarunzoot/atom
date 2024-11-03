use crate::{
    components::settings::AtomSettings,
    font::{icon, CustomFont},
    messages::TitleBarMessage,
    style::{
        button::AtomStyleButton, container::AtomStyleContainer, input::AtomStyleInput, AtomTheme,
    },
};
use iced::{
    widget::{button, column as col, container, row, text, text_input},
    Alignment, Element, Font,
    Length::{Fill, Fixed},
    Padding, Renderer,
};

#[derive(Debug, Default)]
pub struct AtomTitleBar {
    pub search_text: String,
}

impl AtomTitleBar {
    pub fn view(&self, settings: &AtomSettings) -> Element<TitleBarMessage, AtomTheme, Renderer> {
        container(
            row!()
                .spacing(20)
                .height(Fill)
                .push(
                    row!()
                        .spacing(20)
                        .align_y(Alignment::Center)
                        .push(
                            container(text(" "))
                                .padding(Padding::from([5, 0]))
                                .width(Fixed(3.0))
                                .class(AtomStyleContainer::MenuBarActiveContainer),
                        )
                        .push(
                            row!()
                                .align_y(Alignment::Center)
                                .spacing(10)
                                .push(icon('\u{ead8}', CustomFont::IcoFont).size(20))
                                .push(
                                    text("A.T.O.M")
                                        // text("a.t.o.m")
                                        .font(Font {
                                            family: iced::font::Family::Name("Lexend Deca"),
                                            weight: iced::font::Weight::Black,
                                            ..Default::default()
                                        })
                                        .size(22.0),
                                ),
                        ),
                )
                .push(
                    container(
                        col!()
                            .push(
                                row!()
                                    .spacing(20)
                                    .push(
                                        container(
                                            text_input("search downloads...", &self.search_text)
                                                .id(iced::widget::text_input::Id::new("search"))
                                                .on_input(TitleBarMessage::SearchDownload)
                                                .padding(Padding::new(20.0).top(8).bottom(8))
                                                .class(AtomStyleInput::Search),
                                        )
                                        .class(AtomStyleContainer::HeaderContainer)
                                        .center_x(Fill)
                                        .center_y(Fill)
                                        .width(Fill),
                                    )
                                    .push(
                                        container(
                                            row!()
                                                .push(
                                                    button(icon('\u{ef9a}', CustomFont::IcoFont))
                                                        .padding(14)
                                                        .class(AtomStyleButton::HeaderButtons)
                                                        .on_press(TitleBarMessage::AppMinimize),
                                                )
                                                .push(
                                                    button(icon('\u{ef52}', CustomFont::IcoFont))
                                                        .padding(14)
                                                        .class(AtomStyleButton::HeaderButtons)
                                                        .on_press(TitleBarMessage::AppMaximize),
                                                )
                                                .push(
                                                    button(icon('\u{eee1}', CustomFont::IcoFont))
                                                        .padding(14)
                                                        .class(AtomStyleButton::HeaderButtons)
                                                        .on_press(if !settings.minimize_to_tray {
                                                            TitleBarMessage::AppExit
                                                        } else {
                                                            TitleBarMessage::AppHide
                                                        }),
                                                ),
                                        )
                                        .class(AtomStyleContainer::HeaderButtonsContainer),
                                    )
                                    .align_y(Alignment::Center),
                            )
                            .width(Fill)
                            .align_x(Alignment::Center),
                    )
                    .center_x(Fill)
                    .center_y(Fill)
                    .class(AtomStyleContainer::HeaderContainer)
                    .width(Fill),
                )
                .align_y(iced::Alignment::Center)
                .width(Fill),
        )
        .class(AtomStyleContainer::HeaderContainer)
        .center_y(Fill)
        .center_x(Fill)
        // .padding(Padding::from([5, 15, 5, 15]))
        .width(Fill)
        .height(Fixed(50.0))
        .into()
    }
}
