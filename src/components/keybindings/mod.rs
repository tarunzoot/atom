use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::Message,
    style::{button::AtomStyleButton, container::AtomStyleContainer, Theme},
};
use iced::{
    widget::{button, horizontal_space, vertical_space},
    Length,
};
use iced::{
    widget::{column as col, container, row, text},
    Element, Padding, Renderer,
};

pub fn view() -> Element<'static, Message, Theme, Renderer> {
    let shortcuts = vec![
        ("Add new download", "N"),
        ("Open imports view", "I"),
        ("Resume all downloads", "R"),
        ("Pause all downloads", "P"),
        ("Delete all downloads", "D"),
        ("Show all downloads", "H"),
        ("Open settings", ","),
        ("Open keyboard shortcuts", "K"),
        ("Search Downloads", "F"),
        ("Quit app", "Q"),
    ];

    let shortcuts: Vec<&[(&str, &str)]> = shortcuts.chunks(4).collect();

    let mut shortcuts_col = col!()
        .spacing(20)
        .padding(Padding::from([0, 10, 10, 10]))
        .width(iced::Length::Fill)
        .push(GuiElements::panel_title("Keyboard Shortcuts"))
        .push(vertical_space().height(10));

    for shortcut in shortcuts {
        let number_of_shortcuts = shortcut.len();

        let mut shortcut_col = shortcut.iter().fold(
            row![].spacing(20).align_items(iced::Alignment::Center),
            |row, shortcut| {
                row.push(
                    container(
                        col![
                            row![
                                text("Action").size(16).width(Length::Fill),
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
                                        .size(14),
                                    )
                                    .style(AtomStyleButton::ShortcutKeyButton)
                                    .padding(Padding::from([5, 10])),
                                    button(text(shortcut.1).size(14))
                                        .style(AtomStyleButton::ShortcutKeyButton)
                                        .padding(Padding::from([5, 10]))
                                ]
                                .align_items(iced::Alignment::Center)
                                .spacing(5),
                            ]
                            .align_items(iced::Alignment::Center)
                            .spacing(20),
                            text(shortcut.0).size(12)
                        ]
                        .align_items(iced::Alignment::End)
                        .spacing(20),
                    )
                    .width(Length::Fill)
                    .style(AtomStyleContainer::ListItemContainer)
                    .padding(Padding::from([20, 25])),
                )
            },
        );

        if number_of_shortcuts != 4 {
            for _ in 0..4 - number_of_shortcuts {
                shortcut_col = shortcut_col.push(horizontal_space().width(Length::Fill));
            }
        }

        let key_binding_col = col![]
            .width(Length::Fill)
            .align_items(iced::Alignment::Start)
            .push(shortcut_col);

        shortcuts_col = shortcuts_col.push(key_binding_col);
    }

    container(shortcuts_col)
        .style(AtomStyleContainer::ListContainer)
        .padding(Padding::from([0, 10, 10, 10]))
        .width(Length::Fill)
        .into()
}
