use super::Theme;
use crate::color;
use iced::{widget::toggler, Color};

struct ColorPalette {
    background: Color,
    foreground: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleToggler;

impl AtomStyleToggler {
    fn color_palette(&self, theme: &Theme) -> ColorPalette {
        let accent = theme.accent();

        match theme {
            Theme::Default => ColorPalette {
                background: accent,
                foreground: color!(0, 0, 0, 1),
            },
            Theme::Tangerine => ColorPalette {
                background: accent,
                foreground: color!(0, 0, 0, 1),
            },
            Theme::Light => ColorPalette {
                background: accent,
                foreground: color!(255, 255, 255, 0.8),
            },
            Theme::RedLight => ColorPalette {
                background: accent,
                foreground: color!(255, 255, 255, 0.8),
            },
        }
    }
}

impl toggler::StyleSheet for Theme {
    type Style = AtomStyleToggler;

    fn active(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
        let color_palette = style.color_palette(self);

        if is_active {
            toggler::Appearance {
                background: color_palette.background,
                background_border: Some(color_palette.background),
                foreground: color_palette.foreground,
                foreground_border: Some(Color::TRANSPARENT),
            }
        } else {
            toggler::Appearance {
                background: Color::TRANSPARENT,
                background_border: Some(color_palette.background),
                foreground: color_palette.background,
                foreground_border: Some(Color::TRANSPARENT),
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
        let color_palette = style.color_palette(self);

        if is_active {
            toggler::Appearance {
                background: color_palette.background,
                background_border: Some(color_palette.background),
                foreground: color_palette.foreground,
                foreground_border: Some(color_palette.foreground),
            }
        } else {
            toggler::Appearance {
                background: Color::TRANSPARENT,
                background_border: Some(color_palette.background),
                foreground: color_palette.background,
                foreground_border: Some(color_palette.background),
            }
        }
    }
}
