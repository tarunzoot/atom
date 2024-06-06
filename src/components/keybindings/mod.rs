use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::Message,
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomStyleText, Theme},
};
use iced::{
    widget::{button, scrollable, vertical_space},
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

    let main_col = col![GuiElements::panel_title("Keyboard Shortcuts").into()]
        .spacing(10)
        .padding(Padding::from([0, 10, 10, 10]))
        .width(iced::Length::Fill)
        .push(vertical_space().height(10));

    let text_size = 12;
    let shortcuts_col = shortcuts.iter().fold(
        col!().spacing(2).align_items(iced::Alignment::Center),
        |column, shortcut| {
            column.push(
                container(
                    row![
                        icon('\u{ee57}', CustomFont::Symbols)
                            .style(AtomStyleText::Dimmed)
                            .size(text_size),
                        text(shortcut.0.to_string())
                            .size(text_size)
                            .width(iced::Length::Fill),
                        col![container(
                            vertical_space()
                                .height(Length::Fixed(30.0))
                                .width(Length::Fixed(1.0)),
                        )
                        .style(AtomStyleContainer::ListItemContainer)
                        .width(Length::Fixed(2.0))]
                        .align_items(iced::Alignment::Center)
                        .width(Length::Shrink),
                        button("ctrl")
                            .style(AtomStyleButton::ShortcutKeyButton)
                            .padding(Padding::from([5, 10])),
                        col![container(
                            vertical_space()
                                .height(Length::Fixed(30.0))
                                .width(Length::Fixed(1.0)),
                        )
                        .style(AtomStyleContainer::ListItemContainer)
                        .width(Length::Fixed(2.0))]
                        .align_items(iced::Alignment::Center)
                        .width(Length::Shrink),
                        button(text(shortcut.1).size(14))
                            .style(AtomStyleButton::ShortcutKeyButton)
                            .padding(Padding::from([5, 10]))
                    ]
                    .padding(Padding::from([5, 10]))
                    .spacing(10)
                    .align_items(iced::Alignment::Center),
                )
                .style(AtomStyleContainer::ListItemContainer),
            )
        },
    );

    container(main_col.push(scrollable(shortcuts_col)))
        .style(AtomStyleContainer::ListContainer)
        .padding(Padding::from([0, 10, 10, 10]))
        .width(Length::Fill)
        .into()
}
