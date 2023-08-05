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
        column, container, pick_list, row, scrollable, scrollable::Properties, text, text_input,
        toggler,
    },
    Element, Padding, Renderer,
};

impl AtomDownloadForm {
    pub fn view(&self) -> Element<DownloadFormMessage, Renderer<Theme>> {
        let http_headers = vec![
            "accept".to_string(),
            "accept_charset".to_string(),
            "accept_encoding".to_string(),
            "accept_language".to_string(),
            "accept_ranges".to_string(),
            "access_control_allow_credentials".to_string(),
            "access_control_allow_headers".to_string(),
            "access_control_allow_methods".to_string(),
            "access_control_allow_origin".to_string(),
            "access_control_expose_headers".to_string(),
            "access_control_max_age".to_string(),
            "access_control_request_headers".to_string(),
            "access_control_request_method".to_string(),
            "age".to_string(),
            "allow".to_string(),
            "alt_svc".to_string(),
            "authorization".to_string(),
            "cache_control".to_string(),
            "connection".to_string(),
            "content_disposition".to_string(),
            "content_encoding".to_string(),
            "content_language".to_string(),
            "content_length".to_string(),
            "content_location".to_string(),
            "content_range".to_string(),
            "content_security_policy".to_string(),
            "content_security_policy_report_only".to_string(),
            "content_type".to_string(),
            "cookie".to_string(),
            "date".to_string(),
            "dnt".to_string(),
            "etag".to_string(),
            "expect".to_string(),
            "expires".to_string(),
            "forwarded".to_string(),
            "from".to_string(),
            "host".to_string(),
            "if_match".to_string(),
            "if_modified_since".to_string(),
            "if_none_match".to_string(),
            "if_range".to_string(),
            "if_unmodified_since".to_string(),
            "last_modified".to_string(),
            "link".to_string(),
            "location".to_string(),
            "max_forwards".to_string(),
            "origin".to_string(),
            "pragma".to_string(),
            "proxy_authenticate".to_string(),
            "proxy_authorization".to_string(),
            "public_key_pins".to_string(),
            "public_key_pins_report_only".to_string(),
            "range".to_string(),
            "referer".to_string(),
            "referrer_policy".to_string(),
            "refresh".to_string(),
            "retry_after".to_string(),
            "sec_websocket_accept".to_string(),
            "sec_websocket_extensions".to_string(),
            "sec_websocket_key".to_string(),
            "sec_websocket_protocol".to_string(),
            "sec_websocket_version".to_string(),
            "server".to_string(),
            "set_cookie".to_string(),
            "strict_transport_security".to_string(),
            "te".to_string(),
            "trailer".to_string(),
            "transfer_encoding".to_string(),
            "upgrade".to_string(),
            "upgrade_insecure_requests".to_string(),
            "user_agent".to_string(),
            "vary".to_string(),
            "via".to_string(),
            "warning".to_string(),
            "www_authenticate".to_string(),
            "x_content_type_options".to_string(),
            "x_dns_prefetch_control".to_string(),
            "x_frame_options".to_string(),
            "x_xss_protection".to_string(),
        ];

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
                // .push(atom_special_button('\u{efd0}', "clear cache".to_string()).on_press(Message::Null))
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
                        .push(text("Additional Headers").width(iced::Length::Fill))
                        .push(
                            row!()
                                .align_items(iced::Alignment::Center)
                                .spacing(10)
                                .push(pick_list(
                                    http_headers,
                                    Some(self.header_name.to_string()),
                                    DownloadFormMessage::AddHeaderName,
                                ))
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
