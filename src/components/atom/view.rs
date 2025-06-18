use super::{Atom, View};
use crate::{
    components::{
        download::AtomDownload, keybindings, listview_header, sidebar::SideBarActiveButton,
    },
    elements::GuiElements,
    icons,
    messages::{DownloadsListFilterMessage, Message},
    style::{container::AtomStyleContainer, AtomTheme},
    utils::helpers::check_responsive_threshold,
};
use iced::{
    alignment::Vertical::Top,
    widget::{column as col, container, horizontal_space, row, text, vertical_space, Container},
    window::Id,
    Alignment, Element,
    Length::{Fill, FillPortion, Shrink},
    Padding,
};

type DownloadTuple<'a> = (&'a usize, &'a AtomDownload);

impl Atom<'_> {
    fn filter_downloads_view(&self) -> Element<Message, AtomTheme> {
        let failed_filter: Box<dyn Fn(&DownloadTuple) -> bool> =
            Box::new(|f: &(&usize, &AtomDownload)| !f.1.error.is_empty() && !f.1.deleted);
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
            DownloadsListFilterMessage::Failed => self.downloads.iter().filter(failed_filter),
        };

        let responsive = check_responsive_threshold(
            self.window_dimensions.0,
            self.settings.scaling,
            self.settings.sidebar_collapsed,
            self.settings.metadata_always_enabled || self.metadata.enabled,
        );

        let mut count = 0;

        let downloads = filtered_downloads.fold(
            col!().spacing(0).padding(match self.settings.list_layout {
                crate::components::settings::ListLayout::ListExtended => Padding::new(0.0),
                crate::components::settings::ListLayout::List => Padding::new(2.0).left(1).right(1),
            }),
            |column, (index, download)| {
                count += 1;
                column.push(
                    download
                        .view(&self.settings, responsive)
                        .map(|message| Message::Download(message, *index)),
                )
            },
        );

        let filtered_content = GuiElements::scrollbar(downloads, self.settings.scrollbars_visible);

        if self.downloads.is_empty()
            && !matches!(self.filter_type, DownloadsListFilterMessage::Deleted)
        {
            self.download_form
                .view(&self.settings, self.downloads.len(), None)
                .map(|message| Message::DownloadForm(message, None))
        } else {
            let download_state_filters_bar = self.download_state_filter_bar.view(
                &self.sidebar.active,
                &self.downloads,
                responsive,
            );

            let downloads_list_col = match self.settings.list_layout {
                crate::components::settings::ListLayout::ListExtended => col!()
                    .spacing(10)
                    .align_x(Alignment::Center)
                    .push(download_state_filters_bar)
                    .push(
                        container(filtered_content.height(if self.settings.stretch_list_view {
                            Fill
                        } else {
                            Shrink
                        }))
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
                                .push(container(vertical_space().height(1)).height(1).width(Fill))
                                .push(col![
                                    listview_header::view(&self.settings, responsive),
                                    // container(vertical_space().height(1))
                                    //     .height(1)
                                    //     .width(Fill)
                                    //     .class(AtomStyleContainer::LogoContainer),
                                ])
                                .push(filtered_content.height(
                                    if self.settings.stretch_list_view {
                                        Fill
                                    } else {
                                        Shrink
                                    },
                                )),
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

    pub fn view(&self, window_id: Id) -> Element<Message, AtomTheme> {
        let status_bar_text_icon_size = 10;

        if let Some(window) = self.windows.get(&window_id) {
            if window.0 != "main" {
                return window
                    .1
                    .view(&self.settings, self.downloads.len(), Some(window_id))
                    .map(move |message| Message::DownloadForm(message, Some(window_id)));
            }
        }

        if !self.instance.as_ref().unwrap().is_single() {
            return self.get_another_instance_view().into();
        }

        let view = match self.view {
            View::Import => self.import.view().map(Message::Import),
            View::Downloads => self.filter_downloads_view(),
            View::NewDownloadForm => self
                .download_form
                .view(&self.settings, self.downloads.len(), None)
                .map(|message| Message::DownloadForm(message, None)),
            View::Settings => self
                .phantom_settings
                .view(&self.settings, &self.theme)
                .map(Message::Settings),
            View::Shortcuts => keybindings::view(&self.settings),
        };

        let mut items_row = row!()
            .spacing(0)
            .width(Fill)
            .push(
                col!().width(Shrink).align_x(Alignment::Center).push(
                    container(
                        self.sidebar
                            .view(
                                &self.settings,
                                self.downloads.is_empty(),
                                self.titlebar.search_text.is_empty(),
                            )
                            .map(Message::Sidebar),
                    )
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
                .padding(Padding::new(20.0).right(0).left(0))
                .width(Fill)
                .height(Shrink),
            )
            .push(container(vertical_space()).width(15).padding(0));

        if self.metadata.enabled || self.settings.metadata_always_enabled {
            items_row = items_row.push(
                container(self.metadata.view(&self.settings).map(Message::Metadata))
                    .padding(Padding::new(20.0).right(15).left(0))
                    .height(Fill)
                    .width(Shrink),
            );
        }

        let main_row = col![
            self.titlebar
                .view(
                    &self.settings,
                    matches!(self.sidebar.active, SideBarActiveButton::Overview)
                )
                .map(Message::TitleBar),
            items_row,
            container(
                row![
                    icons::layout_statusbar().size(status_bar_text_icon_size),
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

    fn get_another_instance_view(&self) -> Container<Message, AtomTheme> {
        let main_row = col!()
            .width(Fill)
            .height(Fill)
            .padding(0)
            .align_x(Alignment::Center)
            .push(
                self.titlebar
                    .view(&self.settings, false)
                    .map(Message::TitleBar),
            )
            .push(
                container(
                    row!()
                        .align_y(Alignment::Center)
                        .spacing(10)
                        .push(icons::info_circle())
                        .push(text("Another instance of application is already running!")),
                )
                .padding(20)
                .center(Fill)
                .width(Fill)
                .height(Fill),
            );

        container(main_row).width(Fill)
    }
}
