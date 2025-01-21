use super::{Atom, View};
use crate::{
    components::{download::AtomDownload, keybindings, listview_header},
    font::icon,
    messages::{DownloadsListFilterMessage, Message},
    style::{container::AtomStyleContainer, AtomTheme},
};
use iced::{
    alignment::Vertical::Top,
    widget::{
        column as col, container, horizontal_space, row, scrollable, scrollable::Scrollbar, text,
        vertical_space,
    },
    Alignment, Element,
    Length::{Fill, FillPortion, Fixed, Shrink},
    Padding,
};

type DownloadTuple<'a> = (&'a usize, &'a AtomDownload);

impl<'a> Atom<'a> {
    fn filter_downloads_view(&self) -> Element<Message, AtomTheme> {
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

        let responsive = if self.settings.scaling <= 1.0 {
            self.window_dimensions.0 < 1281
                && (self.metadata.enabled || !self.settings.sidebar_collapsed)
        } else {
            self.window_dimensions.0 < 1087
                || (self.window_dimensions.0 < 1281
                    && (self.metadata.enabled || !self.settings.sidebar_collapsed))
        };

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
        downloads = downloads.padding(0);

        let filtered_content = scrollable(downloads);

        if self.downloads.is_empty()
            && !matches!(self.filter_type, DownloadsListFilterMessage::Deleted)
        {
            self.download_form
                .view(self.downloads.len())
                .map(Message::DownloadForm)
        } else {
            let download_state_filters_bar = self.download_state_filter_bar.view(
                &self.sidebar.active,
                &self.downloads,
                &self.settings.list_layout,
                responsive,
                !self.titlebar.search_text.is_empty(),
            );

            let downloads_list_col = match self.settings.list_layout {
                crate::components::settings::ListLayout::ListExtended => col!()
                    .spacing(10)
                    .align_x(Alignment::Center)
                    .push(download_state_filters_bar)
                    .push(
                        container(
                            filtered_content
                                .height(if self.settings.stretch_list_view {
                                    Fill
                                } else {
                                    Shrink
                                })
                                .direction(scrollable::Direction::Vertical(
                                    Scrollbar::new().margin(0).scroller_width(0).width(0),
                                )),
                        )
                        .center(Fill)
                        .height(Shrink)
                        .width(Fill)
                        .padding(if self.settings.stretch_list_view || count > 0 {
                            1
                        } else {
                            0
                        })
                        .class(AtomStyleContainer::ListHeaderContainer),
                    ),
                crate::components::settings::ListLayout::List => col!()
                    .spacing(10)
                    // .push(self.filters.view(&self.sidebar.active, &self.downloads))
                    .push(
                        container(
                            col!()
                                .spacing(0)
                                .push(download_state_filters_bar)
                                .push(
                                    container(vertical_space().height(Fixed(1.0)))
                                        .height(Fixed(1.0))
                                        .width(Fill),
                                )
                                .push(
                                    col!().push(listview_header::view(responsive)).push(
                                        container(text(" ").size(10))
                                            .height(Fixed(1.0))
                                            .width(Fill)
                                            .class(AtomStyleContainer::LogoContainer),
                                    ),
                                )
                                .push(
                                    filtered_content
                                        .height(if self.settings.stretch_list_view {
                                            Fill
                                        } else {
                                            Shrink
                                        })
                                        .direction(scrollable::Direction::Vertical(
                                            Scrollbar::new().margin(0).scroller_width(0).width(0),
                                        )),
                                ),
                        )
                        .padding(0)
                        .center(Fill)
                        .height(Shrink)
                        .class(AtomStyleContainer::ListHeaderContainer),
                        // .class(AtomStyleContainer::Transparent),
                    ),
            };

            container(downloads_list_col)
                .width(Fill)
                .height(Fill)
                .align_y(Top)
                .class(AtomStyleContainer::Transparent)
                .into()
        }
    }

    pub fn view(&self) -> Element<Message, AtomTheme> {
        let status_bar_text_icon_size = 10;

        if !self.instance.as_ref().unwrap().is_single() {
            let main_row = col!()
                .width(Fill)
                .height(Fill)
                .padding(0)
                .align_x(Alignment::Center)
                .push(self.titlebar.view(&self.settings).map(Message::TitleBar))
                .push(
                    container(
                        row!()
                            .align_y(Alignment::Center)
                            .spacing(10)
                            .push(icon('\u{ef4e}', crate::font::CustomFont::IcoFont))
                            .push(text("Another instance of application is already running!")),
                    )
                    .padding(20)
                    .center_x(Fill)
                    .center_y(Fill)
                    .width(Fill)
                    .height(Fill),
                );

            return container(main_row).width(Fill).into();
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
            .width(Fill)
            .push(
                col!().width(Shrink).align_x(Alignment::Center).push(
                    container(self.sidebar.view().map(Message::Sidebar))
                        .padding(Padding::from([20, 15]))
                        .height(Fill)
                        .width(Shrink),
                ),
            )
            .push(
                container(
                    container(col!().push(view).height(Fill).width(FillPortion(1)))
                        .width(Shrink)
                        .height(Shrink),
                )
                .padding(
                    Padding::new(20.0)
                        .right(if self.metadata.enabled { 0 } else { 15 })
                        .left(0),
                )
                .width(Fill)
                .height(Shrink),
            );

        if self.metadata.enabled {
            items_row = items_row.push(
                container(self.metadata.view().map(Message::Metadata))
                    .padding(Padding::from([20, 15]))
                    .height(Fill)
                    .width(Shrink),
            );
        }

        let main_row = col![
            self.titlebar.view(&self.settings).map(Message::TitleBar),
            items_row,
            container(
                row![
                    icon('\u{ebf5}', crate::font::CustomFont::Symbols)
                        .size(status_bar_text_icon_size),
                    text(&self.status_bar_message).size(status_bar_text_icon_size),
                    horizontal_space().width(Fill),
                ]
                .align_y(Alignment::Center)
                .spacing(10)
            )
            .class(AtomStyleContainer::HeaderContainer)
            .padding(Padding::new(5.0).left(15).right(15))
        ]
        .width(Fill)
        .height(Fill)
        .padding(0)
        .align_x(Alignment::Center);

        container(main_row)
            .padding(1)
            .class(AtomStyleContainer::HeaderContainer)
            .width(Fill)
            .into()
    }
}
