use super::AtomImport;
use crate::{
    elements::GuiElements,
    font::{icon, CustomFont::IcoFont},
    messages::ImportMessage,
    style::{container::AtomStyleContainer, input::AtomStyleInput, AtomTheme},
    utils::helpers::ATOM_INPUT_DEFAULT_PADDING,
};
use iced::{
    widget::{column as col, container, row, text, text_input, toggler},
    Alignment, Element,
    Length::{Fill, Shrink},
    Padding,
};

impl AtomImport {
    pub fn view(&self) -> Element<ImportMessage, AtomTheme> {
        let mut start_download_btn =
            GuiElements::primary_button(vec![icon('\u{eee5}', IcoFont), text("start download")]);

        if !self.import_file.is_empty() {
            start_download_btn = start_download_btn.on_press(ImportMessage::StartImportDownload);
        }

        container(
            col!()
                .spacing(20)
                .padding(Padding::new(10.0).top(0))
                .push(GuiElements::panel_title("Import Links"))
                .push(
                    col!().spacing(5).push(text("Select File")).push(
                        row!()
                            .spacing(10)
                            .align_y(Alignment::Center)
                            .push(
                                text_input("selected file will appear here", &self.import_file)
                                    .on_input(|_| ImportMessage::Ignore)
                                    .width(Fill)
                                    .padding(ATOM_INPUT_DEFAULT_PADDING),
                            )
                            .push(
                                GuiElements::primary_button(vec![
                                    icon('\u{ef13}', IcoFont),
                                    text("browse"),
                                ])
                                .on_press(ImportMessage::ImportFileClicked),
                            ),
                    ),
                )
                .push(
                    col!().spacing(5).push(text("Select Download Folder")).push(
                        row!()
                            .spacing(10)
                            .align_y(Alignment::Center)
                            .push(
                                text_input("selected folder will appear here", &self.download_path)
                                    // .on_input(|_| Message::Null)
                                    .class(AtomStyleInput::Disabled)
                                    .width(Fill)
                                    .padding(ATOM_INPUT_DEFAULT_PADDING),
                            )
                            .push(
                                GuiElements::primary_button(vec![
                                    icon('\u{ef13}', IcoFont),
                                    text("browse"),
                                ])
                                .on_press(ImportMessage::DownloadFolderSelectClicked),
                            ),
                    ),
                )
                .push(
                    container(
                        col!().push(
                            toggler(self.is_sequential)
                                .label("Download Sequentially")
                                .on_toggle(ImportMessage::DownloadTypeToggled)
                                .spacing(10)
                                .text_alignment(iced::alignment::Horizontal::Left)
                                .width(Shrink),
                        ),
                    )
                    .padding(15)
                    .width(Fill)
                    .max_height(200)
                    .class(AtomStyleContainer::ListContainer),
                )
                .push(
                    row!().spacing(20).push(start_download_btn).push(
                        GuiElements::primary_button(vec![
                            icon('\u{eedd}', IcoFont),
                            text("cancel"),
                        ])
                        .on_press(ImportMessage::ClosePane),
                    ),
                )
                .width(iced::Length::Fill),
        )
        .padding(Padding::new(10.0).top(0))
        .class(AtomStyleContainer::ListContainer)
        .into()
    }
}
