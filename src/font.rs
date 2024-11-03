#![allow(dead_code)]
#![allow(unused_variables)]

use crate::style::AtomTheme;
use iced::widget::{text, Text};

pub const MONOSPACED_FONT_BYTES: &[u8] =
    // include_bytes!("../resources/fonts/Google-Sans-Mono-Regular.ttf");
    include_bytes!("../resources/fonts/DMMono-Regular.ttf");
pub const LEXEND_BYTES: &[u8] = include_bytes!("../resources/fonts/LexendDeca-Black.ttf");
pub const ICOFONT_BYTES: &[u8] = include_bytes!("../resources/fonts/icofont.ttf");
pub const SYMBOLS_BYTES: &[u8] =
    include_bytes!("../resources/fonts/SymbolsNerdFontMono-Regular.ttf");
pub const ICOFONT: iced::Font = iced::Font::with_name("IcoFont");
pub const SYMBOLS: iced::Font = iced::Font::with_name("Symbols Nerd Font Mono");

pub fn file_type_icon<'a>(file_type: &str) -> Text<'a, AtomTheme> {
    let file_icon = match &file_type.to_lowercase()[..] {
        "jpg" | "jpeg" | "png" | "tiff" | "gif" | "webp" | "bmp" => '\u{eb1a}',
        "js" | "json" | "html" | "css" | "jsx" | "gulp" | "php" | "sass" | "scss" | "py"
        | "pyd" | "pyc" | "rs" | "java" | "vue" | "sh" | "bat" | "cmd" | "go" | "vim" | "c"
        | "cpp" | "h" | "hpp" => '\u{eb0c}',
        "ttf" | "woff" | "woff2" | "otf" | "eot" => '\u{edef}',
        "mp4" | "mkv" | "webm" | "ts" | "mov" | "avi" | "wmv" | "flv" | "f4v" | "swf" | "mpeg" => {
            '\u{eb2c}'
        }
        "ini" | "conf" | "toml" | "lock" => '\u{f085}',
        "xml" | "xhtml" | "xshtml" | "plist" => '\u{eb10}',
        "url" | "link" | "desktop" => '\u{ec84}',
        "pdf" => '\u{eb1e}',
        "zip" | "gz" | "7z" | "rar" | "xz" | "tar.xz" => '\u{eea5}',
        "deb" | "exe" | "msi" | "rpm" | "bin" | "appImage" | "dmg" => '\u{eb11}',
        "ai" => '\u{e7b4}',
        "psd" => '\u{eb24}',
        "sql" | "csv" | "db" | "tsv" => '\u{eb28}',
        "txt" => '\u{eb2a}',
        "mp3" | "wma" | "flac" | "midi" | "opus" | "m3u" | "ogg" | "oga" => '\u{eb1d}',
        "md" | "doc" | "docx" | "ppt" | "pptx" => '\u{eb0e}',
        _ => '\u{eb12}',
    };
    icon(file_icon, CustomFont::IcoFont).center()
}

pub fn get_file_type(file_type: &str) -> &'static str {
    match &file_type.to_lowercase()[..] {
        "jpg" | "jpeg" | "png" | "tiff" | "gif" | "webp" | "bmp" => "Image",
        "js" | "json" | "html" | "css" | "jsx" | "gulp" | "php" | "sass" | "scss" | "py"
        | "pyd" | "pyc" | "rs" | "java" | "vue" | "sh" | "bat" | "cmd" | "go" | "vim" | "c"
        | "cpp" | "h" | "hpp" => "Programming",
        "ttf" | "woff" | "woff2" | "otf" | "eot" => "Font",
        "mp4" | "mkv" | "webm" | "ts" | "mov" | "avi" | "wmv" | "flv" | "f4v" | "swf" | "mpeg" => {
            "Video"
        }
        "ini" | "conf" | "toml" | "lock" | "xml" | "xhtml" | "xshtml" | "plist" => "Configuration",
        "url" | "link" | "desktop" => "Shortcut",
        "pdf" | "docx" | "doc" | "odt" | "md" | "ppt" | "pptx" => "Document",
        "zip" | "gz" | "7z" | "rar" | "xz" | "tar.xz" | "iso" => "Archive",
        "vmdk" | "vdi" => "Virtual Disk",
        "deb" | "exe" | "msi" | "rpm" | "bin" | "appImage" | "dmg" => "Executable",
        "ai" | "psd" => "Graphic",
        "sql" | "csv" | "db" | "tsv" => "Database",
        "txt" => "Text",
        "mp3" | "wma" | "flac" | "midi" | "opus" | "m3u" | "ogg" | "oga" => "Audio",
        _ => "Generic",
    }
}

pub enum CustomFont {
    Symbols,
    IcoFont,
}

pub fn icon<'a>(unicode: char, custom_font: CustomFont) -> Text<'a, AtomTheme> {
    match custom_font {
        CustomFont::IcoFont => text(unicode).font(ICOFONT).center().size(16),

        CustomFont::Symbols => text(unicode).font(SYMBOLS).center().size(30),
    }
}
