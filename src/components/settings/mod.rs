mod update;
mod view;
use crate::utils::helpers::{get_conf_directory, get_downloads_directory};
use serde::{Deserialize, Serialize};
use std::{fs::create_dir_all, path::PathBuf};
use tracing::warn;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ListLayout {
    ListExtended,
    #[default]
    List,
}

impl ListLayout {
    fn variants() -> Vec<String> {
        vec!["List".to_string(), "ListExtended".to_string()]
    }
}

impl From<String> for ListLayout {
    fn from(value: String) -> Self {
        match &value[..] {
            "ListExtended" | "listextended" => Self::ListExtended,
            "List" | "list" | "default" => Self::List,
            _ => Self::List,
        }
    }
}

impl From<ListLayout> for String {
    fn from(value: ListLayout) -> Self {
        match value {
            ListLayout::List => "List".to_owned(),
            ListLayout::ListExtended => "ListExtended".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomSettings {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub downloads_dir: String,
    pub threads: u8,
    pub sidebar_collapsed: bool,
    pub show_notifications: bool,
    pub minimize_to_tray: bool,
    pub auto_start_download: bool,
    pub theme: String,
    pub list_layout: ListLayout,
    pub stretch_list_view: bool,
    pub scrollbars_visible: bool,
    pub scaling: f64,
    pub maximized: bool,
    pub new_download_pos: String,
    pub font_size: f32,
    pub metadata_always_enabled: bool,
    #[serde(skip_deserializing, skip_serializing)]
    pub show_confirm_dialog: bool,
    #[serde(skip_deserializing, skip_serializing)]
    pub reset_settings: bool,
}

impl Default for AtomSettings {
    fn default() -> Self {
        let config_dir_path = get_conf_directory()
            .map_err(|e| {
                warn!("{e:#?}");
            })
            .unwrap();

        let cache_dir_path = config_dir_path.join("cache");
        if !cache_dir_path.exists() {
            create_dir_all(&cache_dir_path).ok();
        }

        let downloads_dir = get_downloads_directory("");

        Self {
            config_dir: config_dir_path.clone(),
            cache_dir: cache_dir_path.clone(),
            downloads_dir,
            threads: 6,
            sidebar_collapsed: true,
            show_notifications: true,
            minimize_to_tray: true,
            auto_start_download: false,
            theme: "Default".to_owned(),
            list_layout: ListLayout::ListExtended,
            scrollbars_visible: false,
            scaling: 1.0,
            maximized: true,
            new_download_pos: "First".to_string(),
            stretch_list_view: false,
            font_size: 16.0,
            show_confirm_dialog: false,
            reset_settings: false,
            metadata_always_enabled: false,
        }
    }
}
