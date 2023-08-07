// build with icon
//  cargo rustc --release -- -C link-args="resources.res"
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::app::App;
use iced::{window::PlatformSpecific, Application, Font, Settings};
mod app;
mod components;
mod elements;
mod font;
mod messages;
mod style;
mod utils;

fn main() -> iced::Result {
    env_logger::init();

    // run app
    App::run(Settings {
        antialiasing: true,
        default_font: Font::with_name("Work Sans"),
        default_text_size: 16.0,
        exit_on_close_request: false,
        flags: (),
        id: Some("A.T.O.M".to_owned()),
        window: iced::window::Settings {
            #[cfg(not(target_os = "macos"))]
            size: (1100, 700),
            #[cfg(target_os = "macos")]
            size: (1150, 720),
            position: iced::window::Position::Centered,
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: false,
            transparent: false,
            icon: None,
            visible: true,
            platform_specific: PlatformSpecific::default(),
            level: iced::window::Level::Normal,
        },
    })
}
