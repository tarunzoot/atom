mod update;
mod view;
use crate::utils::helpers::{get_conf_directory, get_downloads_directory};
use serde::{Deserialize, Serialize};
use std::{fs::create_dir_all, path::PathBuf};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ListLayout {
    ListExtended,
    #[default]
    List,
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
    pub quit_action_closes_app: bool,
    pub auto_start_download: bool,
    pub theme: String,
    pub list_layout: ListLayout,
}

impl Default for AtomSettings {
    fn default() -> Self {
        let config_dir_path = get_conf_directory()
            .map_err(|e| {
                log::warn!("{e:#?}");
            })
            .unwrap();

        let cache_dir_path = config_dir_path.join("cache");
        if !cache_dir_path.exists() {
            create_dir_all(&cache_dir_path).ok();
        }

        Self {
            config_dir: config_dir_path,
            cache_dir: cache_dir_path,
            downloads_dir: get_downloads_directory(""),
            threads: 6,
            sidebar_collapsed: true,
            show_notifications: true,
            quit_action_closes_app: false,
            auto_start_download: false,
            theme: "Default".to_owned(),
            list_layout: ListLayout::ListExtended,
        }
    }
}
