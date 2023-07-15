use crate::{
    components::{atom::Atom, download::AtomDownload, settings::AtomSettings},
    messages::Message,
    style::Theme,
    utils::helpers::{handle_web_request, listen_for_tray_events},
};
use iced::{executor, Application, Command, Subscription};
use single_instance::SingleInstance;
use std::collections::BTreeMap;

impl<'a> Application for Atom<'a> {
    type Message = Message;
    type Flags = (AtomSettings, BTreeMap<usize, AtomDownload>, SingleInstance);
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::new(flags.0, flags.1, flags.2), Command::none())
    }

    fn title(&self) -> String {
        "A.T.O.M".to_string()
    }

    fn scale_factor(&self) -> f64 {
        if self.scale_factor < 1.0 {
            1.0
        } else {
            self.scale_factor
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let mut subscriptions: Vec<_> = self
            .downloads
            .iter()
            .map(|(&index, download)| download.subscription(index, &self.settings.cache_dir))
            .collect();

        subscriptions.push(iced_native::subscription::events().map(Message::EventsOccurred));
        // subscriptions.push(iced::window::frames().map(|_| Message::Tick));
        subscriptions.push(handle_web_request(self.should_exit));
        if self.tray.is_some() && !self.should_exit {
            subscriptions.push(listen_for_tray_events());
        }
        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        self.update(message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.view()
    }
}
