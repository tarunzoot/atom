use std::{ffi::OsStr, path::PathBuf};

use super::AtomDownload;
use crate::{
    components::settings::ListLayout,
    elements::GuiElements,
    font::{file_type_icon, get_file_type, icon, CustomFont},
    messages::DownloadMessage,
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomStyleText, Theme},
    utils::helpers::get_formatted_time,
};
use iced::{
    widget::{button, column as col, container, progress_bar, row, text, tooltip, vertical_space},
    Element, Length, Padding, Renderer,
};

impl AtomDownload {
    fn get_formatted_size(&self, size: usize) -> (f64, &str) {
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
    }

    fn get_formatted_eta(&self) -> String {
        if self.size == 0 || self.transfer_rate == 0.0 {
            String::from("0.0 second(s)")
        } else {
            let size = if self.size > self.downloaded {
                self.size - self.downloaded
            } else {
                self.size
            } as f64;
            get_formatted_time((size / (self.transfer_rate * (1000.0 * 1000.0))) as u64)
        }
    }

    fn get_formatted_transfer_rate(&self) -> String {
        format!("{0:<3.2} MB/s", self.transfer_rate)
    }

    fn get_formatted_progress_percent(&self, decimals: bool) -> String {
        let percent = self.get_progress_percent();
        if decimals {
            format!("{0:>4.2} %", if percent.is_nan() { 0.0 } else { percent })
        } else {
            format!(
                "{0:>4.0}%",
                if percent.is_nan() {
                    0.0
                } else {
                    percent.floor()
                }
            )
        }
    }

    fn get_progress_percent(&self) -> f32 {
        ((self.downloaded * 100) as f64 / self.size as f64) as f32
    }

    fn get_download_state_icon(&self) -> char {
        if self.downloading {
            '\u{ec72}'
        } else if self.is_downloaded() {
            '\u{ec7f}'
        } else {
            '\u{ec74}'
        }
    }

    fn get_file_name_view(&self, text_size: f32) -> Element<DownloadMessage, Theme, Renderer> {
        let downloaded = self.get_formatted_size(self.downloaded);
        let size = self.get_formatted_size(self.size);

        col![
            row![
                file_type_icon(self.file_name.split('.').last().unwrap())
                    .size(text_size)
                    .style(AtomStyleText::Accented),
                text(&self.file_name).size(text_size)
            ]
            .align_items(iced::Alignment::Center)
            .spacing(5),
            row![
                icon('\u{f15fc}', CustomFont::Symbols)
                    .size(text_size - 4.0)
                    .style(AtomStyleText::Dimmed),
                text(format!(
                    "{0:<4.2} {1} of {2:>4.2} {3}",
                    downloaded.0, downloaded.1, size.0, size.1
                ))
                .width(Length::Shrink)
                .size(text_size - 2.0)
                .style(AtomStyleText::Dimmed),
            ]
            .align_items(iced::Alignment::Center)
            .spacing(5)
        ]
        .align_items(iced::Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(5))
        .align_items(iced::Alignment::Start)
        .into()
    }

    // fn get_file_size_view(&self, text_size: f32) -> Element<DownloadMessage, Theme, Renderer> {
    //     let downloaded = self.get_formatted_size(self.downloaded);
    //     let size = self.get_formatted_size(self.size);

    //     let text_col = col![row![
    //         text(format!("{0:<4.2}{1} of ", downloaded.0, downloaded.1))
    //             .width(Length::Shrink)
    //             .size(text_size)
    //             .style(AtomStyleText::Dimmed),
    //         text(format!("{0:>4.2}{1}", size.0, size.1)).size(text_size)
    //     ]
    //     .align_items(iced::Alignment::Center)
    //     .spacing(2)]
    //     .spacing(5)
    //     .align_items(iced::Alignment::Start);

    //     text_col.width(Length::FillPortion(2)).into()
    // }

    fn get_failed_view(
        &self,
        text_size: f32,
        length: Length,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        row!()
            .push(
                container(
                    row![
                        icon('\u{eedd}', CustomFont::IcoFont).size(text_size),
                        text("Failed").size(text_size - 2.0)
                    ]
                    .spacing(5)
                    .align_items(iced::Alignment::Center),
                )
                .style(AtomStyleContainer::PillError)
                .padding(Padding::from([3, 10])),
            )
            .width(length)
            .into()
    }

    fn get_completed_view(
        &self,
        text_size: f32,
        length: Length,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        row![container(
            row![
                icon('\u{eed7}', CustomFont::IcoFont).size(text_size),
                text("Completed").size(text_size - 2.0)
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center),
        )
        .style(AtomStyleContainer::PillSuccess)
        .padding(Padding::from([3, 10]))]
        .width(length)
        .into()
    }

    fn get_joining_view(
        &self,
        text_size: f32,
        length: Length,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        row![container(
            row![
                icon('\u{e984}', CustomFont::IcoFont).size(text_size),
                text("Joining").size(text_size - 2.0)
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center),
        )
        .style(AtomStyleContainer::PillSuccess)
        .padding(Padding::from([3, 10])),]
        .width(length)
        .into()
    }

    fn get_joining_progress_view(
        &self,
        text_size: f32,
        length: Length,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        let status = ((100 * self.joined_bytes) / self.size) as f32;
        row![
            progress_bar(0.0..=100.0, status).height(iced::Length::Fixed(5.0)),
            text(format!(
                "{0:>4.2} %",
                if status.is_nan() { 0.0 } else { status }
            ))
            .size(text_size),
        ]
        .spacing(5)
        .align_items(iced::Alignment::Center)
        .width(length)
        .into()
    }

    fn get_status_view(
        &self,
        layout: ListLayout,
        text_size: f32,
        responsive: bool,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        let length = if matches!(layout, ListLayout::List) {
            Length::FillPortion(2)
        } else {
            Length::Shrink
        };

        if !self.error.is_empty() {
            return self.get_failed_view(text_size, length);
        } else if self.joined_bytes > 0 {
            return self.get_joining_progress_view(text_size, length);
        } else if self.size != 0 && self.downloaded >= self.size && !self.joining {
            return self.get_completed_view(text_size, length);
        } else if self.joining {
            return self.get_joining_view(text_size, length);
        }

        let progress = self.get_progress_percent();
        let percent_el = text(self.get_formatted_progress_percent(true)).size(text_size - 2.0);
        let progress_bar_el = progress_bar(0.0..=100.0, progress).height(iced::Length::Fixed(5.0));
        let mut progress_row = row!()
            .spacing(5)
            .align_items(iced::Alignment::Center)
            .width(length);

        match layout {
            ListLayout::ListExtended => {
                progress_row = progress_row
                    .push(icon('\u{eff4}', CustomFont::IcoFont).size(text_size))
                    .push(progress_bar_el)
                    .push(percent_el);
                return progress_row.into();
            }
            ListLayout::List => {
                let mut upper_row = row!().align_items(iced::Alignment::Center);
                if responsive {
                    upper_row = upper_row.push(
                        text(self.get_formatted_transfer_rate())
                            .width(Length::Fill)
                            .size(text_size - 2.0),
                    );
                }

                progress_row = progress_row.push(
                    col![
                        upper_row,
                        row![progress_bar_el, percent_el]
                            .align_items(iced::Alignment::Center)
                            .spacing(5)
                    ]
                    .align_items(iced::Alignment::Start)
                    .spacing(5),
                );
            }
        }

        progress_row.into()
    }

    fn get_transfer_rate_view(&self, text_size: f32) -> Element<DownloadMessage, Theme, Renderer> {
        col![text(self.get_formatted_transfer_rate()).size(text_size)]
            .width(iced::Length::Fill)
            .align_items(iced::Alignment::Start)
            .into()
    }

    fn get_actions_view(&self, length: Length) -> Element<DownloadMessage, Theme, Renderer> {
        let mut actions_row = row!().spacing(5);
        if !self.deleted {
            let mut start_pause_btn = GuiElements::round_button(self.get_download_state_icon());
            // let mut edit_btn = GuiElements::round_button('\u{ec55}');

            if self.downloading && self.downloaded <= self.size {
                start_pause_btn = start_pause_btn.on_press(DownloadMessage::Paused);
            } else if self.joining || (self.downloaded > self.size && self.downloading) {
            } else {
                start_pause_btn = start_pause_btn.on_press(DownloadMessage::Downloading);
                // edit_btn = edit_btn.on_press(DownloadMessage::MarkDeleted);
            }

            actions_row = actions_row
                .push(start_pause_btn)
                // .push(edit_btn)
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
        container(actions_row)
            .width(length)
            // .width(Length::Fixed(95.0))
            .padding(0)
            .style(AtomStyleContainer::Transparent)
            .align_x(iced::alignment::Horizontal::Right)
            .into()
    }

    fn list_view(
        &self,
        responsive: bool,
        text_size: f32,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        let text_size = text_size - 2.0;
        let file_name_col = self.get_file_name_view(text_size);
        let status_col = self.get_status_view(ListLayout::List, text_size, responsive);
        let transfer_rate_col = self.get_transfer_rate_view(text_size);
        let actions_col = self.get_actions_view(Length::Fixed(75.0));

        let mut main_row = row!()
            .align_items(iced::Alignment::Center)
            .padding(10)
            .spacing(15)
            .push(file_name_col)
            .push(
                text(self.get_formatted_eta())
                    .width(Length::FillPortion(1))
                    .size(text_size),
            )
            .push(status_col);

        if !responsive {
            main_row = main_row.push(transfer_rate_col);
        }

        main_row = main_row
            .push(text(&self.added).size(text_size).width(Length::Fill))
            .push(actions_col);

        col!().push(main_row).into()
    }

    fn get_extended_file_name_view(
        &self,
        text_size: f32,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        let path_buf = PathBuf::from(&self.file_name);
        let extension = path_buf
            .extension()
            .unwrap_or_else(|| OsStr::new(""))
            .to_string_lossy();

        row![col![
            row![
                file_type_icon(&extension)
                    .size(text_size - 2.0)
                    .style(AtomStyleText::Accented),
                text(&self.file_name).size(text_size - 2.0),
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center),
            row![
                icon('\u{ee57}', CustomFont::Symbols)
                    .size(text_size - 6.0)
                    .style(AtomStyleText::Dimmed),
                text(get_file_type(&extension))
                    .size(text_size - 4.0)
                    .style(AtomStyleText::Dimmed),
                text("•").style(AtomStyleText::Dimmed),
                icon('\u{ec45}', CustomFont::IcoFont)
                    .size(text_size - 6.0)
                    .style(AtomStyleText::Dimmed),
                text(&self.added)
                    .size(text_size - 4.0)
                    .style(AtomStyleText::Dimmed)
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(5)
        .align_items(iced::Alignment::Start)]
        .width(Length::FillPortion(5))
        .align_items(iced::Alignment::Center)
        .spacing(10)
        .into()
    }

    fn get_extended_file_size_view(
        &self,
        text_size: f32,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        let downloaded = self.get_formatted_size(self.downloaded);
        let size = self.get_formatted_size(self.size);
        let icon_size = text_size - 4.0;
        let text_size = text_size - 4.0;

        col![col![
            row![
                icon('\u{e90b}', CustomFont::IcoFont)
                    .size(icon_size)
                    .style(AtomStyleText::Dimmed),
                text("Size")
                    .style(AtomStyleText::Dimmed)
                    .size(text_size)
                    .width(Length::Fill),
                text(format!("{0:<4.2} {1}", downloaded.0, downloaded.1))
                    .size(text_size)
                    .style(AtomStyleText::Dimmed)
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center),
            row![text(format!("{0:>4.2} {1}", size.0, size.1)).size(text_size + 2.0)]
                .spacing(5)
                .align_items(iced::Alignment::Center)
        ]
        .spacing(10)
        .align_items(iced::Alignment::Start)]
        .spacing(0)
        .align_items(iced::Alignment::Center)
        .width(Length::FillPortion(2))
        .into()
    }

    fn get_extended_speed_view(&self, text_size: f32) -> Element<DownloadMessage, Theme, Renderer> {
        let icon_size = text_size - 4.0;
        let text_size = text_size - 4.0;

        col![col![
            row![
                icon('\u{eff3}', CustomFont::IcoFont)
                    .size(icon_size)
                    .style(AtomStyleText::Dimmed),
                text("Speed")
                    .style(AtomStyleText::Dimmed)
                    .size(text_size)
                    .width(Length::Fill),
                text(self.get_formatted_eta())
                    .size(text_size)
                    .style(AtomStyleText::Dimmed)
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center),
            row![text(self.get_formatted_transfer_rate()).size(text_size + 2.0)]
                .spacing(5)
                .align_items(iced::Alignment::Center)
        ]
        .spacing(10)
        .align_items(iced::Alignment::Start)]
        .spacing(0)
        .align_items(iced::Alignment::Center)
        .width(Length::FillPortion(2))
        .into()
    }

    fn get_extended_status_view(
        &self,
        text_size: f32,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        let icon_size = text_size - 4.0;
        let text_size = text_size - 4.0;

        col![col![
            row![
                icon('\u{e982}', CustomFont::IcoFont)
                    .size(icon_size)
                    .style(AtomStyleText::Dimmed),
                text("Status")
                    .size(text_size)
                    .style(AtomStyleText::Dimmed)
                    .width(Length::Fill),
                text(if self.is_downloading() {
                    "In Progress"
                } else if self.is_downloaded() {
                    "Done"
                } else {
                    "Paused"
                })
                .size(text_size)
                .style(AtomStyleText::Dimmed)
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center),
            self.get_status_view(ListLayout::ListExtended, text_size, false)
        ]
        .spacing(10)
        .align_items(iced::Alignment::Start)]
        .spacing(0)
        .align_items(iced::Alignment::Center)
        .width(Length::FillPortion(2))
        .into()
    }

    fn vertical_line(&self) -> Element<DownloadMessage, Theme, Renderer> {
        col![container(
            vertical_space()
                .height(Length::Fixed(30.0))
                .width(Length::Fixed(1.0)),
        )
        .style(AtomStyleContainer::ListItemContainer)
        .width(Length::Fixed(1.0))]
        .align_items(iced::Alignment::Center)
        .width(Length::Shrink)
        .into()
    }

    fn list_extended_view(&self, text_size: f32) -> Element<DownloadMessage, Theme, Renderer> {
        let actions_col = self.get_actions_view(Length::Shrink);

        col![row![
            self.get_extended_file_name_view(text_size),
            self.vertical_line(),
            self.get_extended_file_size_view(text_size),
            self.vertical_line(),
            self.get_extended_speed_view(text_size),
            self.vertical_line(),
            self.get_extended_status_view(text_size),
            self.vertical_line(),
            actions_col
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
        .width(Length::Fill)]
        .padding(10)
        .spacing(5)
        .into()
    }

    pub fn view(
        &self,
        layout: &ListLayout,
        text_size: f32,
        responsive: bool,
    ) -> Element<DownloadMessage, Theme, Renderer> {
        let text_size = text_size - if responsive { 2.0 } else { 0.0 };

        let main_row = match layout {
            crate::components::settings::ListLayout::ListExtended => {
                self.list_extended_view(text_size)
            }
            crate::components::settings::ListLayout::List => self.list_view(responsive, text_size),
        };

        let download_container = container(
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
        .padding(0)
        .style(AtomStyleContainer::ListItemContainer);

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
                text("Are you sure?").size(24),
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
