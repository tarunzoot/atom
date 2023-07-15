#![allow(dead_code)]
#![allow(unused_variables)]

use crate::style::Theme;
use iced::{
    alignment::Horizontal,
    alignment::Vertical,
    widget::{text, Text},
    Font, Renderer,
};

pub fn file_type_icon(file_type: &str) -> Text<'static, Renderer<Theme>> {
    let file_icon = match &file_type.to_lowercase()[..] {
        "jpg" | "jpeg" | "png" | "tiff" | "gif" | "webp" | "bmp" => '\u{ef4b}',
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
    icon(file_icon, CustomFont::IcoFont)
}

pub const ICOFONT: Font = Font::External {
    name: "IcoFont",
    bytes: include_bytes!("../resources/fonts/icofont.ttf"),
};

const SYMBOLS: Font = Font::External {
    name: "FiraCode",
    bytes: include_bytes!("../resources/fonts/SymbolsNerdFontMono-Regular.ttf"),
};

pub enum CustomFont {
    Symbols,
    IcoFont,
}

pub fn icon(unicode: char, custom_font: CustomFont) -> Text<'static, Renderer<Theme>> {
    match custom_font {
        CustomFont::IcoFont => text(&unicode.to_string())
            .font(ICOFONT)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(16),

        CustomFont::Symbols => text(&unicode.to_string())
            .font(SYMBOLS)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(30),
    }
}
