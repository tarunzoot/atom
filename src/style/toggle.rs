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
            Theme::Hari => ColorPalette {
                background: accent,
                foreground: color!(0x30394c),
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
                background_border_color: color_palette.background,
                foreground: color_palette.foreground,
                foreground_border_color: Color::TRANSPARENT,
                background_border_width: 2.0,
                foreground_border_width: 2.0,
            }
        } else {
            toggler::Appearance {
                background: Color::TRANSPARENT,
                background_border_color: color_palette.background,
                foreground: color_palette.background,
                foreground_border_color: Color::TRANSPARENT,
                background_border_width: 2.0,
                foreground_border_width: 2.0,
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
        let color_palette = style.color_palette(self);

        if is_active {
            toggler::Appearance {
                background: color_palette.background,
                background_border_color: color_palette.background,
                foreground: color_palette.foreground,
                foreground_border_color: color_palette.foreground,
                ..self.active(style, true)
            }
        } else {
            toggler::Appearance {
                background: Color::TRANSPARENT,
                background_border_color: color_palette.background,
                foreground: color_palette.background,
                foreground_border_color: color_palette.background,
                ..self.active(style, false)
            }
        }
    }
}
