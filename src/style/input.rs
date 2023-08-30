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
        match theme {
            Theme::Default => ColorPalette {
                accent: color!(215, 252, 112),
                background: Color::TRANSPARENT,
                hover_border: color!(215, 252, 112, 0.8),
                placeholder: color!(250, 250, 250, 0.4),
                text: Color::WHITE,
                disabled_border: color!(100, 100, 100, 0.3),
            },
            Theme::Tangerine => ColorPalette {
                accent: color!(254, 161, 47, 1.0),
                background: color!(49, 59, 69, 0.5),
                hover_border: color!(254, 161, 47, 0.8),
                placeholder: color!(250, 250, 250, 0.4),
                text: Color::WHITE,
                disabled_border: Color::TRANSPARENT,
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
                Theme::Default => appearance.accent,
                Theme::Tangerine => Color::TRANSPARENT,
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
