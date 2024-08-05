// build with icon
//  cargo rustc --release -- -C link-args="resources.res"
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::App;
use font::{ICOFONT_BYTES, LEXEND_BYTES, MONOSPACED_FONT_BYTES, SYMBOLS_BYTES};
use iced::{window::settings::PlatformSpecific, Application, Font, Pixels, Settings, Size};
use tracing_subscriber::{prelude::*, registry, EnvFilter};
mod app;
mod components;
mod elements;
mod font;
mod messages;
mod style;
mod utils;
use std::{env, fs::File, sync::Arc};

#[tracing::instrument]
fn main() -> iced::Result {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    if let Ok(log_file_path) = env::var("ATOM_LOG_FILE") {
        let debug_log_file = match File::create(log_file_path) {
            Ok(file) => file,
            Err(error) => panic!("Error: {:?}", error),
        };
        let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(debug_log_file));
        registry()
            .with(stdout_log.and_then(debug_log))
            .with(EnvFilter::from_default_env())
            .init();
    } else {
        registry()
            .with(stdout_log)
            .with(EnvFilter::from_default_env())
            .init();
    }

    // run app
    App::run(Settings {
        antialiasing: true,
        // default_font: Font::with_name("Azeret Mono"),
        default_font: Font {
            // family: iced::font::Family::Name("Google Sans Mono"),
            family: iced::font::Family::Name("DM Mono"),
            weight: iced::font::Weight::Normal,
            ..Default::default()
        },
        fonts: vec![
            MONOSPACED_FONT_BYTES.into(),
            LEXEND_BYTES.into(),
            ICOFONT_BYTES.into(),
            SYMBOLS_BYTES.into(),
        ],
        default_text_size: Pixels::from(16.0),
        flags: (),
        id: Some("A.T.O.M".to_owned()),
        window: iced::window::Settings {
            size: Size {
                width: 1086.0,
                height: 610.0,
            },
            position: iced::window::Position::Centered,
            min_size: Some(Size {
                width: 1086.0,
                height: 610.0,
            }),
            max_size: None,
            resizable: true,
            decorations: false,
            transparent: false,
            icon: None,
            visible: true,
            platform_specific: PlatformSpecific::default(),
            level: iced::window::Level::Normal,
            exit_on_close_request: false,
        },
    })
}
