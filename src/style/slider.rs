use super::Theme;
use crate::color;

use iced::{
    widget::slider::{self, Handle, Rail},
    BorderRadius, Color,
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
    fn color_palette(&self, theme: &Theme) -> ColorPalette {
        match theme {
            Theme::Default => ColorPalette {
                rails: (color!(215, 252, 112), color!(215, 252, 112)),
                handle: color!(30, 30, 30),
                border: color!(215, 252, 112),
            },
            Theme::Tangerine => ColorPalette {
                rails: (color!(254, 161, 47, 1), color!(254, 161, 47, 1)),
                handle: color!(30, 30, 30),
                border: color!(254, 161, 47, 1),
            },
            Theme::Light => ColorPalette {
                rails: (color!(23, 29, 39, 1.0), color!(23, 29, 39, 1.0)),
                handle: color!(30, 30, 30),
                border: color!(23, 29, 39, 1.0),
            },
        }
    }
}

impl slider::StyleSheet for Theme {
    type Style = AtomStyleSlider;

    fn active(&self, style: &Self::Style) -> slider::Appearance {
        let color_palette = style.color_palette(&self);

        slider::Appearance {
            rail: Rail {
                colors: color_palette.rails,
                width: 5.0,
                border_radius: BorderRadius::default(),
            },
            handle: Handle {
                shape: slider::HandleShape::Circle { radius: 10.0 },
                color: color_palette.handle,
                border_width: 2.0,
                border_color: color_palette.border,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: Rail {
                colors: (
                    Color {
                        a: 0.8,
                        ..self.active(style).rail.colors.0
                    },
                    Color {
                        a: 0.8,
                        ..self.active(style).rail.colors.0
                    },
                ),
                width: 5.0,
                border_radius: BorderRadius::default(),
            },
            handle: Handle {
                shape: slider::HandleShape::Circle { radius: 10.0 },
                color: self.active(style).handle.color,
                border_width: 2.0,
                border_color: Color {
                    a: 0.8,
                    ..self.active(style).handle.border_color
                },
            },
        }
    }

    fn dragging(&self, style: &Self::Style) -> slider::Appearance {
        self.hovered(style)
    }
}
