use crate::{
    font::{icon, CustomFont},
    messages::Message,
    style::{AtomStyleButton, AtomStyleContainer, Theme},
};
use iced::{
    widget::{column as col, container, row, text},
    Element, Padding, Renderer,
};
use iced_native::{
    widget::{button, vertical_space},
    Length,
};

pub fn view() -> Element<'static, Message, Renderer<Theme>> {
    let shortcuts = vec![
        ("Add new download", "N"),
        ("Open imports view", "I"),
        ("Resume all downloads", "R"),
        ("Pause all downloads", "P"),
        ("Delete all downloads", "D"),
        ("Show all downloads", "H"),
        ("Open settings", ","),
        ("Open keyboard shortcuts", "K"),
        ("Quit app", "Q"),
    ];

    let shortcuts: Vec<&[(&str, &str)]> = shortcuts.chunks(4).collect();

    let mut shortcuts_col = col!()
        .spacing(10)
        .padding(Padding::from([0, 10, 10, 10]))
        .width(iced::Length::Fill)
        .push(
            container(text("Keyboard Shortcuts"))
                .style(AtomStyleContainer::LogoContainer)
                .padding(Padding::from([10, 30, 10, 30])),
        )
        .push(vertical_space(10));

    for shortcut in shortcuts {
        let key_binding_col = col![]
            .width(Length::Fill)
            .align_items(iced_native::Alignment::Center)
            .push(
                shortcut.iter().fold(
                    row![]
                        .spacing(20)
                        .align_items(iced_native::Alignment::Center),
                    |row, shortcut| {
                        row.push(
                            container(
                                col![
                                    row![
                                        text("Action").size(16),
                                        row![
                                            button(
                                                icon(
                                                    if cfg!(target_os = "macos") {
                                                        '\u{f0633}'
                                                    } else {
                                                        '\u{f0634}'
                                                    },
                                                    CustomFont::Symbols,
                                                )
                                                .size(12),
                                            )
                                            .style(AtomStyleButton::ShortcutKeyButton)
                                            .padding(Padding::from([5, 10])),
                                            button(text(shortcut.1).size(14))
                                                .style(AtomStyleButton::ShortcutKeyButton)
                                                .padding(Padding::from([5, 10]))
                                        ]
                                        .align_items(iced_native::Alignment::Center)
                                        .spacing(5),
                                    ]
                                    .align_items(iced_native::Alignment::Center)
                                    .spacing(20),
                                    text(shortcut.0).size(14)
                                ]
                                .align_items(iced_native::Alignment::End)
                                .spacing(10),
                            )
                            .style(AtomStyleContainer::ListItemContainer)
                            .padding(Padding::from([20, 25])),
                        )
                    },
                ),
            );
        shortcuts_col = shortcuts_col.push(key_binding_col);
    }

    container(shortcuts_col)
        .style(AtomStyleContainer::ListContainer)
        .padding(Padding::from([0, 10, 10, 10]))
        .width(Length::Fill)
        .into()
}
