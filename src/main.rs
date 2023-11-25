// build with icon
//  cargo rustc --release -- -C link-args="resources.res"
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::App;
use iced::{window::PlatformSpecific, Application, Font, Settings};
use tracing_subscriber::{prelude::*, registry, EnvFilter};
mod app;
mod components;
mod elements;
mod font;
mod messages;
mod style;
mod utils;

#[tracing::instrument]
fn main() -> iced::Result {
    registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(EnvFilter::from_default_env())
        .init();

    // run app
    App::run(Settings {
        antialiasing: true,
        // default_font: Font::with_name("Azeret Mono"),
        default_font: Font {
            family: iced::font::Family::Name("Google Sans Mono"),
            weight: iced::font::Weight::Normal,
            ..Default::default()
        },
        default_text_size: 16.0,
        exit_on_close_request: false,
        flags: (),
        id: Some("A.T.O.M".to_owned()),
        window: iced::window::Settings {
            size: (1200, 750),
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
