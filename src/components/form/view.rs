use super::AtomDownloadForm;
use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::DownloadFormMessage,
    style::{container::AtomStyleContainer, input::AtomStyleInput, Theme},
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    advanced::graphics::core::Element,
    widget::{
        column as col, container, row, scrollable, scrollable::Properties, text, text_input,
        toggler, tooltip, tooltip::Position,
    },
    Length, Padding, Renderer,
};

impl AtomDownloadForm {
    fn headers_view(&self) -> Element<DownloadFormMessage, Theme, Renderer> {
        self.headers
            .iter()
            .fold(
                col!().align_items(iced::Alignment::Center),
                |column, header| {
                    column.push(
                        container(
                            row!()
                                .padding(Padding::from([5, 10]))
                                .spacing(10)
                                .align_items(iced::Alignment::Center)
                                .push(
                                    text(header.0.to_string())
                                        .width(iced::Length::Fixed(300.0))
                                        .size(14),
                                )
                                .push(text(header.1).width(iced::Length::Fill).size(14))
                                .push(
                                    GuiElements::round_button('\u{ec55}')
                                        .on_press(DownloadFormMessage::EditHeader(
                                            header.0.to_string(),
                                        ))
                                        .width(iced::Length::Shrink),
                                )
                                .push(
                                    GuiElements::round_button('\u{ec53}')
                                        .on_press(DownloadFormMessage::DeleteHeader(
                                            header.0.to_string(),
                                        ))
                                        .width(iced::Length::Shrink),
                                ),
                        )
                        .style(AtomStyleContainer::ListItemContainer),
                    )
                },
            )
            .into()
    }

    fn toggles_view(&self) -> Element<DownloadFormMessage, Theme, Renderer> {
        let mut toggles = row!().spacing(20).width(iced::Length::Fill).push(
            col!()
                .push(
                    toggler(
                        Some("Download Sequentially".to_string()),
                        self.sequential,
                        DownloadFormMessage::DownloadSequentially,
                    )
                    .spacing(10)
                    .text_alignment(iced::alignment::Horizontal::Left)
                    .width(iced::Length::Fill),
                )
                .width(Length::Fill)
                .align_items(iced::Alignment::Start),
        );
        if self.is_valid_url {
            toggles = toggles
                .push(
                    col!()
                        .push(
                            toggler(
                                Some("Open file after download".to_string()),
                                self.auto_open,
                                DownloadFormMessage::AutoOpen,
                            )
                            .spacing(10)
                            .text_alignment(iced::alignment::Horizontal::Left)
                            .width(iced::Length::Shrink),
                        )
                        .width(Length::Fill)
                        .align_items(iced::Alignment::Center),
                )
                .push(
                    col!()
                        .push(
                            tooltip(
                                toggler(
                                    Some("Add Referer Header".to_string()),
                                    self.auto_referer,
                                    DownloadFormMessage::AutoReferer,
                                )
                                .spacing(10)
                                .text_alignment(iced::alignment::Horizontal::Left)
                                .width(iced::Length::Shrink),
                                text("Automatically adds referer header to the request").size(12),
                                Position::Top,
                            )
                            .gap(10)
                            .padding(10)
                            .style(AtomStyleContainer::ToolTipContainer),
                        )
                        .width(Length::Fill)
                        .align_items(iced::Alignment::End),
                );
        }
        toggles.into()
    }

    pub fn view(&self, downloads_count: usize) -> Element<DownloadFormMessage, Theme, Renderer> {
        let mut download_btn = GuiElements::primary_button(vec![
            icon('\u{eee5}', CustomFont::IcoFont),
            text("download"),
        ]);

        if self.is_valid_url && !self.file_name.is_empty() {
            download_btn = download_btn.on_press(DownloadFormMessage::AddNewDownload);
        }

        let headers = self.headers_view();
        let toggles = self.toggles_view();

        let mut buttons_row = row!()
            .spacing(20)
            .width(iced::Length::Fill)
            .push(download_btn);

        if downloads_count > 0 {
            buttons_row = buttons_row.push(
                GuiElements::primary_button(vec![
                    icon('\u{eede}', CustomFont::IcoFont),
                    text("cancel"),
                ])
                .on_press(DownloadFormMessage::ClosePane),
            );
        }

        let url_input = col!().spacing(5).push(text("URL")).push(
            text_input("e.g: https://www.example.org/file.mp4", &self.url)
                .on_input(DownloadFormMessage::UrlChange)
                .padding(ATOM_INPUT_DEFAULT_PADDING),
        );

        let file_path_input = col!().spacing(5).push(text("File Path")).push(
            row![
                text_input("e.g: file.mp4", &self.file_name)
                    .style(AtomStyleInput::Disabled)
                    .padding(ATOM_INPUT_DEFAULT_PADDING),
                GuiElements::primary_button(vec![
                    icon('\u{ef43}', CustomFont::IcoFont),
                    text("save as")
                ],)
                .on_press(DownloadFormMessage::BrowseSaveAsFolder)
                .padding(Padding::from([7, 15]))
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
        );

        let import_headers_tooltip = tooltip(
            GuiElements::primary_button(vec![
                icon('\u{eabe}', CustomFont::IcoFont),
                text("import headers"),
            ])
            .on_press(DownloadFormMessage::ImportHeaders),
            text("Import headers from a file.\nFile format is: HeaderName: HeaderValue\nFor e.g.:\nContent-Type: text/html\nContent-Length: 123456789").size(12),
            tooltip::Position::Top,
        )
        .gap(10)
        .padding(10)
        .style(AtomStyleContainer::ToolTipContainer);

        let headers_list = row!()
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .push(
                text_input("header name here...", &self.header_name)
                    .on_input(DownloadFormMessage::AddHeaderName)
                    .padding(ATOM_INPUT_DEFAULT_PADDING),
            )
            .push(
                text_input("header value here ...", &self.header_value)
                    .on_input(DownloadFormMessage::AddHeaderValue)
                    .padding(ATOM_INPUT_DEFAULT_PADDING),
            )
            .push(
                GuiElements::primary_button(vec![
                    icon('\u{efc2}', CustomFont::IcoFont),
                    text("Add"),
                ])
                .on_press(DownloadFormMessage::AddHeader)
                .padding(Padding::from([7, 15])),
            )
            .push(text(" or "))
            .push(import_headers_tooltip);

        container(
            col!()
                .spacing(20)
                .padding(Padding::from([0, 10, 10, 10]))
                .push(
                    container(text("Add New Download"))
                        .style(AtomStyleContainer::LogoContainer)
                        .padding(Padding::from([10, 30, 10, 30])),
                )
                .push(
                    scrollable(
                        col!()
                            .height(Length::Shrink)
                            .spacing(20)
                            .push(url_input)
                            .push(file_path_input)
                            .push(
                                col!()
                                    .spacing(5)
                                    .push(text("Additional Headers").width(Length::Fill))
                                    .push(headers_list),
                            )
                            .push(
                                container(scrollable(headers).direction(
                                    scrollable::Direction::Vertical(
                                        Properties::new().margin(0).width(0).scroller_width(0),
                                    ),
                                ))
                                .padding(0)
                                .width(iced::Length::Fill)
                                .max_height(200)
                                .style(AtomStyleContainer::ListHeaderContainer),
                            )
                            .push(
                                container(col!().push(toggles))
                                    .width(Length::Fill)
                                    .padding(20)
                                    .style(AtomStyleContainer::ListContainer),
                            )
                            .push(buttons_row)
                            .height(iced::Length::Shrink)
                            .width(iced::Length::Fill),
                    )
                    .height(Length::Shrink)
                    .direction(scrollable::Direction::Vertical(
                        Properties::new().margin(0).scroller_width(0).width(0),
                    )),
                ),
        )
        .padding(Padding::from([0, 10, 10, 10]))
        .height(Length::Shrink)
        .style(AtomStyleContainer::ListContainer)
        .into()
    }
}
