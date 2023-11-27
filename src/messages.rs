use tray_icon::menu::MenuId;

use crate::{
    components::download::{AtomDownload, DownloadType},
    utils::json_from_browser::JSONFromBrowser,
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct DownloadProperties {
    pub content_length: usize,
    pub download_type: DownloadType,
    pub error: String,
}

#[derive(Debug, Clone)]
pub enum DownloadMessage {
    SetFileSize(usize, usize),
    Downloading,
    DownloadProgress(usize),
    JoiningProgress(usize),
    Paused,
    DownloadDoneJoining,
    Finished,
    Error(String),
    DownloadSelected,
    MarkDeleted,
    RemoveDownload(bool), // force delete is true (for trash)
    HideDialog,
    Ignore,
}

#[derive(Debug, Clone)]
pub enum DownloadFormMessage {
    UrlChange(String),
    DownloadSequentially(bool),
    AddHeaderName(String),
    AddHeaderValue(String),
    EditHeader(String),
    DeleteHeader(String),
    AddHeader,
    FileSavePathChanged(Option<PathBuf>),
    HeaderFilePath(Option<PathBuf>),
    BrowseSaveAsFolder,
    ImportHeaders,
    AutoReferer(bool),
    AddNewDownload,
    ClosePane,
}

#[derive(Debug, Clone, Default)]
pub enum DownloadsListFilterMessage {
    Downloading,
    Paused,
    Finished,
    Deleted,
    #[default]
    All,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ClearCacheClicked,
    ThreadsChanged(u8),
    BrowseDownloadsDirClicked,
    NotificationToggle(bool),
    QuitActionToggle(bool),
    MaximizedActionToggle(bool),
    AutoStartDownloadToggle(bool),
    ThemeChanged(String),
    ListLayoutChanged(String),
    NewDownloadPositionChanged(String),
    ScalingChanged(f64),
    DownloadDirSelected(Option<PathBuf>),
    // BrowseCacheDirClicked,
    ClosePane,
    OpenConfigDir,
    SaveSettings,
}

#[derive(Debug, Clone)]
pub enum ImportMessage {
    ImportFileClicked,
    DownloadTypeToggled(bool),
    DownloadFolderSelectClicked,
    DownloadFolder(Option<PathBuf>),
    StartImportDownload,
    ClosePane,
    Ignore,
}

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    NewDownloadForm,
    ResumeAll,
    PauseAll,
    Settings,
    Shortcuts,
    DeleteConfirm,
    DeleteAll,
    Import,
    Expand,
    Collapse,
    GotoHomePage,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum SideBarActiveButton {
    #[default]
    Overview,
    AddDownload,
    PauseAll,
    DeleteAll,
    ResumeAll,
    Settings,
    Shortcuts,
    Import,
    Downloading,
    Paused,
    Finished,
    Trash,
    Null,
}

impl From<SideBarActiveButton> for String {
    fn from(value: SideBarActiveButton) -> Self {
        match value {
            SideBarActiveButton::Overview => "all".to_owned(),
            SideBarActiveButton::Downloading => "in progress".to_owned(),
            SideBarActiveButton::Paused => "paused".to_owned(),
            SideBarActiveButton::Finished => "finished".to_owned(),
            SideBarActiveButton::Trash => "trashed".to_owned(),
            _ => String::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum SideBarState {
    #[default]
    Collapsed,
    Full,
}

#[derive(Debug, Clone)]
pub enum TrayMessage {
    ShowApp,
    AddNewDownload,
    Settings,
    Import,
    Exit,
}

#[derive(Debug, Clone)]
pub enum MetadataMessage {
    PreviewFile,
    DeleteFile,
    ClosePane,
    CalculateChecksum,
    Checksum(String, String), // file path and url
    Ignore,
}

#[derive(Debug, Clone)]
pub enum TitleBarMessage {
    AppExit,
    AppHide,
    AppShow,
    AppMaximize,
    AppMinimize,
    SearchDownload(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    EventsOccurred(iced::Event),
    TitleBar(TitleBarMessage),
    Sidebar(SidebarMessage),
    DownloadForm(DownloadFormMessage),
    NewDownloadReceivedFromBrowser(JSONFromBrowser),
    AddNewDownload(AtomDownload),
    SaveDownloads,
    GotoHomePage,
    Download(DownloadMessage, usize),
    DownloadsListFilter(DownloadsListFilterMessage),
    Settings(SettingsMessage),
    ShowMetadata(usize),
    Metadata(MetadataMessage),
    Import(ImportMessage),
    TrayMessages(TrayMessage),
    TrayEvent(MenuId),
    FontLoaded(Result<(), iced::font::Error>),
    LoadingComplete,
    Ignore,
}
