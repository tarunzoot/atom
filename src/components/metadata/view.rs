use super::AtomDownloadMetadata;
use crate::{
    components::settings::AtomSettings,
    elements::GuiElements,
    font::file_type_icon,
    icons,
    messages::MetadataMessage,
    style::{container::AtomStyleContainer, AtomStyleText, AtomTheme},
    utils::helpers::{
        get_file_type, get_formatted_time, get_relative_file_size, METADATA_PANEL_WIDTH,
    },
};
use iced::{
    widget::{column as col, container, image, row, text, text_input, vertical_space},
    Alignment, Element,
    Length::{Fill, FillPortion},
    Padding,
};
use std::{path::Path, time::Duration};

impl AtomDownloadMetadata {
    pub fn view(&self, settings: &AtomSettings) -> Element<MetadataMessage, AtomTheme> {
        let file_path = Path::new(&self.file_path);
        let mut open_btn = GuiElements::primary_button(vec![
            icons::envelope_open().size(12),
            text("open").size(14),
        ])
        .padding(7)
        .width(Fill);

        let mut delete_btn = GuiElements::primary_button(vec![
            icons::trash_bin_open().size(12),
            text("delete").size(14),
        ])
        .padding(7)
        .width(Fill);

        if file_path.exists() {
            open_btn = open_btn.on_press(MetadataMessage::PreviewFile);
            delete_btn = delete_btn.on_press(MetadataMessage::DeleteFile);
        }

        let mut preview_column = col!()
            .width(Fill)
            .height(Fill)
            .align_x(Alignment::Center)
            .spacing(10)
            .push(
                row!().width(Fill).align_y(Alignment::Center).push(
                    file_type_icon(&self.extension)
                        .size(20)
                        .align_x(iced::alignment::Horizontal::Left)
                        .width(Fill),
                ),
            );
        preview_column = match (&self.extension[..], file_path.exists()) {
            ("jpg" | "jpeg" | "png" | "gif" | "JPG" | "JPEG" | "PNG" | "GIF", true) => {
                preview_column.push(
                    image(&self.file_path)
                        .height(Fill)
                        .content_fit(iced::ContentFit::Cover),
                )
            }
            _ => preview_column.push(
                container(text("No preview available.").size(14))
                    .width(Fill)
                    .height(Fill)
                    .center_x(Fill)
                    .center_y(Fill)
                    .class(AtomStyleContainer::Transparent),
            ),
        };
        preview_column = preview_column.push(
            col!()
                .width(Fill)
                .align_x(Alignment::End)
                .push(text(self.extension.to_uppercase()).size(14)),
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

        let mut checksum_btn =
            GuiElements::round_button(icons::calculator()).padding(Padding::from([4, 6]));

        if !self.is_calculating_checksum && self.finished && file_path.exists() {
            checksum_btn = checksum_btn.on_press(MetadataMessage::CalculateChecksum);
        }

        let checksum_col = col!()
            .spacing(5)
            .align_x(Alignment::Start)
            .push(
                row!()
                    .spacing(10)
                    .align_y(Alignment::Center)
                    .push(text("SHA256").width(Fill))
                    .push(checksum_btn),
            )
            .push(
                row!().spacing(5).align_y(Alignment::Center).push(
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
                    .size(14)
                    .on_input(|_| MetadataMessage::Ignore),
                ),
            );

        let mut download_info_col = col![].spacing(5);

        download_info_col = if self.download_error.is_empty() {
            download_info_col
                .push(text("URL").width(Fill))
                .push(
                    text_input("", &self.url)
                        .size(14)
                        .on_input(|_| MetadataMessage::Ignore),
                )
                .push(vertical_space().height(5))
                .push(checksum_col)
        } else {
            download_info_col
                .push(text("URL").width(Fill))
                .push(
                    text_input("", &self.url)
                        .size(14)
                        .on_input(|_| MetadataMessage::Ignore),
                )
                .push(vertical_space().height(5))
                .push(checksum_col)
                .push(vertical_space().height(5))
                .push(
                    col![
                        text("ERROR").width(Fill),
                        row![text_input("download error...", &self.download_error)
                            .size(14)
                            .on_input(|_| MetadataMessage::Ignore)]
                        .spacing(5)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(5)
                    .align_x(Alignment::Start),
                )
        };

        let mut pane_close_button =
            GuiElements::round_button(icons::close_line()).padding(Padding::from([2, 4]));
        if !settings.metadata_always_enabled {
            pane_close_button = pane_close_button.on_press(MetadataMessage::ClosePane);
        }

        container(GuiElements::scrollbar(
            col!()
                .padding(1)
                .spacing(20)
                .push(
                    col!()
                        .spacing(5)
                        .push(
                            row!()
                                .width(Fill)
                                .spacing(20)
                                .align_y(Alignment::Center)
                                .push(text("Resources").width(Fill))
                                .push(pane_close_button),
                        )
                        .push(
                            text(format!(
                                "{} • {}",
                                get_file_type(&self.extension),
                                get_relative_file_size(self.size)
                            ))
                            .class(AtomStyleText::Dimmed)
                            .size(12),
                        ),
                )
                .push(download_info_col)
                .push(
                    container(preview_column)
                        .padding(10)
                        .class(AtomStyleContainer::PreviewContainer)
                        .height(250),
                )
                .push(
                    col!()
                        .spacing(5)
                        .width(Fill)
                        .push(text("Information"))
                        .push(
                            row!()
                                .width(Fill)
                                .align_y(Alignment::Center)
                                .push(
                                    text("Created")
                                        .class(AtomStyleText::Dimmed)
                                        .size(12)
                                        .width(FillPortion(1)),
                                )
                                .push(text(time_created).size(10)),
                        )
                        .push(
                            row!()
                                .width(Fill)
                                .align_y(Alignment::Center)
                                .push(
                                    text("Modified")
                                        .class(AtomStyleText::Dimmed)
                                        .size(12)
                                        .width(FillPortion(1)),
                                )
                                .push(text(time_modified).size(10)),
                        )
                        .push(
                            row!()
                                .width(Fill)
                                .align_y(Alignment::Center)
                                .push(
                                    text("Last Opened")
                                        .class(AtomStyleText::Dimmed)
                                        .size(12)
                                        .width(FillPortion(1)),
                                )
                                .push(text(time_accessed).size(10)),
                        ),
                )
                .push(
                    row!()
                        .width(Fill)
                        .spacing(5)
                        .push(open_btn)
                        .push(delete_btn),
                ),
            settings.scrollbars_visible,
        ))
        .padding(15)
        .class(AtomStyleContainer::ListContainer)
        .width(METADATA_PANEL_WIDTH)
        .height(Fill)
        .into()
    }
}
