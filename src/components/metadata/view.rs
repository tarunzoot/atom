use super::AtomDownloadMetadata;
use crate::{
    elements::GuiElements,
    font::{file_type_icon, icon, CustomFont},
    messages::MetadataMessage,
    style::{container::AtomStyleContainer, AtomStyleText, Theme},
    utils::helpers::{get_file_type, get_formatted_time, get_relative_file_size},
};
use iced::{
    widget::{column as col, container, image, row, text, text_input, vertical_space},
    Element, Length, Padding, Renderer,
};
use std::{path::Path, time::Duration};

impl AtomDownloadMetadata {
    pub fn view(&self) -> Element<'static, MetadataMessage, Renderer<Theme>> {
        let file_path = Path::new(&self.file_path);
        let mut open_btn = GuiElements::primary_button(vec![
            icon('\u{ef13}', CustomFont::IcoFont).size(12),
            text("open").size(14),
        ])
        .padding(7)
        .width(Length::Fill);

        let mut delete_btn = GuiElements::primary_button(vec![
            icon('\u{ec53}', CustomFont::IcoFont).size(12),
            text("delete").size(14),
        ])
        .padding(7)
        .width(Length::Fill);

        if file_path.exists() {
            open_btn = open_btn.on_press(MetadataMessage::PreviewFile);
            delete_btn = delete_btn.on_press(MetadataMessage::DeleteFile);
        }

        let mut preview_column = col!()
            .width(Length::Fill)
            .height(Length::Fixed(200.0))
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .push(
                row!()
                    .width(Length::Fill)
                    .align_items(iced::Alignment::Center)
                    .push(
                        file_type_icon(&self.extension)
                            .size(20)
                            .horizontal_alignment(iced::alignment::Horizontal::Left)
                            .width(Length::Fill),
                    ), // .push(text(self.extension.to_uppercase())),
            );
        preview_column = match &self.extension[..] {
            "jpg" | "jpeg" | "png" | "gif" => preview_column.push(
                image(&self.file_path)
                    .height(Length::Fill)
                    .content_fit(iced::ContentFit::Cover),
            ),
            _ => preview_column.push(
                container(text("No preview available.").size(14))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .style(AtomStyleContainer::Transparent),
            ),
        };
        preview_column = preview_column.push(
            col!()
                .width(Length::Fill)
                .align_items(iced::Alignment::End)
                .push(text(&self.extension.to_uppercase()).size(14)),
        );

        let (time_created, time_accessed, time_modified) =
            if let Ok(metadata) = Path::new(&self.file_path).metadata() {
                let created = if let Ok(created) = metadata.created() {
                    let mut formatted_time = get_formatted_time(
                        created
                            .elapsed()
                            .unwrap_or_else(|_| Duration::from_secs(0))
                            .as_secs(),
                    );
                    formatted_time.push_str(" ago");
                    formatted_time
                } else {
                    String::default()
                };

                let accessed = if let Ok(accessed) = metadata.accessed() {
                    let mut formatted_time = get_formatted_time(
                        accessed
                            .elapsed()
                            .unwrap_or_else(|_| Duration::from_secs(0))
                            .as_secs(),
                    );
                    formatted_time.push_str(" ago");
                    formatted_time
                } else {
                    String::default()
                };

                let modified = if let Ok(modified) = metadata.modified() {
                    let mut formatted_time = get_formatted_time(
                        modified
                            .elapsed()
                            .unwrap_or_else(|_| Duration::from_secs(0))
                            .as_secs(),
                    );

                    formatted_time.push_str(" ago");
                    formatted_time
                } else {
                    String::default()
                };

                (created, accessed, modified)
            } else {
                (String::default(), String::default(), String::default())
            };

        let mut checksum_btn = GuiElements::round_button('\u{ec05}').padding(Padding::from([4, 6]));

        if !self.is_calculating_checksum {
            checksum_btn = checksum_btn.on_press(MetadataMessage::CalculateChecksum);
        }

        let checksum_col = col!()
            .spacing(5)
            .align_items(iced::Alignment::Start)
            .push(
                row!()
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(text("SHA256").width(Length::Fill))
                    .push(checksum_btn),
            )
            .push(
                row!().spacing(5).align_items(iced::Alignment::Center).push(
                    text_input(
                        "sha256 hash...",
                        if self.is_calculating_checksum {
                            "calculating..."
                        } else if let Some(checksum) = self.checksums.get(&self.url) {
                            checksum
                        } else {
                            ""
                        },
                    )
                    .on_input(|_| MetadataMessage::Ignore),
                ),
            );

        container(
            col!()
                .spacing(20)
                .push(
                    col!()
                        .spacing(5)
                        .push(
                            row!()
                                .width(Length::Fill)
                                .spacing(20)
                                .push(text("Resources").width(Length::Fill))
                                .push(
                                    GuiElements::round_button('\u{eee1}')
                                        .padding(Padding::from([2, 4]))
                                        .on_press(MetadataMessage::ClosePane),
                                ),
                        )
                        .push(
                            text(format!(
                                "{} • {}",
                                get_file_type(&self.extension),
                                get_relative_file_size(self.size)
                            ))
                            .style(AtomStyleText::Dimmed)
                            .size(12),
                        ),
                )
                .push(
                    col!()
                        .push(row!().spacing(10).push(text("URL").width(Length::Fill)))
                        .push(
                            text_input("", &self.url)
                                .size(14)
                                .on_input(|_| MetadataMessage::Ignore),
                        )
                        .push(vertical_space(5))
                        .push(checksum_col)
                        .spacing(5),
                )
                .push(
                    container(preview_column)
                        .padding(10)
                        .style(AtomStyleContainer::PreviewContainer),
                )
                .push(
                    col!()
                        .spacing(5)
                        .width(Length::Fill)
                        .push(text("Information"))
                        .push(
                            row!()
                                .width(Length::Fill)
                                .align_items(iced::Alignment::Center)
                                .push(
                                    text("Created")
                                        .style(AtomStyleText::Dimmed)
                                        .size(12)
                                        .width(Length::FillPortion(1)),
                                )
                                .push(text(time_created).size(10)),
                        )
                        .push(
                            row!()
                                .width(Length::Fill)
                                .align_items(iced::Alignment::Center)
                                .push(
                                    text("Modified")
                                        .style(AtomStyleText::Dimmed)
                                        .size(12)
                                        .width(Length::FillPortion(1)),
                                )
                                .push(text(time_modified).size(10)),
                        )
                        .push(
                            row!()
                                .width(Length::Fill)
                                .align_items(iced::Alignment::Center)
                                .push(
                                    text("Last Opened")
                                        .style(AtomStyleText::Dimmed)
                                        .size(12)
                                        .width(Length::FillPortion(1)),
                                )
                                .push(text(time_accessed).size(10)),
                        ),
                )
                .push(
                    row!()
                        .width(Length::Fill)
                        .spacing(5)
                        .push(open_btn)
                        .push(delete_btn),
                ),
        )
        .padding(15)
        .style(AtomStyleContainer::ListContainer)
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .into()
    }
}
