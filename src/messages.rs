use crate::{
    components::{
        download::{AtomDownload, DownloadType},
        import::AtomImport,
        settings::AtomSettings,
    },
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
pub enum DownloadStateMessage {
    SetFileSize(usize, usize),
    Downloading,
    DownloadProgress(usize),
    JoiningProgress(usize),
    Paused,
    DownloadDoneJoining,
    Finished,
    Error(String),
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
    BrowseSaveAsFolder,
    AutoReferer(bool),
}

#[derive(Debug, Clone, Default)]
pub enum DownloadsFilterListMessage {
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
    AutoStartDownloadToggle(bool),
    // BrowseCacheDirClicked,
}

#[derive(Debug, Clone)]
pub enum ImportMessage {
    ImportFileClicked,
    DownloadTypeToggled(bool),
    DownloadFolderSelectClicked,
    DownloadFolder(Option<PathBuf>),
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
pub enum Message {
    EventsOccurred(iced_native::Event),
    AppExit,
    AppHide,
    AppShow,
    AppMaximize,
    AppMinimize,
    SearchDownload(String),
    Sidebar(SidebarMessage),
    DownloadForm(DownloadFormMessage),
    NewDownloadReceivedFromBrowser(JSONFromBrowser),
    AddNewDownload(AtomDownload),
    SaveDownloads,
    GotoHomePage,
    DownloadState(DownloadStateMessage, usize),
    MarkDownloadDeleted(usize),
    RemoveDownload(usize),
    FilterList(DownloadsFilterListMessage),
    Settings(SettingsMessage),
    DownloadItemSelected(usize),
    PreviewFile(String),
    DeleteFile(String),
    ClosePreview,
    OpenConfigDir,
    SaveSettings(AtomSettings),
    Import(ImportMessage),
    StartImportDownload(AtomImport),
    TrayMessages(TrayMessage),
    TrayEvent(u32),
    Ignore,
    // Tick,
}
