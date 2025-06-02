use iced::{window::Id, Size};
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
    EditHeaderValue(String, String),
    DeleteHeader(String),
    AddHeader,
    FileSavePathChanged(Option<PathBuf>),
    HeaderFilePath(Option<PathBuf>),
    BrowseSaveAsFolder,
    ImportHeaders,
    AutoReferer(bool),
    AutoOpen(bool),
    AddNewDownload,
    Minimize,
    MouseOverHeading,
    MouseAwayFromHeading,
    ClosePane,
}

#[derive(Debug, Clone, Default)]
pub enum DownloadsListFilterMessage {
    Downloading,
    Paused,
    Finished,
    Deleted,
    Failed,
    #[default]
    All,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ClearCacheClicked(bool),
    ThreadsChanged(u8),
    BrowseDownloadsDirClicked,
    NotificationToggle(bool),
    QuitActionToggle(bool),
    MaximizedActionToggle(bool),
    AutoStartDownloadToggle(bool),
    ListBackgroundToggle(bool),
    AlwaysShowPreviewPaneToggle(bool),
    ThemeChanged(String),
    ListLayoutChanged(String),
    NewDownloadPositionChanged(String),
    ScalingChanged(f64),
    TextSizeChanged(f32),
    DownloadDirSelected(Option<PathBuf>),
    // BrowseCacheDirClicked,
    ClosePane,
    OpenConfigDir,
    SaveSettings(bool),
    ResetSettings(bool),
    HideDialog,
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
    HideDialog,
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
    MouseOnTitlebar(bool),
}

#[derive(Debug, Clone)]
pub enum Message {
    EventsOccurred((iced::Event, Id)),
    WindowResized((Id, Size)),
    StatusBar(String),
    TitleBar(TitleBarMessage),
    Sidebar(SidebarMessage),
    DownloadForm(DownloadFormMessage, Option<Id>),
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
    MainWindow(Id),
    WindowClosed(Id),
    WindowOpened(Id, Option<AtomDownload>),
    Ignore,
}
