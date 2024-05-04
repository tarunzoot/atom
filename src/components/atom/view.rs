use super::{Atom, View};
use crate::{
    components::{download::AtomDownload, keybindings, listview_header},
    font::icon,
    messages::{DownloadsListFilterMessage, Message},
    style::{container::AtomStyleContainer, Theme},
};
use iced::{
    widget::{
        column as col, container, row, scrollable, scrollable::Properties, text, vertical_space,
    },
    Alignment, Element, Length, Padding, Renderer,
};

type DownloadTuple<'a> = (&'a usize, &'a AtomDownload);

impl<'a> Atom<'a> {
    fn filter_downloads_view(&self) -> Element<Message, Theme, Renderer> {
        let deleted_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| f.1.deleted);
        let all_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| {
                if self.titlebar.search_text.is_empty() {
                    !f.1.deleted
                } else {
                    !f.1.deleted
                        && f.1
                            .get_file_name()
                            .to_lowercase()
                            .contains(&self.titlebar.search_text)
                }
            });
        let downloading_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| f.1.downloading && !f.1.deleted);
        let paused_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| {
                !f.1.is_downloading() && !f.1.is_downloaded() && !f.1.deleted
            });
        let finished_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| f.1.is_downloaded() && !f.1.deleted);

        let filtered_downloads = match &self.filter_type {
            DownloadsListFilterMessage::Downloading => {
                self.downloads.iter().filter(downloading_filter)
            }
            DownloadsListFilterMessage::Paused => self.downloads.iter().filter(paused_filter),
            DownloadsListFilterMessage::Finished => self.downloads.iter().filter(finished_filter),
            DownloadsListFilterMessage::Deleted => self.downloads.iter().filter(deleted_filter),
            DownloadsListFilterMessage::All => self.downloads.iter().filter(all_filter),
        };

        let icons_only = (self.metadata.enabled || !self.settings.sidebar_collapsed)
            || self.settings.scaling >= 1.15;
        let responsive = (self.dimensions.0 < 1025 || self.dimensions.1 < 577) && icons_only;

        let mut count = 0;

        let mut downloads =
            filtered_downloads.fold(col!().spacing(0), |column, (index, download)| {
                count += 1;
                column.push(
                    download
                        .view(
                            &self.settings.list_layout,
                            self.settings.font_size,
                            responsive,
                        )
                        .map(|message| Message::Download(message, *index)),
                )
            });
        downloads = downloads.padding(if count > 0 { 1 } else { 0 });

        let filtered_content = scrollable(downloads);

        if self.downloads.is_empty()
            && !matches!(self.filter_type, DownloadsListFilterMessage::Deleted)
        {
            self.download_form
                .view(self.downloads.len())
                .map(Message::DownloadForm)
        } else {
            let filters_view = self.filters.view(
                &self.sidebar.active,
                &self.downloads,
                &self.settings.list_layout,
                icons_only,
            );

            let downloads_list_col = match self.settings.list_layout {
                crate::components::settings::ListLayout::ListExtended => {
                    col!().spacing(10).push(filters_view).push(
                        container(
                            filtered_content
                                .height(if self.settings.stretch_list_view {
                                    Length::Fill
                                } else {
                                    Length::Shrink
                                })
                                .direction(scrollable::Direction::Vertical(
                                    Properties::new().margin(0).scroller_width(0).width(0),
                                )),
                        )
                        .width(Length::Fill)
                        .padding(0)
                        .style(AtomStyleContainer::ListHeaderContainer),
                    )
                }
                crate::components::settings::ListLayout::List => col!()
                    .spacing(10)
                    // .push(self.filters.view(&self.sidebar.active, &self.downloads))
                    .push(
                        container(
                            col!()
                                .spacing(0)
                                .push(filters_view)
                                .push(
                                    container(vertical_space().height(Length::Fixed(1.0)))
                                        .height(Length::Fixed(1.0))
                                        .width(Length::Fill),
                                )
                                .push(
                                    col!().push(listview_header::view(responsive)).push(
                                        container(text(" ").size(10))
                                            .height(iced::Length::Fixed(1.0))
                                            .width(iced::Length::Fill)
                                            .style(AtomStyleContainer::LogoContainer),
                                    ),
                                )
                                .push(
                                    filtered_content
                                        .height(if self.settings.stretch_list_view {
                                            Length::Fill
                                        } else {
                                            Length::Shrink
                                        })
                                        .direction(scrollable::Direction::Vertical(
                                            Properties::new().margin(0).scroller_width(0).width(0),
                                        )),
                                ),
                        )
                        .height(Length::Shrink)
                        .style(AtomStyleContainer::ListHeaderContainer),
                        // .style(AtomStyleContainer::Transparent),
                    ),
            };

            container(downloads_list_col)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(iced::alignment::Vertical::Top)
                .style(AtomStyleContainer::Transparent)
                .into()
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message, Theme, Renderer> {
        if !self.instance.as_ref().unwrap().is_single() {
            let main_row = col!()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(0)
                .align_items(iced::Alignment::Center)
                .push(self.titlebar.view(&self.settings).map(Message::TitleBar))
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
            View::Import => self.import.view().map(Message::Import),
            View::Downloads => self.filter_downloads_view(),
            View::NewDownloadForm => self
                .download_form
                .view(self.downloads.len())
                .map(Message::DownloadForm),
            View::Settings => self
                .phantom_settings
                .view(&self.theme)
                .map(Message::Settings),
            View::Shortcuts => keybindings::view(),
        };

        let mut items_row = row!()
            .width(Length::Fill)
            .push(
                col!()
                    .width(iced::Length::Shrink)
                    .align_items(Alignment::Center)
                    .push(
                        container(self.sidebar.view().map(Message::Sidebar))
                            .padding(Padding::from([20, 15]))
                            .height(iced::Length::Fill)
                            .width(Length::Shrink),
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
                    .width(Length::Shrink)
                    .height(Length::Shrink),
                )
                .padding(Padding::from([
                    20,
                    if self.metadata.enabled { 0 } else { 15 },
                    20,
                    0,
                ]))
                .width(Length::Fill)
                .height(Length::Shrink),
            );

        if self.metadata.enabled {
            items_row = items_row.push(
                container(self.metadata.view().map(Message::Metadata))
                    .padding(Padding::from([20, 15]))
                    .height(iced::Length::Fill)
                    .width(Length::Shrink),
            );
        }

        let main_row = col!()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .align_items(iced::Alignment::Center)
            .push(self.titlebar.view(&self.settings).map(Message::TitleBar))
            .push(items_row);

        container(main_row)
            .padding(1)
            .style(AtomStyleContainer::HeaderContainer)
            .width(Length::Fill)
            .into()
    }
}
