use super::AtomDownload;
use crate::{
    components::settings::ListLayout,
    elements::GuiElements,
    font::{file_type_icon, icon, CustomFont},
    messages::DownloadMessage,
    style::{button::AtomStyleButton, container::AtomStyleContainer, Theme},
    utils::helpers::get_formatted_time,
};
use iced::{
    widget::{
        button, column as col, container, horizontal_space, progress_bar, row, text, tooltip,
    },
    Element, Length, Renderer,
};

impl AtomDownload {
    pub fn view(&self, layout: &ListLayout) -> Element<DownloadMessage, Theme, Renderer> {
        let text_size = 12.0;
        let size_format = |size: usize| -> (f64, &str) {
            let suffix: [&str; 4] = ["Bytes", "KB", "MB", "GB"];
            let size_len = size.to_string().len();
            let size_len = if size_len % 3 == 0 {
                (size_len / 3) - 1
            } else {
                size_len / 3
            };
            let base: usize = 1024;
            let power = base.pow(size_len as u32);
            let size_formatted = size as f64 / power as f64;
            (size_formatted, suffix[size_len])
        };

        let downloaded = size_format(self.downloaded);
        let size = size_format(self.size);
        let progress = ((self.downloaded * 100) as f64 / self.size as f64) as f32;

        let download_state_icon = if self.downloading {
            '\u{ec72}'
        } else if self.is_downloaded() {
            '\u{ec7f}'
        } else {
            '\u{ec74}'
        };

        let transfer_rate = format!("{0:<3.2} MB/s", self.transfer_rate);
        let eta = if self.size == 0 || self.transfer_rate == 0.0 {
            String::from("0.0 second(s)")
        } else {
            let size = if self.size > self.downloaded {
                self.size - self.downloaded
            } else {
                self.size
            } as f64;
            get_formatted_time((size / (self.transfer_rate * (1000.0 * 1000.0))) as u64)
        };

        let file_name_col = container(
            row!()
                .align_items(iced::Alignment::Center)
                .spacing(10)
                .push(file_type_icon(self.file_name.split('.').last().unwrap()).size(20))
                .push(text(&self.file_name).size(text_size)),
        )
        .width(iced::Length::FillPortion(2))
        .style(AtomStyleContainer::Transparent)
        .align_x(iced::alignment::Horizontal::Left);

        let file_size_col = container(
            text(format!(
                "{0:<4.2} / {1:>4.2} {2}",
                downloaded.0, size.0, size.1
            ))
            .size(text_size),
        )
        .width(iced::Length::Fill)
        .style(AtomStyleContainer::Transparent)
        .align_x(iced::alignment::Horizontal::Center);

        let progress_col = container(if !self.error.is_empty() {
            row!().push(text("Failed").size(text_size))
        } else if self.joined_bytes > 0 {
            row!()
                .spacing(5)
                .align_items(iced::Alignment::Center)
                .push(
                    progress_bar(0.0..=100.0, ((100 * self.joined_bytes) / self.size) as f32)
                        .height(iced::Length::Fixed(5.0)),
                )
                .push(
                    text(format!(
                        "{0:>4.2} %",
                        if progress.is_nan() { 0.0 } else { progress }
                    ))
                    .size(text_size),
                )
        } else if self.size != 0 && self.downloaded >= self.size && !self.joining {
            row!().push(text("Completed").size(text_size))
        } else if self.joining {
            row!().push(text("Joining").size(text_size))
        } else {
            row!()
                .spacing(5)
                .align_items(iced::Alignment::Center)
                .push(progress_bar(0.0..=100.0, progress).height(iced::Length::Fixed(5.0)))
                .push(
                    text(format!(
                        "{0:>4.2} %",
                        if progress.is_nan() { 0.0 } else { progress }
                    ))
                    .size(text_size),
                )
        })
        .width(iced::Length::Fill)
        .style(AtomStyleContainer::Transparent)
        .align_x(iced::alignment::Horizontal::Center);

        let mut actions_row = row!().spacing(5);
        if !self.deleted {
            let mut start_pause_btn = GuiElements::round_button(download_state_icon);

            if self.downloading && self.downloaded <= self.size {
                start_pause_btn = start_pause_btn.on_press(DownloadMessage::Paused);
            } else if self.joining || (self.downloaded > self.size && self.downloading) {
            } else {
                start_pause_btn = start_pause_btn.on_press(DownloadMessage::Downloading);
            }

            actions_row = actions_row
                .push(start_pause_btn)
                .push(GuiElements::round_button('\u{ee09}').on_press(DownloadMessage::MarkDeleted));
        } else {
            actions_row = actions_row.push(
                tooltip(
                    GuiElements::round_button('\u{ee09}')
                        .on_press(DownloadMessage::RemoveDownload(true)),
                    text("Will remove all the cached/incomplete files from the disk as well.")
                        .size(10),
                    tooltip::Position::Top,
                )
                .style(AtomStyleContainer::ToolTipContainer)
                .gap(10)
                .padding(10),
            );
        }

        let main_row = match layout {
            crate::components::settings::ListLayout::ListExtended => {
                let icon_size = 12.0;
                col!()
                    .padding(10)
                    .spacing(10)
                    .push(
                        row!()
                            .spacing(10)
                            .align_items(iced::Alignment::Center)
                            .push(file_name_col.width(Length::Fill))
                            .push(actions_row.align_items(iced::Alignment::Center)),
                    )
                    .push(
                        container(horizontal_space().width(Length::Fill))
                            .height(Length::Fixed(1.0))
                            .style(AtomStyleContainer::ListItemContainer)
                            .width(Length::Fill),
                    )
                    .push(
                        row!()
                            .spacing(20)
                            .align_items(iced::Alignment::Center)
                            .push(
                                row!()
                                    .spacing(5)
                                    .align_items(iced::Alignment::Center)
                                    .push(icon('\u{ec45}', CustomFont::IcoFont).size(icon_size))
                                    .push(text(&self.added).size(text_size)),
                            )
                            .push(text("•"))
                            .push(
                                row!()
                                    .spacing(5)
                                    .align_items(iced::Alignment::Center)
                                    .push(icon('\u{efbe}', CustomFont::IcoFont).size(icon_size))
                                    .push(file_size_col.width(Length::Shrink)),
                            )
                            .push(text("•"))
                            .push(
                                row!()
                                    .spacing(5)
                                    .align_items(iced::Alignment::Center)
                                    .push(icon('\u{eff3}', CustomFont::IcoFont).size(icon_size))
                                    .push(text(&transfer_rate).size(text_size)),
                            )
                            .push(text("•"))
                            .push(
                                row!()
                                    .spacing(5)
                                    .align_items(iced::Alignment::Center)
                                    .push(icon('\u{f022}', CustomFont::IcoFont).size(icon_size))
                                    .push(text(&eta).size(text_size)),
                            )
                            .push(text("•"))
                            .push(
                                row!()
                                    .spacing(5)
                                    .align_items(iced::Alignment::Center)
                                    .push(
                                        icon(
                                            if self.error.is_empty() {
                                                if progress >= 100.0 {
                                                    '\u{eed7}'
                                                } else {
                                                    '\u{ec60}'
                                                }
                                            } else {
                                                '\u{eede}'
                                            },
                                            CustomFont::IcoFont,
                                        )
                                        .size(icon_size),
                                    )
                                    .push(progress_col.width(Length::Shrink)),
                            ),
                    )
            }
            crate::components::settings::ListLayout::List => col!().push(
                row!()
                    .align_items(iced::Alignment::Center)
                    .padding(10)
                    .spacing(10)
                    .push(file_name_col)
                    .push(file_size_col)
                    .push(progress_col)
                    .push(
                        container(text(&transfer_rate).size(text_size))
                            .width(iced::Length::Fixed(100.0))
                            .style(AtomStyleContainer::Transparent)
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        container(text(&eta).size(text_size))
                            .width(iced::Length::Fixed(100.0))
                            .style(AtomStyleContainer::Transparent)
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        container(text(&self.added).size(text_size))
                            .width(iced::Length::Fixed(80.0))
                            .style(AtomStyleContainer::Transparent)
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        container(actions_row)
                            .width(iced::Length::Fixed(80.0))
                            .style(AtomStyleContainer::Transparent)
                            .align_x(iced::alignment::Horizontal::Right),
                    ),
            ),
        };

        let mut download_container = container(
            button(main_row)
                .on_press(if self.deleted {
                    DownloadMessage::Ignore
                } else {
                    DownloadMessage::DownloadSelected
                })
                .padding(0)
                .style(AtomStyleButton::Neutral),
        )
        .width(iced::Length::Shrink)
        .padding(0);

        if self.error.is_empty() {
            download_container = download_container.style(AtomStyleContainer::ListItemContainer);
        } else {
            download_container = download_container.style(AtomStyleContainer::ErrorContainer);
        }

        if self.show_delete_confirm_dialog {
            let move2trash_btn = tooltip(
                GuiElements::primary_button(vec![
                    icon('\u{ec53}', CustomFont::IcoFont),
                    text("trash"),
                ])
                .width(Length::Fixed(150.0))
                .on_press(DownloadMessage::RemoveDownload(false)),
                text("Will move the download from the main list to the trashed list without deleting the cached/incomplete files.").size(10),
                tooltip::Position::Top,
            )
            .style(AtomStyleContainer::ToolTipContainer)
            .gap(10)
            .padding(10);

            let force_delete_btn = tooltip(
                GuiElements::primary_button(vec![
                    icon('\u{ec53}', CustomFont::IcoFont),
                    text("force delete"),
                ])
                .width(Length::Fixed(200.0))
                .on_press(DownloadMessage::RemoveDownload(true)),
                text("Removes the download from the list and the cached/incomplete files from the disk.").size(10),
                tooltip::Position::Top,
            )
            .style(AtomStyleContainer::ToolTipContainer)
            .gap(10)
            .padding(10);

            let cancel_btn = GuiElements::primary_button(vec![
                icon('\u{eede}', CustomFont::IcoFont),
                text("cancel"),
            ])
            .width(Length::Fixed(150.0))
            .on_press(DownloadMessage::HideDialog);

            GuiElements::modal(
                download_container,
                "Are you sure?",
                row!()
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(move2trash_btn)
                    .push(force_delete_btn)
                    .push(cancel_btn),
                DownloadMessage::HideDialog,
            )
        } else {
            download_container.into()
        }
    }
}
