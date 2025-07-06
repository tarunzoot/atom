use super::AtomDownloadForm;
use crate::{
    components::settings::AtomSettings,
    elements::GuiElements,
    font::{ICOFONT, SYMBOLS},
    icons,
    messages::DownloadFormMessage,
    style::{
        button::AtomStyleButton, container::AtomStyleContainer, input::AtomStyleInput,
        AtomStyleText, AtomTheme,
    },
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    widget::{
        button, column as col, container, horizontal_space, mouse_area, row, text, text_input,
        vertical_space,
    },
    window::Id,
    Alignment, Element,
    Length::{Fill, FillPortion, Fixed, Shrink},
    Padding,
};

impl AtomDownloadForm {
    fn headers_view(&self) -> Element<DownloadFormMessage, AtomTheme> {
        let text_size = 12;
        self.headers
            .iter()
            .fold(
                col!().spacing(2).align_x(iced::Alignment::Center),
                |column, header| {
                    column.push(
                        container(
                            row![
                                icons::grip3()
                                    .class(AtomStyleText::Dimmed)
                                    .size(text_size - 2),
                                text(header.0.to_string()).width(Fill).size(text_size),
                                self.vertical_line(),
                                text_input("header value here...", header.1)
                                    .on_input(|value| {
                                        DownloadFormMessage::EditHeaderValue(
                                            header.0.to_string(),
                                            value,
                                        )
                                    })
                                    .icon(GuiElements::text_input_icon(
                                        '\u{f040}',
                                        SYMBOLS,
                                        text_size - 2
                                    ))
                                    .class(AtomStyleInput::Dimmed)
                                    .size(text_size)
                                    .width(FillPortion(2)),
                                self.vertical_line(),
                                GuiElements::round_button(icons::trash_bin_open())
                                    .on_press(DownloadFormMessage::DeleteHeader(
                                        header.0.to_string(),
                                    ))
                                    .width(Shrink)
                            ]
                            .padding(Padding::from([5, 10]))
                            .spacing(10)
                            .align_y(Alignment::Center),
                        )
                        .class(AtomStyleContainer::ListItemContainer),
                    )
                },
            )
            .into()
    }

    fn toggles_view(&self) -> Element<DownloadFormMessage, AtomTheme> {
        let sequential_tooltip_text = "Switch modes only if you are certain that the server supports the selected download method; otherwise, the download may fail.";

        let sequential_toggler = GuiElements::toggle(
            self.sequential,
            DownloadFormMessage::DownloadSequentially,
            "Download Sequentially",
        );

        let mut toggles = row!().spacing(20).width(Fill).align_y(Alignment::Center);

        if self.sequential {
            toggles = toggles.push(
                col![GuiElements::tooltip_top(
                    sequential_toggler,
                    sequential_tooltip_text,
                )]
                .width(Fill)
                .align_x(Alignment::Start),
            );
        } else {
            toggles = toggles.push(
                col![sequential_toggler]
                    .width(Fill)
                    .align_x(Alignment::Start),
            );
        }

        let auto_open_toggle = GuiElements::tooltip_top(
            GuiElements::toggle(self.auto_open, DownloadFormMessage::AutoOpen, "Auto Open"),
            "Automatically opens the file upon completion of the download.",
        );

        let auto_referer_toggle = GuiElements::tooltip_top(
            GuiElements::toggle(
                self.auto_referer,
                DownloadFormMessage::AutoReferer,
                "Auto Referer",
            ),
            "Automatically includes the Referer header in the request.",
        );

        if self.is_valid_url {
            toggles = toggles
                .push(
                    col![auto_open_toggle]
                        .width(Fill)
                        .align_x(Alignment::Center),
                )
                .push(
                    col![auto_referer_toggle]
                        .width(Fill)
                        .align_x(Alignment::End),
                )
        }

        toggles.into()
    }

    fn vertical_line(&self) -> Element<DownloadFormMessage, AtomTheme> {
        col![
            container(vertical_space().height(Fixed(30.0)).width(Fixed(1.0)),)
                .class(AtomStyleContainer::ListItemContainer)
                .width(Fixed(2.0))
        ]
        .align_x(iced::Alignment::Center)
        .width(Shrink)
        .into()
    }

    pub fn view(
        &self,
        settings: &AtomSettings,
        downloads_count: usize,
        window_id: Option<Id>,
    ) -> Element<DownloadFormMessage, AtomTheme> {
        let mut download_btn = GuiElements::primary_button(icons::cloud_download(), "download");
        let mut download_later_btn =
            GuiElements::primary_button(icons::clock_alt(), "download later");

        if self.is_valid_url && !self.file_name.is_empty() {
            download_btn = download_btn.on_press(DownloadFormMessage::AddNewDownload);
            download_later_btn = download_later_btn.on_press(DownloadFormMessage::DownloadLater);
        }

        let headers = self.headers_view();
        let toggles = self.toggles_view();

        let mut buttons_row = row!()
            .spacing(20)
            .width(Fill)
            .push(download_btn)
            .push(download_later_btn);

        if downloads_count > 0 || window_id.is_some() {
            buttons_row = buttons_row.push(
                GuiElements::primary_button(icons::close_circled(), "cancel")
                    .on_press(DownloadFormMessage::ClosePane),
            );
        }

        let url_input = col!().spacing(5).push(text("URL")).push(
            text_input("e.g: https://www.example.org/file.mp4", &self.url)
                .icon(GuiElements::text_input_icon('\u{ef71}', ICOFONT, 12))
                .on_input(DownloadFormMessage::UrlChange)
                .padding(ATOM_INPUT_DEFAULT_PADDING),
        );

        let file_path_input = col!().spacing(5).push(text("File Path")).push(
            row![
                text_input("e.g: file.mp4", &self.file_name)
                    .class(AtomStyleInput::Disabled)
                    .padding(ATOM_INPUT_DEFAULT_PADDING),
                GuiElements::primary_button(icons::harddisk(), "save as")
                    .on_press(DownloadFormMessage::BrowseSaveAsFolder)
                    .padding(Padding::from([7, 15]))
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        );

        let import_tooltip_text = "Import headers from a file.\nFile format is: HeaderName: HeaderValue\nFor e.g.:\nContent-Type: text/html\nContent-Length: 123456789";
        let import_headers_tooltip = GuiElements::tooltip_top(
            GuiElements::primary_button(icons::down_zigzag(), "import headers")
                .on_press(DownloadFormMessage::ImportHeaders),
            import_tooltip_text,
        );

        let headers_list = row![
            text_input("header name here...", &self.header_name)
                .on_input(DownloadFormMessage::AddHeaderName)
                .padding(ATOM_INPUT_DEFAULT_PADDING),
            text_input("header value here ...", &self.header_value)
                .on_input(DownloadFormMessage::AddHeaderValue)
                .padding(ATOM_INPUT_DEFAULT_PADDING),
            GuiElements::primary_button(icons::plus(), "Add")
                .on_press(DownloadFormMessage::AddHeader)
                .padding(Padding::from([7, 15])),
            self.vertical_line(),
            import_headers_tooltip
        ]
        .align_y(Alignment::Center)
        .spacing(10);

        let mut headers_container = container(GuiElements::scrollbar(
            if !self.headers.is_empty() {
                headers
            } else {
                col![text("No additional headers").width(Shrink)]
                    .align_x(Alignment::Center)
                    .width(Fill)
                    .into()
            },
            settings.scrollbars_visible,
        ))
        .padding(15)
        .width(Fill)
        .align_y(if self.headers.is_empty() {
            Alignment::Center
        } else {
            Alignment::Start
        })
        .class(AtomStyleContainer::ListContainer);

        if window_id.is_none() {
            headers_container = headers_container.max_height(200);
        } else {
            headers_container = headers_container.height(200);
        }

        let mut page_title = row![GuiElements::panel_title("Add New Download").into()];
        if window_id.is_some() {
            page_title = page_title.push(horizontal_space().width(Fill)).push(
                button(icons::minus())
                    .padding(14)
                    .class(AtomStyleButton::HeaderButtons)
                    .on_press(DownloadFormMessage::Minimize),
            );
        }

        let mut mouse_area_title = mouse_area(page_title);

        if window_id.is_some() {
            mouse_area_title = mouse_area_title
                .on_enter(DownloadFormMessage::MouseOverHeading)
                .on_exit(DownloadFormMessage::MouseAwayFromHeading);
        };

        container(
            col!()
                .spacing(20)
                .padding(Padding::new(10.0).top(0))
                .push(mouse_area_title)
                .push(
                    GuiElements::scrollbar(
                        col![
                            url_input,
                            file_path_input,
                            col![text("Additional Headers").width(Fill), headers_list].spacing(5),
                            headers_container,
                            container(col!().push(toggles))
                                .width(Fill)
                                .padding(20)
                                .class(AtomStyleContainer::ListContainer),
                            buttons_row,
                        ]
                        .height(Shrink)
                        .spacing(20)
                        .height(Shrink)
                        .width(Fill),
                        settings.scrollbars_visible,
                    )
                    .height(Shrink),
                ),
        )
        .padding(
            Padding::new(10.0)
                .top(0)
                .bottom(if window_id.is_none() { 10 } else { 0 }),
        )
        .height(if window_id.is_none() { Shrink } else { Fill })
        .class(if window_id.is_some() {
            AtomStyleContainer::MainContainer
        } else {
            AtomStyleContainer::ListContainer
        })
        .into()
    }
}
