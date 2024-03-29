use crate::{
    components::atom::Atom,
    font::{ICOFONT_BYTES, LEXEND_BYTES, MONOSPACED_FONT_BYTES, SYMBOLS_BYTES},
    messages::Message,
    style::Theme,
    utils::helpers::{handle_web_request, listen_for_tray_events},
};
use iced::{
    event, executor,
    widget::{container, text},
    window::Id,
    Application, Command, Length, Subscription,
};

pub enum App<'a> {
    Loading,
    Loaded(Atom<'a>),
}

impl<'a> Application for App<'a> {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Message>) {
        (
            App::Loading,
            Command::batch(vec![
                iced::font::load(LEXEND_BYTES).map(Message::FontLoaded),
                iced::font::load(MONOSPACED_FONT_BYTES).map(Message::FontLoaded),
                iced::font::load(ICOFONT_BYTES).map(Message::FontLoaded),
                iced::font::load(SYMBOLS_BYTES).map(Message::FontLoaded),
                Command::perform(async {}, |_| Message::LoadingComplete),
            ]),
        )
    }

    fn title(&self) -> String {
        "A.T.O.M".to_string()
    }

    fn theme(&self) -> Self::Theme {
        match self {
            App::Loading => Theme::Default,
            App::Loaded(atom) => atom.theme,
        }
    }

    fn scale_factor(&self) -> f64 {
        match self {
            App::Loading => 1.0,
            App::Loaded(atom) => atom.settings.scaling,
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
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
                // subscriptions.push(iced::window::frames().map(|_| Message::Tick));
                subscriptions.push(handle_web_request(atom.should_exit));
                if atom.tray.is_some() && !atom.should_exit {
                    subscriptions.push(listen_for_tray_events());
                }
                Subscription::batch(subscriptions)
            }
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match self {
            App::Loading => {
                let mut command = Command::none();
                if let Message::LoadingComplete = message {
                    let atom = Atom::new();
                    if atom.settings.maximized {
                        command = iced::window::toggle_maximize(Id::MAIN);
                    }
                    *self = App::Loaded(atom);
                }

                command
            }
            App::Loaded(atom) => atom.update(message),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Theme> {
        match self {
            App::Loading => container(
                text("loading...")
                    .size(50)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into(),
            App::Loaded(atom) => atom.view(),
        }
    }
}
