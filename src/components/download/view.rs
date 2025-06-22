use super::AtomDownload;
use crate::{
    components::{
        listview_header::get_list_view_header_column_length,
        settings::{AtomSettings, ListLayout},
    },
    elements::GuiElements,
    font::{file_type_icon, get_file_type},
    icons,
    messages::DownloadMessage,
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomStyleText, AtomTheme},
    utils::helpers::{get_formatted_time, ListViewColumns},
};
use iced::{
    widget::{
        button, column as col, container, progress_bar, row, text, tooltip, vertical_space, Text,
    },
    Alignment, Element,
    Length::{self, FillPortion, Fixed, Shrink},
    Padding, Renderer,
};
use std::path::PathBuf;

impl AtomDownload {
    fn get_formatted_size(&self, size: usize) -> (f64, &str) {
        let suffix: [&str; 4] = ["By", "KB", "MB", "GB"];
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
            format!("{0:>4.2} %", percent)
        } else {
            format!("{0:>4.0} %", percent.floor())
        }
    }

    fn get_progress_percent(&self) -> f32 {
        let percent = ((self.downloaded * 100) as f64 / self.size as f64) as f32;
        if percent.is_infinite() || percent.is_nan() {
            0.0
        } else {
            percent
        }
    }

    fn get_download_state_icon<'a>(&self) -> Text<'a, AtomTheme> {
        if self.downloading {
            icons::pause()
        } else if self.is_downloaded() {
            icons::reply()
        } else {
            icons::play()
        }
    }

    fn get_file_name_view(&self, text_size: f32) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let path_buf = PathBuf::from(&self.file_name);
        let extension = path_buf
            .extension()
            .map(|extension| extension.to_string_lossy().to_string())
            .unwrap_or_else(String::default);

        let file_name = if self.file_name.len() > 50 {
            format!("{}....{}", &self.file_name[0..41], extension)
        } else {
            self.file_name.clone()
        };

        col![
            row![
                file_type_icon(self.file_name.split('.').next_back().unwrap())
                    .size(text_size)
                    .class(AtomStyleText::Accented),
                text(file_name).size(text_size)
            ]
            .align_y(iced::Alignment::Center)
            .spacing(5),
            row![
                icons::hash()
                    .size(text_size - 4.0)
                    .class(AtomStyleText::Dimmed),
                text(get_file_type(&extension))
                    .size(text_size - 2.0)
                    .class(AtomStyleText::Dimmed),
            ]
            .align_y(iced::Alignment::Center)
            .spacing(5)
        ]
        .spacing(10)
        .width(get_list_view_header_column_length(
            ListViewColumns::FileName,
        ))
        .align_x(iced::Alignment::Start)
        .into()
    }

    fn get_file_size_view(&self, text_size: f32) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let downloaded = self.get_formatted_size(self.downloaded);
        let size = self.get_formatted_size(self.size);

        let text_col = col![col![
            row![
                icons::download_alt().size(text_size),
                text(format!("{0:0>6.2} {1}", downloaded.0, downloaded.1))
                    .width(Shrink)
                    .size(text_size),
            ]
            .align_y(Alignment::Center)
            .spacing(5),
            row![
                icons::box_open().size(text_size),
                text(format!("{0:0>6.2} {1}", size.0, size.1)).size(text_size)
            ]
            .align_y(Alignment::Center)
            .spacing(5)
        ]
        .align_x(Alignment::Start)
        .spacing(5)]
        .spacing(5)
        .align_x(Alignment::Start);

        text_col
            .width(get_list_view_header_column_length(
                ListViewColumns::FileSize,
            ))
            .into()
    }

    fn get_failed_view(
        &self,
        text_size: f32,
        length: Length,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        row!()
            .push(
                container(
                    row![
                        icons::close_circled().size(text_size),
                        text("Failed").size(text_size - 2.0)
                    ]
                    .spacing(5)
                    .align_y(iced::Alignment::Center),
                )
                .class(AtomStyleContainer::PillError)
                .padding(Padding::from([3, 10])),
            )
            .width(length)
            .into()
    }

    fn get_completed_view(
        &self,
        text_size: f32,
        length: Length,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        row![container(
            row![
                icons::check_circled().size(text_size),
                text("Completed").size(text_size - 2.0)
            ]
            .spacing(5)
            .align_y(iced::Alignment::Center),
        )
        .class(AtomStyleContainer::PillSuccess)
        .padding(Padding::from([3, 10]))]
        .width(length)
        .into()
    }

    fn get_joining_view(
        &self,
        text_size: f32,
        length: Length,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        row![container(
            row![
                icons::joining().size(text_size),
                text("Joining").size(text_size - 2.0)
            ]
            .spacing(5)
            .align_y(iced::Alignment::Center),
        )
        .class(AtomStyleContainer::PillSuccess)
        .padding(Padding::from([3, 10])),]
        .width(length)
        .into()
    }

    fn get_joining_progress_view(
        &self,
        text_size: f32,
        length: Length,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
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
        .align_y(iced::Alignment::Center)
        .width(length)
        .into()
    }

    fn get_status_view(
        &self,
        layout: ListLayout,
        text_size: f32,
        _responsive: bool,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let (length, progress_bar_length) = if matches!(layout, ListLayout::List) {
            (
                get_list_view_header_column_length(ListViewColumns::Status),
                Length::Fixed(50.0),
            )
        } else {
            (Length::Shrink, Length::Fill)
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
        let progress_bar_el = progress_bar(0.0..=100.0, progress)
            .height(iced::Length::Fixed(5.0))
            .width(progress_bar_length);
        let mut progress_row = row!()
            .spacing(5)
            .align_y(iced::Alignment::Center)
            .width(length);

        match layout {
            ListLayout::ListExtended => {
                progress_row = progress_row
                    .push(icons::spinner().size(text_size))
                    .push(progress_bar_el)
                    .push(percent_el);
                return progress_row.into();
            }
            ListLayout::List => {
                // let mut upper_row = row!().align_y(Alignment::Center);
                // if responsive {
                //     upper_row = upper_row.push(
                //         text(self.get_formatted_transfer_rate())
                //             .width(Length::Fill)
                //             .size(text_size - 2.0),
                //     );
                // }

                progress_row = progress_row.push(
                    col![
                        // upper_row,
                        row![progress_bar_el, percent_el]
                            .align_y(Alignment::Center)
                            .spacing(5)
                    ]
                    .align_x(Alignment::Start)
                    .spacing(5),
                );
            }
        }

        progress_row.into()
    }

    fn get_transfer_rate_view(
        &self,
        text_size: f32,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        col![text(self.get_formatted_transfer_rate()).size(text_size)]
            .width(get_list_view_header_column_length(ListViewColumns::Speed))
            .align_x(Alignment::Start)
            .into()
    }

    fn get_actions_view(&self, length: Length) -> Element<DownloadMessage, AtomTheme, Renderer> {
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
                .push(
                    GuiElements::round_button(icons::trash_bin_closed())
                        .on_press(DownloadMessage::MarkDeleted),
                );
        } else {
            actions_row = actions_row.push(
                tooltip(
                    GuiElements::round_button(icons::trash_bin_closed())
                        .on_press(DownloadMessage::RemoveDownload(true)),
                    text("Will remove all the cached/incomplete files from the disk as well.")
                        .size(10),
                    tooltip::Position::Top,
                )
                .class(AtomStyleContainer::ToolTipContainer)
                .gap(10)
                .padding(10),
            );
        }
        container(actions_row)
            .width(length)
            // .width(Length::Fixed(95.0))
            .padding(0)
            .class(AtomStyleContainer::Transparent)
            .align_x(iced::alignment::Horizontal::Right)
            .into()
    }

    fn list_view(
        &self,
        responsive: bool,
        text_size: f32,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let text_size = text_size - 4.0;
        let file_name_col = self.get_file_name_view(text_size);
        let file_size_col = self.get_file_size_view(text_size);
        let status_col = self.get_status_view(ListLayout::List, text_size, responsive);
        let transfer_rate_col = self.get_transfer_rate_view(text_size);
        let actions_col =
            self.get_actions_view(get_list_view_header_column_length(ListViewColumns::Actions));

        let mut main_row = row!()
            .align_y(iced::Alignment::Center)
            .padding(10)
            .spacing(15)
            .push(file_name_col)
            .push(file_size_col)
            .push(status_col)
            .push(transfer_rate_col)
            .push(
                text(self.get_formatted_eta())
                    .width(get_list_view_header_column_length(ListViewColumns::Eta))
                    .size(text_size),
            );

        if !responsive {
            main_row = main_row.push(
                text(&self.added)
                    .size(text_size)
                    .width(get_list_view_header_column_length(ListViewColumns::Added)),
            );
        }

        main_row = main_row.push(actions_col);

        col!().push(main_row).into()
    }

    fn get_extended_file_name_view(&self, text_size: f32) -> Element<DownloadMessage, AtomTheme> {
        let text_icon_size = text_size - 6.0;
        let path_buf = PathBuf::from(&self.file_name);
        let extension = path_buf
            .extension()
            .map(|extension| extension.to_string_lossy().to_string())
            .unwrap_or_else(String::default);

        let file_name = if self.file_name.len() > 50 {
            format!("{}....{}", &self.file_name[0..41], extension)
        } else {
            self.file_name.clone()
        };

        row![col![
            row![
                file_type_icon(&extension)
                    .size(text_size - 2.0)
                    .class(AtomStyleText::Accented),
                text(file_name).size(text_size - 2.0),
            ]
            .spacing(5)
            .align_y(iced::Alignment::Center),
            row![
                icons::hash()
                    .size(text_icon_size)
                    .class(AtomStyleText::Dimmed),
                text(get_file_type(&extension))
                    .size(text_icon_size)
                    .class(AtomStyleText::Dimmed),
                text("•").class(AtomStyleText::Dimmed),
                icons::calendar()
                    .size(text_icon_size)
                    .class(AtomStyleText::Dimmed),
                text(&self.added)
                    .size(text_icon_size)
                    .class(AtomStyleText::Dimmed)
            ]
            .spacing(5)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(5)
        .align_x(Alignment::Start)]
        .width(Length::FillPortion(5))
        .align_y(Alignment::Center)
        .spacing(10)
        .into()
    }

    fn get_extended_file_size_view(
        &self,
        text_size: f32,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let downloaded = self.get_formatted_size(self.downloaded);
        let size = self.get_formatted_size(self.size);
        let icon_size = text_size - 4.0;
        let text_size = text_size - 4.0;

        col![col![
            row![
                icons::file_size()
                    .size(icon_size)
                    .class(AtomStyleText::Dimmed),
                text("Size")
                    .font(iced::Font {
                        family: iced::font::Family::Name("Lexend Deca"),
                        weight: iced::font::Weight::Black,
                        ..Default::default()
                    })
                    .class(AtomStyleText::Dimmed)
                    .size(text_size)
                    .width(Shrink),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
            row![
                text(format!("{0:<4.2} {1}", downloaded.0, downloaded.1)).size(text_size),
                text("•").class(AtomStyleText::Dimmed),
                text(format!("{0:>4.2} {1}", size.0, size.1)).size(text_size)
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        ]
        .spacing(10)
        .align_x(Alignment::Center)]
        .spacing(0)
        .align_x(Alignment::Center)
        .width(FillPortion(2))
        .into()
    }

    fn get_extended_speed_view(
        &self,
        text_size: f32,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let icon_size = text_size - 4.0;
        let text_size = text_size - 4.0;

        col![col![
            row![
                icons::speedmeter()
                    .size(icon_size + 2.0)
                    .class(AtomStyleText::Dimmed),
                text("Speed")
                    .font(iced::Font {
                        family: iced::font::Family::Name("Lexend Deca"),
                        weight: iced::font::Weight::Black,
                        ..Default::default()
                    })
                    .class(AtomStyleText::Dimmed)
                    .size(text_size)
                    .width(Shrink)
            ]
            .spacing(5)
            .align_y(Alignment::Center),
            row![text(self.get_formatted_transfer_rate()).size(text_size)]
                .spacing(5)
                .align_y(Alignment::Center)
        ]
        .spacing(10)
        .align_x(Alignment::Center)]
        .spacing(0)
        .align_x(Alignment::Center)
        .width(FillPortion(2))
        .into()
    }

    fn get_extended_eta_view(
        &self,
        text_size: f32,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let icon_size = text_size - 4.0;
        let text_size = text_size - 4.0;

        col![col![
            row![
                icons::clock().size(icon_size).class(AtomStyleText::Dimmed),
                text("E.T.A")
                    .font(iced::Font {
                        family: iced::font::Family::Name("Lexend Deca"),
                        weight: iced::font::Weight::Black,
                        ..Default::default()
                    })
                    .class(AtomStyleText::Dimmed)
                    .size(text_size)
                    .width(Shrink),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
            row![text(self.get_formatted_eta()).size(text_size)]
                .spacing(5)
                .align_y(Alignment::Center)
        ]
        .spacing(10)
        .align_x(Alignment::Center)]
        .spacing(0)
        .align_x(Alignment::Center)
        .width(FillPortion(2))
        .into()
    }

    fn get_extended_status_view(
        &self,
        text_size: f32,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let icon_size = text_size - 4.0;
        let text_size = text_size - 4.0;

        col![col![
            row![
                icons::chart_alt()
                    .size(icon_size)
                    .class(AtomStyleText::Dimmed),
                text("Status")
                    .font(iced::Font {
                        family: iced::font::Family::Name("Lexend Deca"),
                        weight: iced::font::Weight::Black,
                        ..Default::default()
                    })
                    .size(text_size)
                    .class(AtomStyleText::Dimmed)
                    .width(Shrink),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
            self.get_status_view(ListLayout::ListExtended, text_size, false)
        ]
        .spacing(10)
        .align_x(Alignment::Center)]
        .spacing(0)
        .align_x(Alignment::Center)
        .width(FillPortion(2))
        .into()
    }

    fn vertical_line(&self) -> Element<DownloadMessage, AtomTheme, Renderer> {
        col![
            container(vertical_space().height(Fixed(30.0)).width(Fixed(1.0)),)
                .class(AtomStyleContainer::ListItemContainer)
                .width(Fixed(1.0))
        ]
        .align_x(Alignment::Center)
        .width(Shrink)
        .into()
    }

    fn list_extended_view(&self, text_size: f32) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let actions_col = self.get_actions_view(Length::Shrink);

        col![row![
            self.get_extended_file_name_view(text_size),
            self.vertical_line(),
            self.get_extended_file_size_view(text_size),
            self.vertical_line(),
            self.get_extended_speed_view(text_size),
            self.vertical_line(),
            self.get_extended_eta_view(text_size),
            self.vertical_line(),
            self.get_extended_status_view(text_size),
            self.vertical_line(),
            actions_col
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center)
        .width(Length::Fill)]
        .padding(10)
        .spacing(5)
        .into()
    }

    pub fn view(
        &self,
        settings: &AtomSettings,
        responsive: bool,
    ) -> Element<DownloadMessage, AtomTheme, Renderer> {
        let text_size = settings.font_size - if responsive { 2.0 } else { 0.0 };

        let main_row = match settings.list_layout {
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
                .class(AtomStyleButton::Neutral),
        )
        .width(iced::Length::Shrink)
        .padding(0)
        .class(AtomStyleContainer::ListItemContainer);

        if self.show_delete_confirm_dialog {
            let move2trash_btn = tooltip(
                GuiElements::primary_button(
                    icons::trash_bin_open(),
                    "trash",
                )
                .width(Length::Fixed(150.0))
                .on_press(DownloadMessage::RemoveDownload(false)),
                text("Without erasing the cached or unfinished files, will transfer the download from the main list to the garbage list.").size(10),
                tooltip::Position::Top,
            )
            .class(AtomStyleContainer::ToolTipContainer)
            .gap(10)
            .padding(10);

            let force_delete_btn = tooltip(
                GuiElements::primary_button(
                    icons::recycle_bin(),
                    "delete",
                )
                .width(Length::Fixed(200.0))
                .on_press(DownloadMessage::RemoveDownload(true)),
                text("Deletes the cached or partial files from the disc and the download from the list.").size(10),
                tooltip::Position::Top,
            )
            .class(AtomStyleContainer::ToolTipContainer)
            .gap(10)
            .padding(10);

            let cancel_btn = GuiElements::primary_button(icons::close_line_circled(), "cancel")
                .width(Length::Fixed(150.0))
                .on_press(DownloadMessage::HideDialog);

            GuiElements::modal(
                download_container,
                text(format!(
                    "Are you sure you want to delete \"{}\"?",
                    self.file_name
                ))
                .size(24),
                row!()
                    .spacing(10)
                    .align_y(iced::Alignment::Center)
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
