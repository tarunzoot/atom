use crate::{
    components::settings::AtomSettings,
    font::{icon, CustomFont},
    messages::TitleBarMessage,
    style::{button::AtomStyleButton, container::AtomStyleContainer, input::AtomStyleInput, Theme},
};
use iced::{
    widget::{button, column as col, container, row, text, text_input},
    Element, Font, Length, Padding, Renderer,
};

#[derive(Debug, Default)]
pub struct AtomTitleBar {
    pub search_text: String,
}

impl AtomTitleBar {
    pub fn view(&self, settings: &AtomSettings) -> Element<TitleBarMessage, Renderer<Theme>> {
        container(
            row!()
                .spacing(20)
                .height(iced::Length::Fill)
                .push(
                    row!()
                        .spacing(20)
                        .align_items(iced::Alignment::Center)
                        .push(
                            container(text(" "))
                                .padding(Padding::from([5, 0]))
                                .width(Length::Fixed(3.0))
                                .style(AtomStyleContainer::MenuBarActiveContainer),
                        )
                        .push(
                            row!()
                                .align_items(iced::Alignment::Center)
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
                                                .padding(Padding::from([8, 20, 8, 20]))
                                                .style(AtomStyleInput::Search),
                                        )
                                        .style(AtomStyleContainer::HeaderContainer)
                                        .center_x()
                                        .center_y()
                                        .width(iced::Length::Fill),
                                    )
                                    .push(
                                        container(
                                            row!()
                                                .push(
                                                    button(icon('\u{ef9a}', CustomFont::IcoFont))
                                                        .padding(15)
                                                        .style(AtomStyleButton::HeaderButtons)
                                                        .on_press(TitleBarMessage::AppMinimize),
                                                )
                                                .push(
                                                    button(icon('\u{ef52}', CustomFont::IcoFont))
                                                        .padding(15)
                                                        .style(AtomStyleButton::HeaderButtons)
                                                        .on_press(TitleBarMessage::AppMaximize),
                                                )
                                                .push(
                                                    button(icon('\u{eee1}', CustomFont::IcoFont))
                                                        .padding(15)
                                                        .style(AtomStyleButton::HeaderButtons)
                                                        .on_press(
                                                            if settings.quit_action_closes_app {
                                                                TitleBarMessage::AppExit
                                                            } else {
                                                                TitleBarMessage::AppHide
                                                            },
                                                        ),
                                                ),
                                        )
                                        .style(AtomStyleContainer::HeaderButtonsContainer),
                                    )
                                    .align_items(iced::Alignment::Center),
                            )
                            .width(iced::Length::Fill)
                            .align_items(iced::Alignment::Center),
                    )
                    .center_x()
                    .center_y()
                    .style(AtomStyleContainer::HeaderContainer)
                    .width(iced::Length::Fill),
                )
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill),
        )
        .style(AtomStyleContainer::HeaderContainer)
        .center_y()
        .center_x()
        // .padding(Padding::from([5, 15, 5, 15]))
        .width(iced::Length::Fill)
        .height(iced::Length::Fixed(50.0))
        .into()
    }
}
