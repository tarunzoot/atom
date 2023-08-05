use super::AtomDownloadForm;
use crate::{
    font::{icon, CustomFont},
    gui::GuiElements,
    messages::DownloadFormMessage,
    style::{AtomStyleContainer, AtomStyleInput, Theme},
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    widget::{
        column, container, row, scrollable, scrollable::Properties, text, text_input, toggler,
        tooltip,
    },
    Element, Length, Padding, Renderer,
};

impl AtomDownloadForm {
    pub fn view(&self) -> Element<DownloadFormMessage, Renderer<Theme>> {
        let mut download_btn = GuiElements::primary_button(vec![
            icon('\u{eee5}', CustomFont::IcoFont),
            text("download"),
        ]);

        if self.is_valid_url {
            download_btn = download_btn.on_press(DownloadFormMessage::AddNewDownload);
        }

        let headers = self.headers.iter().fold(
            column!().align_items(iced::Alignment::Center),
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
                                    .on_press(DownloadFormMessage::EditHeader(header.0.to_string()))
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
        );

        let mut toggles = row!().spacing(20).width(iced::Length::Fill).push(
            toggler(
                Some("Download Sequentially".to_string()),
                self.sequential,
                DownloadFormMessage::DownloadSequentially,
            )
            .spacing(10)
            .text_alignment(iced::alignment::Horizontal::Left)
            .width(iced::Length::Shrink),
        );
        if self.is_valid_url {
            toggles = toggles.push(
                toggler(
                    Some(
                        "Auto Referer (Automatically adds referer header to the request)"
                            .to_string(),
                    ),
                    self.auto_referer,
                    DownloadFormMessage::AutoReferer,
                )
                .spacing(10)
                .text_alignment(iced::alignment::Horizontal::Left)
                .width(iced::Length::Shrink),
            );
        }

        container(
            column!()
                .spacing(20)
                .padding(Padding::from([0, 10, 10, 10]))
                .push(
                    container(text("Add New Download"))
                        .style(AtomStyleContainer::LogoContainer)
                        .padding(Padding::from([10, 30, 10, 30])),
                )
                .push(
                    column!().spacing(5).push(text("URL")).push(
                        text_input("e.g: https://www.example.org/file.mp4", &self.url)
                            .on_input(DownloadFormMessage::UrlChange)
                            .padding(ATOM_INPUT_DEFAULT_PADDING),
                    ),
                )
                .push(
                    column!().spacing(5).push(text("File Path")).push(
                        row![
                            text_input("e.g: file.mp4", &self.file_name)
                                .style(AtomStyleInput::Disabled)
                                .padding(ATOM_INPUT_DEFAULT_PADDING),
                            GuiElements::primary_button(vec![
                                icon('\u{ee00}', CustomFont::IcoFont),
                                text("save as")
                            ],)
                            .on_press(DownloadFormMessage::BrowseSaveAsFolder)
                            .padding(Padding::from([7, 15]))
                        ]
                        .spacing(10)
                        .align_items(iced::Alignment::Center),
                    ),
                )
                .push(
                    column!()
                        .spacing(5)
                        .push(text("Additional Headers").width(Length::Fill))
                        .push(
                            row!()
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
                                .push(
                                    tooltip(
                                        GuiElements::primary_button(vec![icon(
                                            '\u{eabe}',
                                            CustomFont::IcoFont,
                                        ),text("import headers")])
                                        .on_press(DownloadFormMessage::ImportHeaders),
                                        "Import headers from a file.\nFile format is: HeaderName: HeaderValue\nFor e.g.:\nContent-Type: text/html\nContent-Length: 123456789",
                                        tooltip::Position::Top,
                                    )
                                    .gap(10)
                                    .size(12)
                                    .padding(10)
                                    .style(AtomStyleContainer::ToolTipContainer),
                                ),
                        ),
                )
                .push(
                    container(
                        scrollable(headers).direction(scrollable::Direction::Vertical(
                            Properties::new().margin(0).width(0).scroller_width(0),
                        )),
                    )
                    .padding(0)
                    .width(iced::Length::Fill)
                    .max_height(190)
                    .style(AtomStyleContainer::ListContainer),
                )
                .push(column!().push(toggles))
                .push(
                    row!()
                        .spacing(20)
                        .width(iced::Length::Fill)
                        .push(download_btn)
                        .push(
                            GuiElements::primary_button(vec![
                                icon('\u{eede}', CustomFont::IcoFont),
                                text("cancel"),
                            ])
                            .on_press(DownloadFormMessage::ClosePane),
                        ),
                )
                .height(iced::Length::Fill)
                .width(iced::Length::Fill),
        )
        .padding(Padding::from([0, 10, 10, 10]))
        .style(AtomStyleContainer::ListContainer)
        .into()
    }
}
