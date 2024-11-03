use super::AtomTheme;
use crate::color;

use iced::{
    widget::slider::{self, Handle, Rail},
    Background, Border, Color,
};

struct ColorPalette {
    rails: (Color, Color),
    handle: Color,
    border: Color,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum AtomStyleSlider {
    #[default]
    Default,
}

impl AtomStyleSlider {
    fn color_palette(&self, theme: &AtomTheme) -> ColorPalette {
        let accent = theme.accent();

        match theme {
            AtomTheme::Default => ColorPalette {
                rails: (accent, accent),
                handle: color!(30, 30, 30),
                border: accent,
            },
            AtomTheme::Tangerine => ColorPalette {
                rails: (accent, accent),
                handle: color!(30, 30, 30),
                border: accent,
            },
            AtomTheme::Light => ColorPalette {
                rails: (accent, accent),
                handle: color!(30, 30, 30),
                border: accent,
            },
            AtomTheme::Hari => ColorPalette {
                rails: (accent, accent),
                handle: color!(0x2a3345),
                border: accent,
            },
        }
    }
}

impl slider::Catalog for AtomTheme {
    type Class<'a> = AtomStyleSlider;

    fn default<'a>() -> Self::Class<'a> {
        AtomStyleSlider::Default
    }

    fn style(&self, class: &Self::Class<'_>, status: slider::Status) -> slider::Style {
        match status {
            slider::Status::Active => {
                let color_palette = class.color_palette(self);

                slider::Style {
                    rail: Rail {
                        backgrounds: (
                            Background::Color(color_palette.rails.0),
                            Background::Color(color_palette.rails.1),
                        ),
                        width: 5.0,
                        border: Border::default(),
                    },
                    handle: Handle {
                        shape: slider::HandleShape::Circle { radius: 10.0 },
                        background: Background::Color(color_palette.handle),
                        border_width: 2.0,
                        border_color: color_palette.border,
                    },
                }
            }
            slider::Status::Hovered => slider::Style {
                rail: Rail {
                    backgrounds: (
                        self.style(class, slider::Status::Active)
                            .rail
                            .backgrounds
                            .0
                            .scale_alpha(0.8),
                        self.style(class, slider::Status::Active)
                            .rail
                            .backgrounds
                            .0
                            .scale_alpha(0.8),
                    ),
                    width: 5.0,
                    border: Border::default(),
                },
                handle: Handle {
                    shape: slider::HandleShape::Circle { radius: 10.0 },
                    background: self.style(class, slider::Status::Active).handle.background,
                    border_width: 2.0,
                    border_color: Color {
                        a: 0.8,
                        ..self
                            .style(class, slider::Status::Active)
                            .handle
                            .border_color
                    },
                },
            },
            slider::Status::Dragged => self.style(class, slider::Status::Hovered),
        }
    }
}
