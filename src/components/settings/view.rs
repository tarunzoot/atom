use super::{AtomSettings, ListLayout};
use crate::{
    elements::GuiElements,
    font::{icon, CustomFont::IcoFont},
    messages::SettingsMessage,
    style::{container::AtomStyleContainer, input::AtomStyleInput, Theme},
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    widget::{
        column as col, container, pick_list, row, scrollable, slider, text, text_input, toggler,
        tooltip,
    },
    Element, Length, Padding, Renderer,
};

impl AtomSettings {
    pub fn view(&self, theme: &Theme) -> Element<SettingsMessage, Theme, Renderer> {
        let config_dir_col = col!()
            .spacing(5)
            .push(text("Configuration Directory"))
            .push(
                row!()
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(
                        text_input("", &self.config_dir.to_string_lossy())
                            .width(iced::Length::Fill)
                            .style(AtomStyleInput::Disabled)
                            .padding(ATOM_INPUT_DEFAULT_PADDING),
                    )
                    .push(
                        GuiElements::primary_button(vec![icon('\u{ef36}', IcoFont), text("open")])
                            .on_press(SettingsMessage::OpenConfigDir),
                    ),
            );

        let temp_dir_col = col!()
            .spacing(5)
            .push(text(
                "Temporary File Location (chunk files will be stored here)",
            ))
            .push(
                row!()
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(
                        text_input("", &self.cache_dir.to_string_lossy())
                            .width(iced::Length::Fill)
                            .style(AtomStyleInput::Disabled)
                            .padding(ATOM_INPUT_DEFAULT_PADDING),
                    ),
            );

        let default_dir_col = col!()
            .spacing(5)
            .push(text(
                "Default Download Location (directory where files will be moved after download)",
            ))
            .push(
                row!()
                    .align_items(iced::Alignment::Center)
                    .spacing(10)
                    .push(
                        text_input("", &self.downloads_dir)
                            .on_input(|_| SettingsMessage::BrowseDownloadsDirClicked)
                            .padding(ATOM_INPUT_DEFAULT_PADDING),
                    )
                    .push(
                        GuiElements::primary_button(vec![
                            icon('\u{ef13}', IcoFont),
                            text("browse"),
                        ])
                        .on_press(SettingsMessage::BrowseDownloadsDirClicked),
                    ),
            );

        let buttons_row = row!()
            .spacing(20)
            .push(
                GuiElements::primary_button(vec![icon('\u{ef43}', IcoFont), text("save")])
                    .on_press(SettingsMessage::SaveSettings),
            )
            .push(
                GuiElements::primary_button(vec![icon('\u{efd0}', IcoFont), text("clear cache")])
                    .on_press(SettingsMessage::ClearCacheClicked),
            )
            .push(
                GuiElements::primary_button(vec![icon('\u{eede}', IcoFont), text("cancel")])
                    .on_press(SettingsMessage::ClosePane),
            );

        let notification_toggler = toggler(
            Some("Show download notification".into()),
            self.show_notifications,
            SettingsMessage::NotificationToggle,
        )
        .spacing(10)
        .text_alignment(iced::alignment::Horizontal::Left)
        .width(iced::Length::Shrink);

        let auto_start_toggler = tooltip(
            toggler(
                Some("Auto start download from browser".into()),
                self.auto_start_download,
                SettingsMessage::AutoStartDownloadToggle,
            )
            .spacing(10)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(iced::Length::Shrink),
            text("Adding downloads from browser auto starts the download without showing new download form(disables auto open feature)").size(12),
            tooltip::Position::Top,
        )
        .style(AtomStyleContainer::ToolTipContainer)
        .padding(10)
        .gap(5);

        let close_btn_toggler = toggler(
            Some("Minimize to tray".into()),
            self.minimize_to_tray,
            SettingsMessage::QuitActionToggle,
        )
        .spacing(10)
        .text_alignment(iced::alignment::Horizontal::Left)
        .width(iced::Length::Shrink);

        let maximized_toggler = toggler(
            Some("Start Maximized ".into()),
            self.maximized,
            SettingsMessage::MaximizedActionToggle,
        )
        .spacing(10)
        .text_alignment(iced::alignment::Horizontal::Left)
        .width(iced::Length::Shrink);

        let stretch_list_toggler = tooltip(
            toggler(
                Some("Stretch List Background  ".into()),
                self.stretch_list_view,
                SettingsMessage::ListBackgroundToggle,
            )
            .spacing(10)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(iced::Length::Shrink),
            text("Stretch the list view container to fill the available space(applies a background color)").size(12),
            tooltip::Position::Top,
        )
        .gap(10)
        .padding(10)
        .style(AtomStyleContainer::ToolTipContainer);

        let new_download_notification_toggler = tooltip(
            toggler(
                Some("New Download Notification".into()),
                self.new_download_notification,
                SettingsMessage::NewDownloadNotificationToggle,
            )
            .spacing(10)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(iced::Length::Shrink),
            text("A notification is shown when a new download is added").size(12),
            tooltip::Position::Top,
        )
        .gap(10)
        .padding(10)
        .style(AtomStyleContainer::ToolTipContainer);

        let options_row = container(
            row!()
                .spacing(10)
                .align_items(iced::Alignment::Center)
                .width(Length::Fill)
                .push(
                    col!()
                        .spacing(10)
                        .width(Length::Fill)
                        .align_items(iced::Alignment::Start)
                        .push(notification_toggler)
                        .push(auto_start_toggler),
                )
                .push(
                    col!()
                        .spacing(10)
                        .width(Length::Fill)
                        .align_items(iced::Alignment::Center)
                        .push(close_btn_toggler)
                        .push(maximized_toggler),
                )
                .push(
                    col!()
                        .spacing(10)
                        .width(Length::Fill)
                        .align_items(iced::Alignment::End)
                        .push(stretch_list_toggler)
                        .push(new_download_notification_toggler),
                ),
        )
        .width(Length::Fill)
        .padding(20)
        .style(AtomStyleContainer::ListContainer);

        let settings_col = col!()
            .spacing(20)
            .padding(Padding::from([0, 10, 10, 10]))
            .push(GuiElements::panel_title("Settings"))
            .push(scrollable(
                col!()
                    .spacing(20)
                    .push(config_dir_col)
                    .push(temp_dir_col)
                    .push(default_dir_col)
                    .push(
                        row!()
                            .spacing(10)
                            .push(
                                col!()
                                    .width(Length::Fill)
                                    .spacing(5)
                                    .push(text("Theme"))
                                    .push(
                                        pick_list(
                                            theme.variants(),
                                            Some(self.theme.clone()),
                                            SettingsMessage::ThemeChanged,
                                        )
                                        .width(Length::Fill),
                                    ),
                            )
                            .push(
                                col!()
                                    .width(Length::Fill)
                                    .spacing(5)
                                    .push(text("List View Layout"))
                                    .push(
                                        pick_list(
                                            ListLayout::variants(),
                                            Some::<String>(self.list_layout.clone().into()),
                                            SettingsMessage::ListLayoutChanged,
                                        )
                                        .width(Length::Fill),
                                    ),
                            )
                            .push(
                                col!()
                                    .width(Length::Fill)
                                    .spacing(5)
                                    .push(text("New Download Position"))
                                    .push(
                                        pick_list(
                                            vec!["First".to_string(), "Last".to_string()],
                                            Some(self.new_download_pos.clone()),
                                            SettingsMessage::NewDownloadPositionChanged,
                                        )
                                        .width(Length::Fill),
                                    ),
                            ),
                    )
                    .push(
                        container(
                            col!().spacing(5).push(
                                row![
                                    col![
                                        text(format!("Threads : {}", self.threads)),
                                        slider(2..=8, self.threads, |threads| {
                                            SettingsMessage::ThreadsChanged(threads)
                                        })
                                        .width(iced::Length::Fill)
                                    ]
                                    .width(Length::Fill),
                                    col![
                                        text(format!("UI Scaling : {0:>1.2}", self.scaling)),
                                        tooltip(
                                            slider(0.70..=2.00, self.scaling, |scaling| {
                                                SettingsMessage::ScalingChanged(scaling)
                                            })
                                            .step(0.01)
                                            .width(iced::Length::Fill),
                                            text("Resize window if not applied properly").size(12),
                                            tooltip::Position::Top
                                        )
                                        .style(AtomStyleContainer::ToolTipContainer)
                                        .padding(10)
                                        .gap(5),
                                    ]
                                    .width(Length::Fill),
                                    col![
                                        text(format!(
                                            "Font Size : {0:>2.0}",
                                            self.font_size.floor()
                                        )),
                                        slider(12.0..=28.0, self.font_size, |font_size| {
                                            SettingsMessage::TextSizeChanged(font_size)
                                        })
                                        .step(1.0)
                                        .width(iced::Length::Fill),
                                    ]
                                    .width(Length::Fill)
                                ]
                                .align_items(iced::Alignment::Center)
                                .spacing(30),
                            ),
                        )
                        .padding(20)
                        .style(AtomStyleContainer::ListContainer),
                    )
                    .push(options_row)
                    .push(buttons_row)
                    .width(iced::Length::Fill),
            ));

        container(settings_col)
            .style(AtomStyleContainer::ListContainer)
            .padding(Padding::from([0, 10, 10, 10]))
            .into()
    }
}
