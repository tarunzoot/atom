use crate::{
    components::atom::Atom,
    font::{ICOFONT_BYTES, JOSEFIN_BYTES, LEXEND_BYTES, MONOSPACED_FONT_BYTES, SYMBOLS_BYTES},
    messages::Message,
    style::AtomTheme,
    utils::helpers::{handle_web_request, listen_for_tray_events},
};
use iced::{
    event,
    widget::{container, text},
    window,
    Length::Fill,
    Subscription, Task as Command,
};

pub enum App<'a> {
    Loading,
    Loaded(Atom<'a>),
}

impl<'a> App<'a> {
    pub fn new() -> (Self, Command<Message>) {
        (
            App::Loading,
            Command::batch(vec![
                iced::font::load(MONOSPACED_FONT_BYTES).map(Message::FontLoaded),
                iced::font::load(LEXEND_BYTES).map(Message::FontLoaded),
                iced::font::load(JOSEFIN_BYTES).map(Message::FontLoaded),
                iced::font::load(ICOFONT_BYTES).map(Message::FontLoaded),
                iced::font::load(SYMBOLS_BYTES).map(Message::FontLoaded),
                Command::done(Message::LoadingComplete),
            ]),
        )
    }

    pub fn title(&self) -> String {
        "A.T.O.M".to_string()
    }

    pub fn theme(&self) -> AtomTheme {
        match self {
            App::Loading => AtomTheme::Default,
            App::Loaded(atom) => atom.theme,
        }
    }

    pub fn scale_factor(&self) -> f64 {
        match self {
            App::Loading => 1.0,
            App::Loaded(atom) => atom.settings.scaling,
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        match self {
            App::Loading => Subscription::none(),
            App::Loaded(atom) => {
                let mut subscriptions: Vec<_> = atom
                    .downloads
                    .iter()
                    .map(|(&index, download)| {
                        download.subscription(index, &atom.settings.cache_dir, atom.client.clone())
                    })
                    .collect();

                subscriptions.push(event::listen().map(Message::EventsOccurred));
                subscriptions.push(atom.metadata.subscription().map(Message::Metadata));

                if !atom.should_exit {
                    subscriptions.push(handle_web_request());
                }

                if atom.tray.is_some() && !atom.should_exit {
                    subscriptions.push(listen_for_tray_events());
                }

                // subscriptions.push(iced::window::frames().map(|_| Message::Tick));
                Subscription::batch(subscriptions)
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            App::Loading => {
                let mut command = Command::none();
                if let Message::LoadingComplete = message {
                    let atom = Atom::new();
                    if atom.settings.maximized {
                        command = window::get_latest().and_then(iced::window::toggle_maximize)
                    }
                    *self = App::Loaded(atom);
                }

                command
            }
            App::Loaded(atom) => atom.update(message),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message, AtomTheme> {
        match self {
            App::Loading => container(
                text("loading...")
                    .size(50)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
            .into(),
            App::Loaded(atom) => atom.view(),
        }
    }
}
