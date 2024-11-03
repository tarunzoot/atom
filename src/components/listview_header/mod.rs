use crate::{
    font::{icon, CustomFont},
    messages::Message,
    style::{container::AtomStyleContainer, AtomTheme},
};
use iced::{
    widget::{container, row, text},
    Alignment, Element, Font,
    Length::{Fill, FillPortion, Fixed},
};

pub fn view<'a>(responsive: bool) -> Element<'a, Message, AtomTheme> {
    let icon_size = 14;
    let text_size = 14;
    let font = Font {
        family: iced::font::Family::Name("Lexend Deca"),
        weight: iced::font::Weight::Black,
        ..Default::default()
    };

    let file_name_container = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icon('\u{eb08}', CustomFont::IcoFont).size(icon_size))
            .push(text("File Name").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(FillPortion(5));

    let file_size_container = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icon('\u{e90b}', CustomFont::IcoFont).size(icon_size - 2))
            .push(text("Size").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(FillPortion(3));

    let eta_container = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icon('\u{f022}', CustomFont::IcoFont).size(icon_size))
            .push(text("E.T.A").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(FillPortion(2));

    let status_container = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icon('\u{eed7}', CustomFont::IcoFont).size(icon_size))
            .push(text("Status").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(FillPortion(2));

    let speed_con = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icon('\u{eff3}', CustomFont::IcoFont).size(icon_size))
            .push(text("Speed").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(FillPortion(2));

    let added_con = container(
        row!()
            .spacing(7)
            .align_y(iced::Alignment::Center)
            .push(icon('\u{ec45}', CustomFont::IcoFont).size(icon_size))
            .push(text("Added").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(FillPortion(2));

    let actions_con = container(
        row!()
            .spacing(7)
            .align_y(iced::Alignment::Center)
            .push(icon('\u{eecb}', CustomFont::IcoFont).size(icon_size))
            .push(text("Actions").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(Fixed(75.0));

    let mut main_row = row!()
        .padding(10)
        .spacing(15)
        .align_y(Alignment::Center)
        .push(file_name_container)
        .push(file_size_container)
        .push(status_container)
        .push(speed_con)
        .push(eta_container);

    if !responsive {
        main_row = main_row.push(added_con);
    }
    main_row = main_row.push(actions_con);

    let main_container = container(main_row)
        .width(Fill)
        .class(AtomStyleContainer::ListHeaderContainer);

    main_container.into()
}
