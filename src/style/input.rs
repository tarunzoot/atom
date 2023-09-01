use super::Theme;
use crate::color;
use iced::{widget::text_input, Background, BorderRadius, Color};

struct ColorPalette {
    accent: Color,
    background: Color,
    hover_border: Color,
    disabled_border: Color,
    placeholder: Color,
    text: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleInput {
    #[default]
    Default,
    Search,
    Disabled,
}

impl AtomStyleInput {
    fn appearance(&self, theme: &Theme) -> ColorPalette {
        let accent = theme.accent();
        match theme {
            Theme::Default => ColorPalette {
                accent,
                background: Color::TRANSPARENT,
                hover_border: Color { a: 0.8, ..accent },
                placeholder: color!(250, 250, 250, 0.4),
                text: Color::WHITE,
                disabled_border: color!(100, 100, 100, 0.3),
            },
            Theme::Tangerine => ColorPalette {
                accent,
                background: color!(49, 59, 69, 0.5),
                hover_border: Color { a: 0.8, ..accent },
                placeholder: color!(250, 250, 250, 0.4),
                text: Color::WHITE,
                disabled_border: Color::TRANSPARENT,
            },
            Theme::Light => ColorPalette {
                accent,
                background: color!(255, 255, 255, 0.6),
                hover_border: Color { a: 0.8, ..accent },
                placeholder: color!(50, 50, 50, 0.6),
                text: Color::BLACK,
                disabled_border: color!(198, 202, 210, 1),
            },
            Theme::RedLight => ColorPalette {
                accent,
                background: color!(255, 255, 255, 0.6),
                hover_border: Color { a: 0.8, ..accent },
                placeholder: color!(50, 50, 50, 0.6),
                text: Color::BLACK,
                disabled_border: color!(198, 202, 210, 1),
            },
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = AtomStyleInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let appearance = style.appearance(&self);

        text_input::Appearance {
            background: Background::Color(appearance.background),
            border_radius: match style {
                AtomStyleInput::Search => BorderRadius::from(20.0),
                _ => BorderRadius::from(5.0),
            },
            border_width: 1.0,
            border_color: match self {
                Theme::Tangerine => Color::TRANSPARENT,
                Theme::Default => match style {
                    AtomStyleInput::Search => Color {
                        a: 0.1,
                        ..appearance.placeholder
                    },
                    _ => appearance.accent,
                },
                _ => match style {
                    AtomStyleInput::Search => Color {
                        a: 0.4,
                        ..appearance.placeholder
                    },
                    _ => appearance.accent,
                },
            },
            icon_color: appearance.accent,
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        let appearance = style.appearance(&self);
        appearance.placeholder
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let appearance = style.appearance(&self);

        text_input::Appearance {
            border_color: appearance.disabled_border,
            icon_color: appearance.accent,
            background: Background::Color(appearance.background),
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let appearance = style.appearance(self);

        text_input::Appearance {
            border_color: appearance.accent,
            ..self.active(style)
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        let appearance = style.appearance(&self);
        appearance.placeholder
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        let appearance = style.appearance(&self);
        appearance.text
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        let appearance = style.appearance(&self);
        appearance.accent
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let appearance = style.appearance(&self);

        text_input::Appearance {
            background: Background::Color(appearance.background),
            border_color: appearance.hover_border,
            icon_color: appearance.accent,
            ..self.active(style)
        }
    }
}
