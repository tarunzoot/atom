use super::AtomDownloadForm;
use crate::{
    elements::GuiElements,
    font::{icon, CustomFont, ICOFONT, SYMBOLS},
    messages::DownloadFormMessage,
    style::{container::AtomStyleContainer, input::AtomStyleInput, AtomStyleText, AtomTheme},
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    widget::{
        column as col, container, row, scrollable, scrollable::Scrollbar, text, text_input,
        toggler, tooltip, tooltip::Position, vertical_space,
    },
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
                                icon('\u{ee57}', CustomFont::Symbols)
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
                                GuiElements::round_button('\u{ec53}')
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
        let mut toggles = row!().spacing(20).width(Fill).push(
            col!()
                .push(
                    toggler(self.sequential)
                        .label("Download Sequentially".to_string())
                        .on_toggle(DownloadFormMessage::DownloadSequentially)
                        .spacing(10)
                        .text_alignment(iced::alignment::Horizontal::Left)
                        .width(Fill),
                )
                .width(Fill)
                .align_x(iced::Alignment::Start),
        );
        if self.is_valid_url {
            toggles = toggles
                .push(
                    col!()
                        .push(
                            toggler(self.auto_open)
                                .label("Open file after download".to_string())
                                .on_toggle(DownloadFormMessage::AutoOpen)
                                .spacing(10)
                                .text_alignment(iced::alignment::Horizontal::Left)
                                .width(Shrink),
                        )
                        .width(Fill)
                        .align_x(Alignment::Center),
                )
                .push(
                    col!()
                        .push(
                            tooltip(
                                toggler(self.auto_referer)
                                    .label("Add Referer Header".to_string())
                                    .on_toggle(DownloadFormMessage::AutoReferer)
                                    .spacing(10)
                                    .text_alignment(iced::alignment::Horizontal::Left)
                                    .width(Shrink),
                                text("Automatically adds referer header to the request").size(12),
                                Position::Top,
                            )
                            .gap(10)
                            .padding(10)
                            .class(AtomStyleContainer::ToolTipContainer),
                        )
                        .width(Fill)
                        .align_x(Alignment::End),
                );
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

    pub fn view(&self, downloads_count: usize) -> Element<DownloadFormMessage, AtomTheme> {
        let mut download_btn = GuiElements::primary_button(vec![
            icon('\u{eee5}', CustomFont::IcoFont),
            text("download"),
        ]);

        if self.is_valid_url && !self.file_name.is_empty() {
            download_btn = download_btn.on_press(DownloadFormMessage::AddNewDownload);
        }

        let headers = self.headers_view();
        let toggles = self.toggles_view();

        let mut buttons_row = row!().spacing(20).width(Fill).push(download_btn);

        if downloads_count > 0 {
            buttons_row = buttons_row.push(
                GuiElements::primary_button(vec![
                    icon('\u{eedd}', CustomFont::IcoFont),
                    text("cancel"),
                ])
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
                GuiElements::primary_button(vec![
                    icon('\u{ef43}', CustomFont::IcoFont),
                    text("save as")
                ])
                .on_press(DownloadFormMessage::BrowseSaveAsFolder)
                .padding(Padding::from([7, 15]))
            ]
            .spacing(10)
            .align_y(Alignment::Center),
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
        .class(AtomStyleContainer::ToolTipContainer);

        let headers_list = row!()
            .align_y(Alignment::Center)
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
            .push(self.vertical_line())
            .push(import_headers_tooltip);

        container(
            col!()
                .spacing(20)
                .padding(Padding::new(10.0).top(0))
                .push(GuiElements::panel_title("Add New Download"))
                .push(
                    scrollable(
                        col!()
                            .height(Shrink)
                            .spacing(20)
                            .push(url_input)
                            .push(file_path_input)
                            .push(
                                col!()
                                    .spacing(5)
                                    .push(text("Additional Headers").width(Fill))
                                    .push(headers_list),
                            )
                            .push(
                                container(
                                    scrollable(if self.headers.len() > 0 {
                                        headers
                                    } else {
                                        col![text("No additional headers").width(Shrink)]
                                            .align_x(Alignment::Center)
                                            .width(Fill)
                                            .into()
                                    })
                                    .direction(
                                        scrollable::Direction::Vertical(
                                            Scrollbar::new().margin(0).width(0).scroller_width(0),
                                        ),
                                    ),
                                )
                                .padding(15)
                                .width(Fill)
                                .max_height(200)
                                .class(AtomStyleContainer::ListContainer),
                            )
                            .push(
                                container(col!().push(toggles))
                                    .width(Fill)
                                    .padding(20)
                                    .class(AtomStyleContainer::ListContainer),
                            )
                            .push(buttons_row)
                            .height(Shrink)
                            .width(Fill),
                    )
                    .height(Shrink)
                    .direction(scrollable::Direction::Vertical(
                        Scrollbar::new().margin(0).scroller_width(0).width(0),
                    )),
                ),
        )
        .padding(Padding::new(10.0).top(0))
        .height(Shrink)
        .class(AtomStyleContainer::ListContainer)
        .into()
    }
}
