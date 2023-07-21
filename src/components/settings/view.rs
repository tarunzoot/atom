use super::AtomSettings;
use crate::{
    font::{icon, CustomFont::IcoFont},
    gui::GuiElements,
    messages::SettingsMessage,
    style::{AtomStyleContainer, AtomStyleInput, Theme},
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    widget::{column as col, container, row, slider, text, text_input, toggler, tooltip},
    Element, Padding, Renderer,
};

impl AtomSettings {
    pub fn view(&self) -> Element<SettingsMessage, Renderer<Theme>> {
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
                "Default Download Location (directory where file will be moved after download)",
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

        let options_row = row!()
                    .align_items(iced_native::Alignment::Center)
                    .spacing(10)
                    .push(
                        toggler(
                            Some("Show download completion/error notification".into()),
                            self.show_notifications,
                            |checked| {
                                SettingsMessage::NotificationToggle(checked)
                            },
                        )
                        .spacing(10)
                        .text_alignment(iced::alignment::Horizontal::Left)
                        .width(iced::Length::Shrink),
                    )
                    .push(
                        toggler(
                            Some("Close button quits app".into()),
                            self.quit_action_closes_app,
                            SettingsMessage::QuitActionToggle,
                        )
                        .spacing(10)
                        .text_alignment(iced::alignment::Horizontal::Left)
                        .width(iced::Length::Shrink),
                    )
                    .push(
                        tooltip(toggler(
                            Some("Auto start download from browser".into()),
                            self.auto_start_download,
                            SettingsMessage::AutoStartDownloadToggle
                        )
                        .spacing(10)
                        .text_alignment(iced::alignment::Horizontal::Left)
                        .width(iced::Length::Shrink), "Adding downloads from browser auto starts the download without showing new download form", tooltip::Position::Top).style(AtomStyleContainer::ToolTipContainer).size(14).padding(5).gap(5)
                    );

        let settings_col = col!()
            .spacing(20)
            .padding(Padding::from([0, 10, 10, 10]))
            .push(
                container(text("Settings"))
                    .style(AtomStyleContainer::LogoContainer)
                    .padding(Padding::from([10, 30, 10, 30])),
            )
            .push(config_dir_col)
            .push(temp_dir_col)
            .push(default_dir_col)
            .push(
                col!()
                    .spacing(5)
                    .push(text(format!("Threads : {}", self.threads)))
                    .push(
                        slider(2..=8, self.threads, |threads| {
                            SettingsMessage::ThreadsChanged(threads)
                        })
                        .width(iced::Length::Fill),
                    ),
            )
            .push(options_row)
            .push(buttons_row)
            .width(iced::Length::Fill);

        container(settings_col)
            .style(AtomStyleContainer::ListContainer)
            .padding(Padding::from([0, 10, 10, 10]))
            .into()
    }
}
