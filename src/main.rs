// build with icon
//  cargo rustc --release -- -C link-args="resources.res"
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use components::{atom::Atom, download::AtomDownload, settings::AtomSettings};
use iced::{window::PlatformSpecific, Application, Settings};
use std::{collections::BTreeMap, fs::create_dir_all, path::PathBuf};
use utils::helpers::{
    get_conf_directory, parse_downloads_toml, parse_settings_toml, save_settings_toml,
};
mod components;
mod font;
mod gui;
mod messages;
mod style;
mod utils;

fn main() -> iced::Result {
    env_logger::init();

    // check single instance of application
    let app_instance = single_instance::SingleInstance::new("fade9985-845c-4ca3-84b2-8a1b29a6c636")
        .map_err(|_| {
            log::error!("SingleInstance cannot be initialized!");
            std::process::exit(-1);
        })
        .unwrap();

    // check if config path can be created or exists
    let config_dir_path = get_conf_directory()
        .map_err(|e| {
            log::error!("{e:#?}");
            std::process::exit(1);
        })
        .unwrap();

    if !config_dir_path.exists() && create_dir_all(&config_dir_path).is_err() {
        log::error!("Error: cannot create config directory `{config_dir_path:#?}`, exiting.");
        std::process::exit(1);
    }

    let settings_path = config_dir_path.join("settings.toml");
    if !settings_path.exists() {
        log::warn!("No settings.toml found, using defaults");
        save_settings_toml(&AtomSettings {
            ..Default::default()
        });
    }

    let settings = parse_settings_toml(&settings_path);
    let downloads_toml_path = PathBuf::from(&settings.config_dir).join("downloads.toml");
    let downloads: BTreeMap<usize, AtomDownload> = parse_downloads_toml(&downloads_toml_path);

    log::info!("loading ATOM ui!");

    // run app
    Atom::run(Settings {
        antialiasing: true,
        default_font: Some(include_bytes!("../resources/fonts/Nunito-Regular.ttf")),
        default_text_size: 20.0,
        exit_on_close_request: false,
        flags: (settings, downloads, app_instance),
        id: Some("A.T.O.M".to_owned()),
        text_multithreading: true,
        try_opengles_first: false,
        window: iced::window::Settings {
            size: (1000, 650),
            position: iced::window::Position::Centered,
            min_size: Some((1000, 650)),
            max_size: None,
            resizable: true,
            decorations: false,
            transparent: false,
            always_on_top: false,
            icon: None,
            visible: true,
            platform_specific: PlatformSpecific::default(),
        },
    })
}
