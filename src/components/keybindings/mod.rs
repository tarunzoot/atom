use crate::{
    components::settings::AtomSettings,
    elements::GuiElements,
    font::{icon, CustomFont, JOSEFIN},
    messages::Message,
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomStyleText, AtomTheme},
};
use iced::{
    widget::{button, column as col, container, horizontal_space, row, text, vertical_space},
    Alignment, Element,
    Length::{Fill, FillPortion, Fixed, Shrink},
    Padding,
};

pub fn view<'a>(settings: &AtomSettings) -> Element<'a, Message, AtomTheme> {
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
        ("cycle through themes", "T", '\u{ec88}'),
        ("toggle list layout", "L", '\u{ef72}'),
        ("clear search", "G", '\u{eedd}'),
        ("decrease scaling by 0.10", "-", '\u{ef9a}'),
        ("increase scaling by 0.10", "=", '\u{efc2}'),
        ("reset scaling", "0", '\u{ec7f}'),
        ("quit app", "Q", '\u{ef1d}'),
    ];

    let main_col = col![GuiElements::panel_title("Keyboard Shortcuts").into()]
        .spacing(10)
        .padding(Padding::new(10.0).top(0))
        .width(iced::Length::Fill)
        .push(vertical_space().height(10));

    let text_size = 12;
    let chunk_len = 3;
    let mut shortcuts_col = col![].spacing(20).align_x(Alignment::Center);
    shortcuts_col = shortcuts_col.push(shortcuts.chunks(chunk_len).fold(
        col![].spacing(20).align_x(Alignment::Center),
        |column, chunk| {
            let mut row = chunk.iter().fold(
                row![]
                    .spacing(20)
                    .align_y(iced::Alignment::Center)
                    .width(Shrink),
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
                                        icon(shortcut.2, CustomFont::IcoFont)
                                            .class(AtomStyleText::Dimmed)
                                            .size(text_size - 2),
                                        text(shortcut.0.to_string())
                                            .font(JOSEFIN)
                                            .size(text_size)
                                            .width(Fill),
                                    ]
                                    .spacing(5)
                                    .align_y(Alignment::Center)
                                ]
                                .spacing(20)
                                .align_x(Alignment::Start)
                                .width(FillPortion(3)),
                                col![container(
                                    vertical_space().height(Fixed(30.0)).width(Fixed(1.0)),
                                )
                                .class(AtomStyleContainer::ListItemContainer)
                                .width(Fixed(1.0))]
                                .align_x(iced::Alignment::Center)
                                .width(Shrink),
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
                                        .class(AtomStyleButton::ShortcutKeyButton)
                                        .padding(Padding::from([5, 10])),
                                        button(text(shortcut.1).size(12))
                                            .class(AtomStyleButton::ShortcutKeyButton)
                                            .padding(Padding::from([5, 10]))
                                    ]
                                    .spacing(5)
                                    .align_y(Alignment::Center)
                                ]
                                .spacing(20)
                                .align_x(Alignment::End)
                                .width(FillPortion(3))
                            ]
                            .spacing(5)
                            .align_y(Alignment::Center),
                        )
                        .padding(20)
                        .class(AtomStyleContainer::ListItemContainer),
                    )
                },
            );

            let chunk_size = chunk.len();

            if chunk_size < chunk_len {
                for _ in 0..(chunk_len - chunk_size) {
                    row = row.push(horizontal_space().width(Fill));
                }
            }
            column.push(row)
        },
    ));

    container(main_col.push(GuiElements::scrollbar(
        shortcuts_col,
        settings.scrollbars_visible,
    )))
    .class(AtomStyleContainer::ListContainer)
    .padding(Padding::new(10.0).top(0))
    .height(Fill)
    .width(Fill)
    .into()
}
