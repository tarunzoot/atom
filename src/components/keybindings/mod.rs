use crate::{
    components::settings::AtomSettings,
    elements::GuiElements,
    font::JOSEFIN,
    icons,
    messages::Message,
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomStyleText, AtomTheme},
};
use iced::{
    widget::{button, column as col, container, horizontal_space, row, text, vertical_space, Text},
    Alignment, Element,
    Length::{Fill, FillPortion, Fixed, Shrink},
    Padding,
};

pub fn view<'a>(settings: &AtomSettings) -> Element<'a, Message, AtomTheme> {
    let shortcuts: Vec<(&'static str, &'static str, fn() -> Text<'a, AtomTheme>)> = vec![
        ("add new download", "N", icons::plus_circle),
        ("open imports view", "I", icons::social_link),
        ("resume all downloads", "R", icons::play_alt),
        ("pause all downloads", "P", icons::pause_alt),
        ("delete all downloads", "D", icons::recycle_bin),
        ("show all downloads", "H", icons::overview),
        ("open settings", ",", icons::settings),
        ("open keyboard shortcuts", "K", icons::keyboard),
        ("search downloads", "F", icons::search),
        ("cycle through themes", "T", icons::theme),
        ("toggle list layout", "L", icons::list),
        ("clear search", "G", icons::close_circled),
        ("decrease scaling by 0.10", "-", icons::minus),
        ("increase scaling by 0.10", "=", icons::plus),
        ("reset scaling", "0", icons::reply),
        ("quit app", "Q", icons::exit),
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
            let mut row = chunk.into_iter().fold(
                row![].spacing(20).align_y(Alignment::Center).width(Shrink),
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
                                        shortcut.2()
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
                                            if cfg!(target_os = "macos") {
                                                icons::command()
                                            } else {
                                                icons::ctrl()
                                            }
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
