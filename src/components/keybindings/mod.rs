use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::Message,
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomStyleText, Theme},
};
use iced::{
    widget::{button, horizontal_space, scrollable, vertical_space},
    Length,
};
use iced::{
    widget::{column as col, container, row, text},
    Element, Padding, Renderer,
};

pub fn view() -> Element<'static, Message, Theme, Renderer> {
    let shortcuts = vec![
        ("add new download", "N", '\u{efc0}'),
        ("open imports view", "I", '\u{ec84}'),
        ("resume all downloads", "R", '\u{eca8}'),
        ("pause all downloads", "P", '\u{eca5}'),
        ("delete all downloads", "D", '\u{edec}'),
        ("show all downloads", "H", '\u{e944}'),
        ("open settings", ",", '\u{ec83}'),
        ("open keyboard shortcuts", "K", '\u{ea54}'),
        ("search downloads", "F", '\u{ed11}'),
        ("quit app", "Q", '\u{eedd}'),
    ];

    let main_col = col![GuiElements::panel_title("Keyboard Shortcuts").into()]
        .spacing(10)
        .padding(Padding::from([0, 10, 10, 10]))
        .width(iced::Length::Fill)
        .push(vertical_space().height(10));

    let text_size = 12;
    let chunk_len = 3;
    let mut shortcuts_col = col![].spacing(20).align_items(iced::Alignment::Center);
    shortcuts_col = shortcuts_col.push(shortcuts.chunks(chunk_len).fold(
        col![].spacing(20).align_items(iced::Alignment::Center),
        |column, chunk| {
            let mut row = chunk.iter().fold(
                row![]
                    .spacing(20)
                    .align_items(iced::Alignment::Center)
                    .width(Length::Shrink),
                |row, shortcut| {
                    row.push(
                        container(
                            row![
                                col![
                                    text("Command").font(iced::Font {
                                        family: iced::font::Family::Name("Lexend Deca"),
                                        weight: iced::font::Weight::Black,
                                        ..Default::default()
                                    }),
                                    row![
                                        icon('\u{ee57}', CustomFont::Symbols)
                                            .style(AtomStyleText::Dimmed)
                                            .size(text_size - 2),
                                        text(shortcut.0.to_string())
                                            .size(text_size)
                                            .width(iced::Length::Fill),
                                    ]
                                    .spacing(5)
                                    .align_items(iced::Alignment::Center)
                                ]
                                .spacing(20)
                                .align_items(iced::Alignment::Start)
                                .width(Length::FillPortion(3)),
                                col![container(
                                    vertical_space()
                                        .height(Length::Fixed(30.0))
                                        .width(Length::Fixed(1.0)),
                                )
                                .style(AtomStyleContainer::ListItemContainer)
                                .width(Length::Fixed(1.0))]
                                .align_items(iced::Alignment::Center)
                                .width(Length::Shrink),
                                col![
                                    text("Keybinding").font(iced::Font {
                                        family: iced::font::Family::Name("Lexend Deca"),
                                        weight: iced::font::Weight::Black,
                                        ..Default::default()
                                    }),
                                    row![
                                        button(
                                            icon(
                                                if cfg!(target_os = "macos") {
                                                    '\u{f0633}'
                                                } else {
                                                    '\u{f0634}'
                                                },
                                                CustomFont::Symbols
                                            )
                                            .size(12)
                                        )
                                        .style(AtomStyleButton::ShortcutKeyButton)
                                        .padding(Padding::from([5, 10])),
                                        button(text(shortcut.1).size(12))
                                            .style(AtomStyleButton::ShortcutKeyButton)
                                            .padding(Padding::from([5, 10]))
                                    ]
                                    .spacing(5)
                                    .align_items(iced::Alignment::Center)
                                ]
                                .spacing(20)
                                .align_items(iced::Alignment::End)
                                .width(Length::FillPortion(3))
                            ]
                            .spacing(5)
                            .align_items(iced::Alignment::Center),
                        )
                        .padding(20)
                        .style(AtomStyleContainer::ListItemContainer),
                    )
                },
            );

            let chunk_size = chunk.len();

            if chunk_size < chunk_len {
                for _ in 0..(chunk_len - chunk_size) {
                    row = row.push(horizontal_space().width(Length::Fill));
                }
            }
            column.push(row)
        },
    ));

    container(main_col.push(scrollable(shortcuts_col)))
        .style(AtomStyleContainer::ListContainer)
        .padding(Padding::from([0, 10, 10, 10]))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}
