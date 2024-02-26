use super::Theme;
use crate::color;
use iced::{border::Radius, widget::progress_bar, Background, Color};

struct ColorPalette {
    background: Color,
    bar: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleProgressBar;

impl AtomStyleProgressBar {
    fn color_palette(&self, theme: &Theme) -> ColorPalette {
        let accent = theme.accent();

        match theme {
            Theme::Default => ColorPalette {
                background: color!(100, 100, 100),
                bar: accent,
            },
            Theme::Tangerine => ColorPalette {
                background: color!(100, 100, 100),
                bar: accent,
            },
            Theme::Light => ColorPalette {
                background: color!(200, 200, 200),
                bar: accent,
            },
            Theme::RedLight => ColorPalette {
                background: color!(200, 200, 200),
                bar: accent,
            },
        }
    }
}

impl progress_bar::StyleSheet for Theme {
    type Style = AtomStyleProgressBar;

    fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
        let color_palette = style.color_palette(self);

        progress_bar::Appearance {
            background: Background::Color(color_palette.background),
            bar: Background::Color(color_palette.bar),
            border_radius: Radius::from(5.0),
        }
    }
}
