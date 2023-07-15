use crate::{
    font::{icon, CustomFont},
    messages::Message,
    style::{AtomStyleContainer, Theme},
};
use iced::{
    widget::{container, row, text},
    Element, Renderer,
};

pub fn view() -> Element<'static, Message, Renderer<Theme>> {
    container(
        row!()
            .padding(10)
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{eb08}', CustomFont::IcoFont).size(14))
                        .push(text("File Name")),
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
                        .push(icon('\u{e90b}', CustomFont::IcoFont).size(14))
                        .push(text("Size")),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::Fill),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{eed7}', CustomFont::IcoFont).size(14))
                        .push(text("Status")),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::FillPortion(1)),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{eff3}', CustomFont::IcoFont).size(14))
                        .push(text("Speed")),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::Fill),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{ee20}', CustomFont::IcoFont).size(18))
                        .push(text("ETA")),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::Fill),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::Center)
                        .push(icon('\u{ec45}', CustomFont::IcoFont).size(14))
                        .push(text("Added")),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Length::Fill),
            )
            .push(
                container(
                    row!()
                        .spacing(7)
                        .align_items(iced::Alignment::End)
                        .push(icon('\u{eecb}', CustomFont::IcoFont).size(14))
                        .push(text("Actions")),
                )
                .style(AtomStyleContainer::Transparent)
                .align_x(iced::alignment::Horizontal::Right)
                .width(iced::Length::Shrink),
            )
            .spacing(10)
            .align_items(iced::Alignment::Center),
    )
    .width(iced::Length::Fill)
    .style(AtomStyleContainer::ListHeaderContainer)
    .into()
}
