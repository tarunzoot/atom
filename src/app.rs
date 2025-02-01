use crate::{
    components::atom::Atom,
    font::{ICOFONT_BYTES, JOSEFIN_BYTES, LEXEND_BYTES, MONOSPACED_FONT_BYTES, SYMBOLS_BYTES},
    messages::Message,
    style::AtomTheme,
    utils::{
        helpers::{ATOM_ICON, ATOM_SOCKET_ADDRESS},
        json_from_browser::JSONFromBrowser,
    },
};
use iced::{
    event,
    futures::{self, Stream},
    keyboard,
    widget::{container, text},
    window::{self, settings::PlatformSpecific, Id},
    Event,
    Length::Fill,
    Size, Subscription, Task as Command,
};
use std::{
    io::{prelude::*, BufReader, Write},
    net::TcpListener,
};
use tracing::warn;
use tray_icon::menu::MenuEvent;

pub enum App<'a> {
    Loading,
    Loaded(Atom<'a>),
}

impl App<'_> {
    pub fn new() -> (Self, Command<Message>) {
        #[cfg(target_os = "windows")]
        let platform_specific_settings = PlatformSpecific {
            undecorated_shadow: true,
            ..Default::default()
        };
        #[cfg(not(target_os = "windows"))]
        let platform_specific_settings = PlatformSpecific::default();

        let (_id, open) = window::open(window::Settings {
            size: Size {
                width: 1086.0,
                height: 610.0,
            },
            position: window::Position::Centered,
            min_size: Some(Size {
                width: 1086.0,
                height: 610.0,
            }),
            visible: true,
            resizable: true,
            decorations: false,
            transparent: false,
            level: window::Level::Normal,
            exit_on_close_request: false,
            platform_specific: platform_specific_settings,
            icon: Some(iced::window::icon::from_file_data(ATOM_ICON, None).unwrap()),
            ..Default::default()
        });

        (
            App::Loading,
            Command::batch(vec![
                iced::font::load(MONOSPACED_FONT_BYTES).map(Message::FontLoaded),
                iced::font::load(LEXEND_BYTES).map(Message::FontLoaded),
                iced::font::load(JOSEFIN_BYTES).map(Message::FontLoaded),
                iced::font::load(ICOFONT_BYTES).map(Message::FontLoaded),
                iced::font::load(SYMBOLS_BYTES).map(Message::FontLoaded),
                open.map(Message::MainWindow),
                Command::done(Message::LoadingComplete),
            ]),
        )
    }

    pub fn title(&self, _: Id) -> String {
        "A.T.O.M".to_string()
    }

    pub fn theme(&self, _: Id) -> AtomTheme {
        match self {
            App::Loading => AtomTheme::Default,
            App::Loaded(atom) => atom.theme,
        }
    }

    pub fn scale_factor(&self, _: Id) -> f64 {
        match self {
            App::Loading => 1.0,
            App::Loaded(atom) => atom.settings.scaling,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
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

                // subscriptions.push(event::listen().map(Message::EventsOccurred));
                subscriptions.push(
                    event::listen_with(|event, _, id| match event {
                        Event::Keyboard(keyboard::Event::KeyReleased { .. })
                        | Event::Keyboard(keyboard::Event::KeyPressed { .. })
                        | Event::Window(window::Event::Resized(_))
                        | Event::Mouse(iced::mouse::Event::ButtonPressed(
                            iced::mouse::Button::Left,
                        ))
                        | Event::Window(window::Event::CloseRequested) => Some((event, id)),
                        _ => None,
                    })
                    .map(Message::EventsOccurred),
                );
                subscriptions.push(atom.metadata.subscription().map(Message::Metadata));
                subscriptions.push(window::close_events().map(Message::WindowClosed));
                subscriptions.push(window::resize_events().map(Message::WindowResized));

                if !atom.should_exit {
                    subscriptions.push(Subscription::run(App::subscribe_web_requests));
                }

                if atom.tray.is_some() && !atom.should_exit {
                    subscriptions.push(Subscription::run(App::subscribe_tray_events));
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

    pub fn view(&self, window_id: Id) -> iced::Element<Message, AtomTheme> {
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
            App::Loaded(atom) => atom.view(window_id),
        }
    }
}

impl App<'_> {
    fn subscribe_tray_events() -> impl Stream<Item = Message> {
        let (mut sender, receiver) = futures::channel::mpsc::channel(100);
        std::thread::spawn(move || loop {
            if let Ok(event) = MenuEvent::receiver().recv() {
                sender.try_send(Message::TrayEvent(event.id)).ok();
            }
        });

        receiver
    }

    fn subscribe_web_requests() -> impl Stream<Item = Message> {
        let (mut sender, receiver) = futures::channel::mpsc::channel(100);

        std::thread::spawn(move || {
            let listener = TcpListener::bind(ATOM_SOCKET_ADDRESS);

            if listener.is_err() {
                sender
                    .try_send(Message::StatusBar(
                        "Browser download capture : OFF (Listener Failed)".to_string(),
                    ))
                    .ok();
                return;
            } else {
                sender
                    .try_send(Message::StatusBar(
                        "Browser download capture : ON".to_string(),
                    ))
                    .ok();
            }

            let listener = listener.unwrap();

            while let Ok((stream, _)) = listener.accept() {
                let mut stream = stream;
                let buf_reader = BufReader::new(&mut stream);
                let http_request: Vec<_> = buf_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.ends_with("<END>"))
                    .collect();

                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write_all(response.as_bytes()).ok();

                if http_request.is_empty() {
                    warn!("http_request from the browser is empty, maybe the app is exiting//");
                    continue;
                }

                let json = http_request.last().unwrap();
                let json = serde_json::from_str::<JSONFromBrowser>(json);
                if json.is_err() {
                    warn!("TCP JSON error : {:#?}", json);
                    continue;
                }

                let json = json.unwrap();
                if json.file_name.is_empty() || json.url.is_empty() {
                    continue;
                }

                sender
                    .try_send(Message::NewDownloadReceivedFromBrowser(json))
                    .ok();
            }
        });

        receiver
    }
}
