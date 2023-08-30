use super::Theme;
use crate::color;
use iced::{widget::progress_bar, Background, BorderRadius, Color};

struct ColorPalette {
    background: Color,
    bar: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleProgressBar;

impl AtomStyleProgressBar {
    fn color_palette(&self, theme: &Theme) -> ColorPalette {
        match theme {
            Theme::Default => ColorPalette {
                background: color!(100, 100, 100),
                bar: color!(215, 252, 112),
            },
            Theme::Tangerine => ColorPalette {
                background: color!(100, 100, 100),
                bar: color!(254, 161, 47, 1),
            },
            Theme::Light => ColorPalette {
                background: color!(200, 200, 200),
                bar: color!(23, 29, 39, 1.0),
            },
        }
    }
}

impl progress_bar::StyleSheet for Theme {
    type Style = AtomStyleProgressBar;

    fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
        let color_palette = style.color_palette(&self);

        progress_bar::Appearance {
            background: Background::Color(color_palette.background),
            bar: Background::Color(color_palette.bar),
            border_radius: BorderRadius::from(5.0),
        }
    }
}
