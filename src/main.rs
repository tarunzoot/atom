// build with icon
//  cargo rustc --release -- -C link-args="resources.res"
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::App;
use font::MONOSPACED_FONT_BYTES;
use iced::Font;
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
    iced::daemon(App::title, App::update, App::view)
        .theme(App::theme)
        .scale_factor(App::scale_factor)
        .subscription(App::subscription)
        .antialiasing(true)
        .default_font(Font {
            // family: iced::font::Family::Name("Google Sans Mono"),
            family: iced::font::Family::Name("DM Mono"),
            weight: iced::font::Weight::Normal,
            ..Default::default()
        })
        .font(MONOSPACED_FONT_BYTES)
        .run_with(App::new)
}
