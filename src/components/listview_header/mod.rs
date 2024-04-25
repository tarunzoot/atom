use crate::{
    font::{icon, CustomFont},
    messages::Message,
    style::{container::AtomStyleContainer, Theme},
};
use iced::{
    widget::{container, row, text},
    Element, Font, Renderer,
};

pub fn view() -> Element<'static, Message, Theme, Renderer> {
    let icon_size = 14;
    let text_size = 14;
    let font = Font {
        family: iced::font::Family::Name("Lexend Deca"),
        weight: iced::font::Weight::Black,
        ..Default::default()
    };
    container(
        row!()
            .padding(10)
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{eb08}', CustomFont::IcoFont).size(icon_size))
                        .push(text("File Name").size(text_size).font(font)),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Left)
                .width(iced::Length::FillPortion(2)),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{e90b}', CustomFont::IcoFont).size(icon_size - 2))
                        .push(text("Size").size(text_size).font(font)),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::Fixed(130.0)),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{eed7}', CustomFont::IcoFont).size(icon_size))
                        .push(text("Status").size(text_size).font(font)),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::Fixed(180.0)),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{eff3}', CustomFont::IcoFont).size(icon_size))
                        .push(text("Speed").size(text_size).font(font)),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::Fixed(100.0)),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{f022}', CustomFont::IcoFont).size(icon_size))
                        .push(text("ETA").size(text_size).font(font)),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Left)
                .width(iced::Length::Fixed(100.0)),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{ec45}', CustomFont::IcoFont).size(icon_size))
                        .push(text("Added").size(text_size).font(font)),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Left)
                .width(iced::Length::Fixed(80.0)),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{eecb}', CustomFont::IcoFont).size(icon_size))
                        .push(text("Actions").size(text_size).font(font)),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::Fixed(75.0)),
                // .width(iced::Length::Fixed(95.0)),
            )
            .spacing(10)
            .align_items(iced::Alignment::Center),
    )
    .width(iced::Length::Fill)
    .style(AtomStyleContainer::ListHeaderContainer)
    .into()
}
