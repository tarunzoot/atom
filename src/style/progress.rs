use super::AtomTheme;
use crate::color;
use iced::{widget::progress_bar, Background, Border, Color};

struct ColorPalette {
    background: Color,
    bar: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleProgressBar {
    #[default]
    Default,
}

impl AtomStyleProgressBar {
    fn color_palette(&self, theme: &AtomTheme) -> ColorPalette {
        let accent = theme.accent();

        match theme {
            AtomTheme::Default => ColorPalette {
                background: color!(100, 100, 100),
                bar: accent,
            },
            AtomTheme::Tangerine => ColorPalette {
                background: color!(100, 100, 100),
                bar: accent,
            },
            AtomTheme::Light => ColorPalette {
                background: color!(200, 200, 200),
                bar: accent,
            },
            AtomTheme::Hari => ColorPalette {
                background: color!(200, 200, 200),
                bar: accent,
            },
        }
    }
}

impl progress_bar::Catalog for AtomTheme {
    type Class<'a> = AtomStyleProgressBar;

    fn default<'a>() -> Self::Class<'a> {
        AtomStyleProgressBar::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> progress_bar::Style {
        let color_palette = class.color_palette(self);

        progress_bar::Style {
            background: Background::Color(color_palette.background),
            bar: Background::Color(color_palette.bar),
            border: Border {
                radius: 5.0.into(),
                ..Default::default()
            },
        }
    }
}
