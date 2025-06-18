use super::{AtomSettings, ListLayout};
use crate::{
    elements::GuiElements,
    icons,
    messages::SettingsMessage,
    style::{container::AtomStyleContainer, input::AtomStyleInput, AtomTheme},
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    widget::{column as col, container, pick_list, row, slider, text, text_input},
    Alignment, Element,
    Length::{Fill, Fixed, Shrink},
    Padding, Renderer,
};

impl AtomSettings {
    pub fn view(
        &self,
        settings: &AtomSettings,
        theme: &AtomTheme,
    ) -> Element<SettingsMessage, AtomTheme, Renderer> {
        let config_dir_col = col!()
            .spacing(5)
            .push(text("Configuration Directory"))
            .push(
                row!()
                    .spacing(10)
                    .align_y(iced::Alignment::Center)
                    .push(
                        text_input("", &self.config_dir.to_string_lossy())
                            .width(iced::Fill)
                            .class(AtomStyleInput::Disabled)
                            .padding(ATOM_INPUT_DEFAULT_PADDING),
                    )
                    .push(
                        GuiElements::primary_button(vec![icons::folder(), text("open")])
                            .on_press(SettingsMessage::OpenConfigDir),
                    ),
            );

        let temp_dir_col = col!()
            .spacing(5)
            .push(text(
                "Temporary File Location (chunk files will be stored here)",
            ))
            .push(
                row!().spacing(10).align_y(Alignment::Center).push(
                    text_input("", &self.cache_dir.to_string_lossy())
                        .width(Fill)
                        .class(AtomStyleInput::Disabled)
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
                    .align_y(Alignment::Center)
                    .spacing(10)
                    .push(
                        text_input("", &self.downloads_dir)
                            .on_input(|_| SettingsMessage::BrowseDownloadsDirClicked)
                            .padding(ATOM_INPUT_DEFAULT_PADDING),
                    )
                    .push(
                        GuiElements::primary_button(vec![icons::envelope_open(), text("browse")])
                            .on_press(SettingsMessage::BrowseDownloadsDirClicked),
                    ),
            );

        let buttons_row = row![
            GuiElements::primary_button(vec![icons::harddisk(), text("save")])
                .on_press(SettingsMessage::SaveSettings(true)),
            GuiElements::primary_button(vec![icons::rotation(), text("reset")])
                .on_press(SettingsMessage::ResetSettings(false)),
            GuiElements::primary_button(vec![icons::trash_bin_open(), text("clear cache")])
                .on_press(SettingsMessage::ClearCacheClicked(false)),
            GuiElements::primary_button(vec![icons::close_circled(), text("cancel")])
                .on_press(SettingsMessage::ClosePane),
        ]
        .spacing(20);

        let toggles_text_size = settings.font_size - 1.0;

        let notification_toggler = GuiElements::toggle(
            self.show_notifications,
            SettingsMessage::NotificationToggle,
            "Download complete notification",
        )
        .text_size(toggles_text_size);

        let label = "The browser's captured download begins automatically without displaying a download form (disables auto open feature).";
        let auto_start_toggler = GuiElements::tooltip_top(
            GuiElements::toggle(
                self.auto_start_download,
                SettingsMessage::AutoStartDownloadToggle,
                "Auto start download",
            )
            .text_size(toggles_text_size),
            label,
        );

        let close_btn_toggler = GuiElements::toggle(
            self.minimize_to_tray,
            SettingsMessage::QuitActionToggle,
            "Minimize to tray",
        )
        .text_size(toggles_text_size);

        let maximized_toggler = GuiElements::toggle(
            self.maximized,
            SettingsMessage::MaximizedActionToggle,
            "Start Maximized ",
        )
        .text_size(toggles_text_size);

        let label = "Stretch the list view container to fill the available space(applies a background color)";
        let stretch_list_toggler = GuiElements::tooltip_top(
            GuiElements::toggle(
                self.stretch_list_view,
                SettingsMessage::ListBackgroundToggle,
                "Stretch List Background  ",
            )
            .text_size(toggles_text_size),
            label,
        );

        let always_show_metadata_toggler = GuiElements::tooltip_top(
            GuiElements::toggle(
                self.metadata_always_enabled,
                SettingsMessage::AlwaysShowPreviewPaneToggle,
                "Always Show Preview Panel",
            )
            .text_size(toggles_text_size),
            "Always keep the preview panel open",
        );

        let scrollbar_toggler = GuiElements::toggle(
            self.scrollbars_visible,
            SettingsMessage::ScrollbarsVisible,
            "Scrollbars Visible",
        )
        .text_size(toggles_text_size);

        let options_row = container(
            col![row![
                col![notification_toggler, auto_start_toggler, scrollbar_toggler]
                    .spacing(10)
                    .width(Fill)
                    .align_x(Alignment::Start),
                col![close_btn_toggler, maximized_toggler]
                    .spacing(10)
                    .width(Fill)
                    .align_x(Alignment::Center),
                col![stretch_list_toggler, always_show_metadata_toggler]
                    .spacing(10)
                    .width(Fill)
                    .align_x(Alignment::End),
            ]
            .spacing(10)
            .align_y(Alignment::Start)
            .width(Fill)]
            .spacing(10),
        )
        .width(Fill)
        .padding(20)
        .class(AtomStyleContainer::ListContainer);

        let settings_col = col!()
            .spacing(20)
            .padding(Padding::new(10.0).top(0))
            .push(GuiElements::panel_title("Settings"))
            .push(GuiElements::scrollbar(
                col!()
                    .spacing(20)
                    .push(config_dir_col)
                    .push(temp_dir_col)
                    .push(default_dir_col)
                    .push(
                        row!()
                            .spacing(10)
                            .push(
                                col!().width(Fill).spacing(5).push(text("Theme")).push(
                                    pick_list(
                                        theme.variants(),
                                        Some(self.theme.clone()),
                                        SettingsMessage::ThemeChanged,
                                    )
                                    .width(Fill),
                                ),
                            )
                            .push(
                                col!()
                                    .width(Fill)
                                    .spacing(5)
                                    .push(text("List View Layout"))
                                    .push(
                                        pick_list(
                                            ListLayout::variants(),
                                            Some::<String>(self.list_layout.clone().into()),
                                            SettingsMessage::ListLayoutChanged,
                                        )
                                        .width(Fill),
                                    ),
                            )
                            .push(
                                col!()
                                    .width(Fill)
                                    .spacing(5)
                                    .push(text("New Download Position"))
                                    .push(
                                        pick_list(
                                            vec!["First".to_string(), "Last".to_string()],
                                            Some(self.new_download_pos.clone()),
                                            SettingsMessage::NewDownloadPositionChanged,
                                        )
                                        .width(Fill),
                                    ),
                            ),
                    )
                    .push(
                        container(
                            col!().spacing(5).push(
                                row![
                                    col![
                                        row![text("Threads").width(Fill), text(self.threads)]
                                            .spacing(10),
                                        slider(2..=8, self.threads, |threads| {
                                            SettingsMessage::ThreadsChanged(threads)
                                        })
                                        .width(Fill)
                                    ]
                                    .spacing(5)
                                    .width(Fill),
                                    GuiElements::vertical_separator().into(),
                                    col![
                                        row![
                                            text("UI Scaling").width(Fill),
                                            text(format!("{0:>1.2}", self.scaling)).width(Shrink)
                                        ]
                                        .spacing(10),
                                        GuiElements::tooltip_bottom(
                                            slider(0.70..=2.00, self.scaling, |scaling| {
                                                SettingsMessage::ScalingChanged(scaling)
                                            })
                                            .step(0.01)
                                            .width(Fill),
                                            "Resize window if not applied properly"
                                        ),
                                    ]
                                    .spacing(5)
                                    .width(Fill),
                                    GuiElements::vertical_separator().into(),
                                    col![
                                        row![
                                            text("Font Size").width(Fill),
                                            text(self.font_size.floor()).width(Shrink)
                                        ]
                                        .spacing(10),
                                        slider(12.0..=28.0, self.font_size, |font_size| {
                                            SettingsMessage::TextSizeChanged(font_size)
                                        })
                                        .step(1.0)
                                        .width(Fill),
                                    ]
                                    .spacing(5)
                                    .width(Fill)
                                ]
                                .align_y(Alignment::Center)
                                .spacing(30),
                            ),
                        )
                        .padding(20)
                        .class(AtomStyleContainer::ListContainer),
                    )
                    .push(options_row)
                    .push(buttons_row)
                    .width(Fill),
                settings.scrollbars_visible,
            ));

        let settings_container = container(settings_col)
            .class(AtomStyleContainer::ListContainer)
            .padding(Padding::new(10.0).top(0))
            .into();

        if self.show_confirm_dialog {
            let action_btn = if self.reset_settings {
                GuiElements::primary_button(vec![icons::rotation(), text("reset")])
                    .width(Fixed(150.0))
                    .on_press(SettingsMessage::ResetSettings(true))
            } else {
                GuiElements::primary_button(vec![icons::trash_bin_open(), text("delete")])
                    .width(Fixed(150.0))
                    .on_press(SettingsMessage::ClearCacheClicked(true))
            };

            let cancel_btn =
                GuiElements::primary_button(vec![icons::close_circled(), text("cancel")])
                    .width(Fixed(150.0))
                    .on_press(SettingsMessage::HideDialog);

            GuiElements::modal(
                settings_container,
                col![
                    text(if self.reset_settings {
                        "The current settings will be restored to the initial state by this action."
                    } else {
                        "This will remove the cache directory, which contains partial downloads."
                    })
                    .size(14),
                    text("Are you sure you want to continue?").size(14)
                ]
                .spacing(5)
                .align_x(Alignment::Center),
                row!()
                    .spacing(10)
                    .align_y(Alignment::Center)
                    .push(action_btn)
                    .push(cancel_btn),
                SettingsMessage::HideDialog,
            )
        } else {
            settings_container
        }
    }
}
