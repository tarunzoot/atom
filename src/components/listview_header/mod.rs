use crate::{
    components::settings::AtomSettings,
    icons,
    messages::Message,
    style::{container::AtomStyleContainer, AtomTheme},
    utils::helpers::ListViewColumns,
};
use iced::{
    widget::{container, row, text},
    Alignment, Element, Font, Length,
    Length::Fill,
};

pub fn get_list_view_header_column_length(view: ListViewColumns) -> Length {
    match view {
        ListViewColumns::FileName => Length::FillPortion(5),
        ListViewColumns::FileSize => Length::FillPortion(2),
        ListViewColumns::Status => Length::FillPortion(2),
        ListViewColumns::Speed => Length::FillPortion(2),
        ListViewColumns::Eta => Length::FillPortion(2),
        ListViewColumns::Added => Length::FillPortion(2),
        ListViewColumns::Actions => Length::Fixed(75.0),
        // _ => Length::Fill,
    }
}

pub fn view<'a>(settings: &AtomSettings, responsive: bool) -> Element<'a, Message, AtomTheme> {
    let icon_size = (settings.font_size as u16 / 8) * 7;
    let text_size = icon_size;
    let font = Font {
        family: iced::font::Family::Name("Lexend Deca"),
        weight: iced::font::Weight::Black,
        ..Default::default()
    };

    let file_name_container = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icons::file_alt().size(icon_size))
            .push(text("File Name").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(get_list_view_header_column_length(
        ListViewColumns::FileName,
    ));

    let file_size_container = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icons::file_size().size(icon_size - 2))
            .push(text("Size").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(get_list_view_header_column_length(
        ListViewColumns::FileSize,
    ));

    let eta_container = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icons::clock().size(icon_size))
            .push(text("E.T.A").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(get_list_view_header_column_length(ListViewColumns::Eta));

    let status_container = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icons::check_circled().size(icon_size))
            .push(text("Status").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(get_list_view_header_column_length(ListViewColumns::Status));

    let speed_con = container(
        row!()
            .spacing(7)
            .align_y(Alignment::Center)
            .push(icons::speedmeter().size(icon_size))
            .push(text("Speed").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(get_list_view_header_column_length(ListViewColumns::Speed));

    let added_con = container(
        row!()
            .spacing(7)
            .align_y(iced::Alignment::Center)
            .push(icons::calendar().size(icon_size))
            .push(text("Added").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Left)
    .width(get_list_view_header_column_length(ListViewColumns::Added));

    let actions_con = container(
        row!()
            .spacing(7)
            .align_y(iced::Alignment::Center)
            .push(icons::bullhorn().size(icon_size))
            .push(text("Actions").size(text_size).font(font)),
    )
    .class(AtomStyleContainer::Transparent)
    .align_x(iced::alignment::Horizontal::Right)
    .width(get_list_view_header_column_length(ListViewColumns::Actions));

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
