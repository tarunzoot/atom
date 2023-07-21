use super::AtomDownload;
use crate::{
    font::file_type_icon,
    gui::GuiElements,
    messages::DownloadMessage,
    style::{AtomStyleButton, AtomStyleContainer, Theme},
    utils::helpers::get_formatted_time,
};
use iced::{
    widget::{button, container, progress_bar, row, text},
    Element, Renderer,
};

impl AtomDownload {
    pub fn view(&self) -> Element<DownloadMessage, Renderer<Theme>> {
        let size_format = |size: usize| -> (f64, &str) {
            let suffix = vec!["Bytes", "KB", "MB", "GB"];
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

        let download_state_icon = if self.is_downloading {
            '\u{ec72}'
        } else if self.is_downloaded() {
            '\u{ec7f}'
        } else {
            '\u{ec74}'
        };

        let transfer_rate = format!("{0:6.2} MB/s", self.transfer_rate);
        let eta = if self.size == 0 || self.transfer_rate == 0.0 {
            String::from("0.0 second(s)")
        } else {
            get_formatted_time((self.size as f64 / (self.transfer_rate * (1000.0 * 1000.0))) as u64)
        };

        let file_name_col = container(
            row!()
                .align_items(iced::Alignment::Center)
                .spacing(10)
                .push(file_type_icon(self.file_name.split('.').last().unwrap()).size(20))
                .push(text(&self.file_name).size(14)),
        )
        .width(iced::Length::FillPortion(2))
        .style(AtomStyleContainer::Transparent)
        .align_x(iced::alignment::Horizontal::Left);

        let file_size_col = container(
            text(format!(
                "{0:<7.2} / {1:>7.2} {2}",
                downloaded.0, size.0, size.1
            ))
            .size(14),
        )
        .width(iced::Length::Fill)
        .style(AtomStyleContainer::Transparent)
        .align_x(iced::alignment::Horizontal::Center);

        let progress_col = container(if !self.error.is_empty() {
            row!().push(text("Error").size(14))
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
                        "{0:>6.2} %",
                        if progress.is_nan() { 0.0 } else { progress }
                    ))
                    .size(14),
                )
        } else if self.size != 0 && self.downloaded >= self.size && !self.is_joining {
            row!().push(text("Completed").size(14))
        } else if self.is_joining {
            row!().push(text("Joining").size(14))
        } else {
            row!()
                .spacing(5)
                .align_items(iced::Alignment::Center)
                .push(progress_bar(0.0..=100.0, progress).height(iced::Length::Fixed(5.0)))
                .push(
                    text(format!(
                        "{0:>6.2} %",
                        if progress.is_nan() { 0.0 } else { progress }
                    ))
                    .size(14),
                )
        })
        .width(iced::Length::FillPortion(1))
        .style(AtomStyleContainer::Transparent)
        .align_x(iced::alignment::Horizontal::Center);

        let mut actions_row = row!().spacing(5);
        if !self.is_deleted {
            let mut start_pause_btn = GuiElements::round_button(download_state_icon);

            if self.is_downloading {
                start_pause_btn = start_pause_btn.on_press(DownloadMessage::Paused);
            } else if self.is_joining {
            } else {
                start_pause_btn = start_pause_btn.on_press(DownloadMessage::Downloading);
            }

            actions_row = actions_row
                .push(start_pause_btn)
                .push(GuiElements::round_button('\u{ee09}').on_press(DownloadMessage::MarkDeleted));
        } else {
            actions_row = actions_row.push(
                GuiElements::round_button('\u{ee09}').on_press(DownloadMessage::RemoveDownload),
            );
        }

        let main_row = row!()
            .align_items(iced::Alignment::Center)
            .padding(10)
            .spacing(10)
            .push(file_name_col)
            .push(file_size_col)
            .push(progress_col)
            .push(
                container(text(&transfer_rate).size(14))
                    .width(iced::Length::Fill)
                    .style(AtomStyleContainer::Transparent)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .push(
                container(text(&eta).size(14))
                    .width(iced::Length::Fill)
                    .style(AtomStyleContainer::Transparent)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .push(
                container(text(&self.added).size(14))
                    .width(iced::Length::Fill)
                    .style(AtomStyleContainer::Transparent)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .push(
                container(actions_row)
                    .width(iced::Length::Fixed(70.0))
                    .style(AtomStyleContainer::Transparent)
                    .align_x(iced::alignment::Horizontal::Right),
            );

        let mut download_container = container(
            button(main_row)
                .on_press(if self.is_deleted {
                    DownloadMessage::Ignore
                } else {
                    DownloadMessage::DownloadSelected
                })
                .padding(0)
                .style(AtomStyleButton::Neutral),
        )
        .width(iced::Length::Fill)
        .padding(0);

        if self.error.is_empty() {
            download_container = download_container.style(AtomStyleContainer::ListItemContainer);
        } else {
            download_container = download_container.style(AtomStyleContainer::ErrorContainer);
        }

        download_container.into()
    }
}
