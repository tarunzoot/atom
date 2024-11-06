use super::{AtomSettings, ListLayout};
use crate::{
    elements::GuiElements,
    font::{
        icon,
        CustomFont::{self, IcoFont},
    },
    messages::SettingsMessage,
    style::{container::AtomStyleContainer, input::AtomStyleInput, AtomTheme},
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    widget::{
        column as col, container, pick_list, row, scrollable, scrollable::Scrollbar, slider, text,
        text_input, toggler, tooltip, vertical_space,
    },
    Alignment, Element,
    Length::{Fill, Fixed, Shrink},
    Padding, Renderer,
};

impl AtomSettings {
    fn vertical_line(&self) -> Element<SettingsMessage, AtomTheme, Renderer> {
        col![
            container(vertical_space().height(Fixed(30.0)).width(Fixed(1.0)))
                .class(AtomStyleContainer::ListItemContainer)
                .width(Fixed(1.0))
        ]
        .align_x(iced::Alignment::Center)
        .width(Shrink)
        .into()
    }

    pub fn view(&self, theme: &AtomTheme) -> Element<SettingsMessage, AtomTheme, Renderer> {
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
                        GuiElements::primary_button(vec![
                            icon('\u{ef13}', IcoFont),
                            text("browse"),
                        ])
                        .on_press(SettingsMessage::BrowseDownloadsDirClicked),
                    ),
            );

        let buttons_row = row![
            GuiElements::primary_button(vec![icon('\u{ef43}', IcoFont), text("save")])
                .on_press(SettingsMessage::SaveSettings(true)),
            GuiElements::primary_button(vec![icon('\u{ec80}', IcoFont), text("reset")])
                .on_press(SettingsMessage::ResetSettings(false)),
            GuiElements::primary_button(vec![icon('\u{ec53}', IcoFont), text("clear cache")])
                .on_press(SettingsMessage::ClearCacheClicked(false)),
            GuiElements::primary_button(vec![icon('\u{eedd}', IcoFont), text("cancel")])
                .on_press(SettingsMessage::ClosePane),
        ]
        .spacing(20);

        let toggles_text_size = self.font_size - 1.0;

        let notification_toggler = toggler(self.show_notifications)
            .label("Show download notification")
            .on_toggle(SettingsMessage::NotificationToggle)
            .spacing(10)
            .text_size(toggles_text_size)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(Shrink);

        let auto_start_toggler = tooltip(
            toggler(
                self.auto_start_download,
            )
            .label("Auto start download from browser")
            .on_toggle(SettingsMessage::AutoStartDownloadToggle)
            .spacing(10)
            .text_size(toggles_text_size)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(Shrink),
            text("Adding downloads from browser auto starts the download without showing new download form(disables auto open feature)").size(12),
            tooltip::Position::Top,
        )
        .class(AtomStyleContainer::ToolTipContainer)
        .padding(10)
        .gap(5);

        let close_btn_toggler = toggler(self.minimize_to_tray)
            .label("Minimize to tray")
            .on_toggle(SettingsMessage::QuitActionToggle)
            .spacing(10)
            .text_size(toggles_text_size)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(Shrink);

        let maximized_toggler = toggler(self.maximized)
            .label("Start Maximized ")
            .on_toggle(SettingsMessage::MaximizedActionToggle)
            .spacing(10)
            .text_size(toggles_text_size)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(Shrink);

        let stretch_list_toggler = tooltip(
            toggler(
                self.stretch_list_view
            ).label("Stretch List Background  ").on_toggle(SettingsMessage::ListBackgroundToggle)
            .spacing(10).text_size(toggles_text_size)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(Shrink),
            text("Stretch the list view container to fill the available space(applies a background color)").size(12),
            tooltip::Position::Top,
        )
        .gap(10)
        .padding(10)
        .class(AtomStyleContainer::ToolTipContainer);

        let new_download_notification_toggler = tooltip(
            toggler(self.new_download_notification)
                .label("New Download Notification")
                .on_toggle(SettingsMessage::NewDownloadNotificationToggle)
                .spacing(10)
                .text_size(toggles_text_size)
                .text_alignment(iced::alignment::Horizontal::Left)
                .width(Shrink),
            text("A notification is shown when a new download is added").size(12),
            tooltip::Position::Top,
        )
        .gap(10)
        .padding(10)
        .class(AtomStyleContainer::ToolTipContainer);

        let options_row = container(
            row!()
                .spacing(10)
                .align_y(Alignment::Center)
                .width(Fill)
                .push(
                    col!()
                        .spacing(10)
                        .width(Fill)
                        .align_x(Alignment::Start)
                        .push(notification_toggler)
                        .push(auto_start_toggler),
                )
                .push(
                    col!()
                        .spacing(10)
                        .width(Fill)
                        .align_x(Alignment::Center)
                        .push(close_btn_toggler)
                        .push(maximized_toggler),
                )
                .push(
                    col!()
                        .spacing(10)
                        .width(Fill)
                        .align_x(Alignment::End)
                        .push(stretch_list_toggler)
                        .push(new_download_notification_toggler),
                ),
        )
        .width(Fill)
        .padding(20)
        .class(AtomStyleContainer::ListContainer);

        let settings_col = col!()
            .spacing(20)
            .padding(Padding::new(10.0).top(0))
            .push(GuiElements::panel_title("Settings"))
            .push(
                scrollable(
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
                                        self.vertical_line(),
                                        col![
                                            row![
                                                text("UI Scaling").width(Fill),
                                                text(format!("{0:>1.2}", self.scaling))
                                                    .width(Shrink)
                                            ]
                                            .spacing(10),
                                            tooltip(
                                                slider(0.70..=2.00, self.scaling, |scaling| {
                                                    SettingsMessage::ScalingChanged(scaling)
                                                })
                                                .step(0.01)
                                                .width(Fill),
                                                text("Resize window if not applied properly")
                                                    .size(12),
                                                tooltip::Position::Bottom
                                            )
                                            .class(AtomStyleContainer::ToolTipContainer)
                                            .padding(10)
                                            .gap(5),
                                        ]
                                        .spacing(5)
                                        .width(Fill),
                                        self.vertical_line(),
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
                )
                .direction(scrollable::Direction::Vertical(
                    Scrollbar::new().margin(0).width(0).scroller_width(0),
                )),
            );

        let settings_container = container(settings_col)
            .class(AtomStyleContainer::ListContainer)
            .padding(Padding::new(10.0).top(0))
            .into();

        if self.show_confirm_dialog {
            let action_btn = if self.reset_settings {
                GuiElements::primary_button(vec![
                    icon('\u{ec80}', CustomFont::IcoFont),
                    text("reset"),
                ])
                .width(Fixed(150.0))
                .on_press(SettingsMessage::ResetSettings(true))
            } else {
                GuiElements::primary_button(vec![
                    icon('\u{ec53}', CustomFont::IcoFont),
                    text("delete"),
                ])
                .width(Fixed(150.0))
                .on_press(SettingsMessage::ClearCacheClicked(true))
            };

            let cancel_btn = GuiElements::primary_button(vec![
                icon('\u{eedd}', CustomFont::IcoFont),
                text("cancel"),
            ])
            .width(Fixed(150.0))
            .on_press(SettingsMessage::HideDialog);

            GuiElements::modal(
                settings_container,
                col![
                    text(if self.reset_settings {
                        "The current settings will be restored to the inital state by this action."
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
