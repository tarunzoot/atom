use super::AtomTheme;
use crate::color;
use iced::{widget::toggler, Background, Color};

struct ColorPalette {
    background: Color,
    foreground: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleToggler {
    #[default]
    Default,
}

impl AtomStyleToggler {
    fn color_palette(&self, theme: &AtomTheme) -> ColorPalette {
        let accent = theme.accent();

        match theme {
            AtomTheme::Default => ColorPalette {
                background: accent,
                foreground: color!(0, 0, 0, 1),
            },
            AtomTheme::Tangerine => ColorPalette {
                background: accent,
                foreground: color!(0, 0, 0, 1),
            },
            AtomTheme::Light => ColorPalette {
                background: accent,
                foreground: color!(255, 255, 255, 0.8),
            },
            AtomTheme::Hari => ColorPalette {
                background: accent,
                foreground: color!(0x30394c),
            },
        }
    }
}

impl toggler::Catalog for AtomTheme {
    type Class<'a> = AtomStyleToggler;

    fn default<'a>() -> Self::Class<'a> {
        AtomStyleToggler::Default
    }

    fn style(&self, class: &Self::Class<'_>, status: toggler::Status) -> toggler::Style {
        let color_palette = class.color_palette(self);

        match status {
            toggler::Status::Active { is_toggled } => {
                if is_toggled {
                    toggler::Style {
                        background: Background::Color(color_palette.background),
                        background_border_color: color_palette.background,
                        foreground: Background::Color(color_palette.foreground),
                        foreground_border_color: color_palette.foreground,
                        background_border_width: 2.0,
                        foreground_border_width: 2.0,
                        border_radius: None,
                        padding_ratio: 0.2,
                        text_color: None,
                    }
                } else {
                    toggler::Style {
                        background: Background::Color(Color::TRANSPARENT),
                        background_border_color: color_palette.background,
                        foreground: Background::Color(color_palette.background),
                        foreground_border_color: color_palette.background,
                        background_border_width: 2.0,
                        foreground_border_width: 2.0,
                        border_radius: None,
                        padding_ratio: 0.2,
                        text_color: None,
                    }
                }
            }
            toggler::Status::Hovered { is_toggled } => {
                if is_toggled {
                    toggler::Style {
                        background: Background::Color(color_palette.background),
                        background_border_color: color_palette.background,
                        foreground: Background::Color(color_palette.foreground),
                        foreground_border_color: color_palette.foreground.scale_alpha(0.4),
                        ..self.style(class, toggler::Status::Active { is_toggled: true })
                    }
                } else {
                    toggler::Style {
                        background: Background::Color(Color::TRANSPARENT),
                        background_border_color: color_palette.background,
                        foreground: Background::Color(color_palette.background),
                        foreground_border_color: color_palette.background.scale_alpha(0.4),
                        ..self.style(class, toggler::Status::Active { is_toggled: false })
                    }
                }
            }
            toggler::Status::Disabled { is_toggled: _ } => toggler::Style {
                background: Background::Color(color_palette.background.scale_alpha(0.6)),
                background_border_color: color_palette.background.scale_alpha(0.6),
                foreground: Background::Color(color_palette.foreground.scale_alpha(0.6)),
                foreground_border_color: Color::TRANSPARENT,
                background_border_width: 2.0,
                foreground_border_width: 2.0,
                border_radius: None,
                padding_ratio: 0.0,
                text_color: None,
            },
        }
    }
}
