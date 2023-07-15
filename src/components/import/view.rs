use super::AtomImport;
use iced::{
    widget::{column as col, container, row, text, text_input, toggler},
    Element, Padding, Renderer,
};

use crate::{
    // styles::style::{AtomInputDisabled, AtomToggler},
    font::{icon, CustomFont::IcoFont},
    messages::{ImportMessage, Message},
    style::{AtomStyleContainer, AtomStyleInput, Theme},
    utils::helpers::{atom_button, ButtonType, ATOM_INPUT_DEFAULT_PADDING},
};

impl AtomImport {
    pub fn view(&self) -> Element<'static, Message, Renderer<Theme>> {
        let mut start_download_btn = atom_button(
            ButtonType::IconWithText,
            vec![icon('\u{eee5}', IcoFont), text("start download")],
        );

        if !self.import_file.is_empty() {
            start_download_btn =
                start_download_btn.on_press(Message::StartImportDownload(self.clone()));
        }

        container(
            col!()
                .spacing(20)
                .padding(Padding::from([0, 10, 10, 10]))
                .push(
                    container(text("Import Links File"))
                        .style(AtomStyleContainer::LogoContainer)
                        .padding(Padding::from([10, 30, 10, 30])),
                )
                .push(
                    col!().spacing(5).push(text("Select File")).push(
                        row!()
                            .spacing(10)
                            .align_items(iced::Alignment::Center)
                            .push(
                                text_input("selected file will appear here", &self.import_file)
                                    .on_input(|_| Message::Ignore)
                                    .width(iced::Length::Fill)
                                    .padding(ATOM_INPUT_DEFAULT_PADDING),
                            )
                            .push(
                                atom_button(
                                    ButtonType::IconWithText,
                                    vec![icon('\u{ef13}', IcoFont), text("browse")],
                                )
                                .on_press(Message::Import(ImportMessage::ImportFileClicked)),
                            ),
                    ),
                )
                .push(
                    col!().spacing(5).push(text("Select Download Folder")).push(
                        row!()
                            .spacing(10)
                            .align_items(iced::Alignment::Center)
                            .push(
                                text_input("selected folder will appear here", &self.download_path)
                                    // .on_input(|_| Message::Null)
                                    .style(AtomStyleInput::Disabled)
                                    .width(iced::Length::Fill)
                                    .padding(ATOM_INPUT_DEFAULT_PADDING),
                            )
                            .push(
                                atom_button(
                                    ButtonType::IconWithText,
                                    vec![icon('\u{ef13}', IcoFont), text("browse")],
                                )
                                .on_press(Message::Import(
                                    ImportMessage::DownloadFolderSelectClicked,
                                )),
                            ),
                    ),
                )
                .push(
                    col!().push(
                        toggler(
                            Some("Download Sequentially".to_string()),
                            self.is_sequential,
                            |checked| Message::Import(ImportMessage::DownloadTypeToggled(checked)),
                        )
                        .spacing(10)
                        .text_alignment(iced::alignment::Horizontal::Left)
                        .width(iced::Length::Shrink),
                    ),
                )
                .push(
                    row!().spacing(20).push(start_download_btn).push(
                        atom_button(
                            ButtonType::IconWithText,
                            vec![icon('\u{eede}', IcoFont), text("cancel")],
                        )
                        .on_press(Message::GotoHomePage),
                    ),
                )
                .width(iced::Length::Fill),
        )
        .padding(Padding::from([0, 10, 10, 10]))
        .style(AtomStyleContainer::ListContainer)
        .into()
    }
}
