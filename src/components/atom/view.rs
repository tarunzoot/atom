use super::{Atom, View};
use crate::{
    components::{delete_downloads, download::AtomDownload, keybindings, listview_header},
    font::icon,
    messages::{DownloadsFilterListMessage, Message},
    style::{AtomStyleContainer, Theme},
};
use iced::{
    widget::{column as col, container, row, scrollable, scrollable::Properties, text},
    Alignment, Element, Length, Padding, Renderer,
};

type DownloadTuple<'a> = (&'a usize, &'a AtomDownload);

impl<'a> Atom<'a> {
    fn filter_downloads_view(&self) -> Element<'a, Message, Renderer<Theme>> {
        let deleted_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| f.1.is_deleted);
        let all_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| {
                if self.titlebar.search_text.is_empty() {
                    !f.1.is_deleted
                } else {
                    !f.1.is_deleted
                        && f.1
                            .get_file_name()
                            .to_lowercase()
                            .contains(&self.titlebar.search_text)
                }
            });
        let downloading_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| f.1.is_downloading && !f.1.is_deleted);
        let paused_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| {
                !f.1.is_downloading() && !f.1.is_downloaded() && !f.1.is_deleted
            });
        let finished_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| f.1.is_downloaded() && !f.1.is_deleted);

        let filtered_downloads = match &self.filter_type {
            DownloadsFilterListMessage::Downloading => {
                self.downloads.iter().filter(downloading_filter)
            }
            DownloadsFilterListMessage::Paused => self.downloads.iter().filter(paused_filter),
            DownloadsFilterListMessage::Finished => self.downloads.iter().filter(finished_filter),
            DownloadsFilterListMessage::Deleted => self.downloads.iter().filter(deleted_filter),
            DownloadsFilterListMessage::All => self.downloads.iter().filter(all_filter),
        };

        let filtered_content = scrollable(filtered_downloads.fold(
            col!().padding(1).spacing(0),
            |column, (&index, download)| column.push(download.view(index)),
        ));

        if self.downloads.is_empty()
            && !matches!(self.filter_type, DownloadsFilterListMessage::Deleted)
        {
            self.download_form.view()
        } else {
            container(
                col!()
                    .spacing(10)
                    .push(self.filters.view(&self.sidebar.active))
                    .push(
                        container(
                            col!()
                                .spacing(0)
                                .push(
                                    col!().push(listview_header::view()).push(
                                        container(text(" ").size(0))
                                            .height(iced::Length::Fixed(1.0))
                                            .width(iced::Length::Fill)
                                            .style(AtomStyleContainer::LogoContainer),
                                    ),
                                )
                                .push(filtered_content.vertical_scroll(
                                    Properties::new().margin(0).scroller_width(0).width(0),
                                )),
                        )
                        .style(AtomStyleContainer::ListHeaderContainer),
                    ),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(iced::alignment::Vertical::Top)
            .into()
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message, iced::Renderer<Theme>> {
        if !self.instance.as_ref().unwrap().is_single() {
            let main_row = col!()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(0)
                .align_items(iced::Alignment::Center)
                .push(self.titlebar.view(&self.settings))
                .push(
                    container(
                        row!()
                            .align_items(iced::Alignment::Center)
                            .spacing(10)
                            .push(icon('\u{ef4e}', crate::font::CustomFont::IcoFont))
                            .push(text("Another instance of application is already running!")),
                    )
                    .padding(20)
                    .center_x()
                    .center_y()
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill),
                );

            return container(main_row).width(Length::Fill).into();
        }

        let view = match self.view {
            View::Import => self.import.view(),
            View::DeleteConfirm => delete_downloads::view(),
            View::Downloads => self.filter_downloads_view(),
            View::NewDownloadForm => self.download_form.view(),
            View::Settings => self.settings.view(),
            View::Shortcuts => keybindings::view(),
        };

        let mut items_row = row!()
            .push(
                col!().align_items(Alignment::Center).push(
                    container(self.sidebar.view())
                        .padding(Padding::from([20, 15]))
                        .height(iced::Length::Fill),
                ),
            )
            .push(
                container(
                    container(
                        col!()
                            .push(view)
                            .height(Length::Fill)
                            .width(Length::FillPortion(1)),
                    )
                    .style(AtomStyleContainer::ListContainer),
                )
                .padding(Padding::from([
                    20,
                    if self.metadata.enabled { 0 } else { 15 },
                    20,
                    0,
                ]))
                .width(iced::Length::Fill),
            );

        if self.metadata.enabled {
            items_row = items_row.push(
                container(self.metadata.view())
                    .padding(Padding::from([20, 15]))
                    .height(iced::Length::Fill),
            );
        }

        let main_row = col!()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .align_items(iced::Alignment::Center)
            .push(self.titlebar.view(&self.settings))
            .push(items_row);

        container(main_row)
            .padding(1)
            .style(AtomStyleContainer::MainBorderedContainer)
            .width(Length::Fill)
            .into()
    }
}
