use super::Theme;
use crate::color;
use iced::{widget::button, Background, BorderRadius, Color, Vector};

struct ColorPalette {
    background: Color,
    border: Color,
    text: Color,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum AtomStyleButton {
    SidebarButton,
    SidebarButtonActive,
    RoundButton,
    #[default]
    PrimaryButton,
    HeaderButtons,
    Neutral,
    ShortcutKeyButton,
}

impl AtomStyleButton {
    fn appearance(&self, theme: &Theme) -> ColorPalette {
        match theme {
            Theme::Default => ColorPalette {
                background: color!(215, 252, 112, 1),
                border: color!(30, 30, 30, 1),
                text: Color::BLACK,
            },
            Theme::Tangerine => ColorPalette {
                background: color!(254, 161, 47, 1),
                border: color!(50, 58, 65, 1),
                text: Color::BLACK,
            },
        }
    }

    fn color_offset(&self, color: Color, offset: f32) -> Color {
        let new_offset = offset / 255.0;
        Color {
            r: color.r + new_offset,
            g: color.g + new_offset,
            b: color.b + new_offset,
            a: color.a,
        }
    }
}

impl button::StyleSheet for Theme {
    type Style = AtomStyleButton;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let color_palette = style.appearance(&self);

        button::Appearance {
            background: match style {
                AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::Neutral
                | AtomStyleButton::SidebarButtonActive
                | AtomStyleButton::SidebarButton
                | AtomStyleButton::HeaderButtons => None,
                _ => Some(Background::Color(color_palette.background)),
            },
            border_radius: match style {
                AtomStyleButton::RoundButton => BorderRadius::from(50.0),
                AtomStyleButton::HeaderButtons => BorderRadius::from(0.0),
                _ => BorderRadius::from(5.0),
            },
            border_width: match style {
                AtomStyleButton::HeaderButtons => 0.0,
                _ => 1.0,
            },
            border_color: match style {
                AtomStyleButton::Neutral
                | AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::SidebarButton
                | AtomStyleButton::SidebarButtonActive => Color::TRANSPARENT,
                AtomStyleButton::RoundButton | AtomStyleButton::HeaderButtons => {
                    style.color_offset(color_palette.border, 20.0)
                }
                AtomStyleButton::PrimaryButton => color_palette.border,
            },
            text_color: match style {
                AtomStyleButton::SidebarButtonActive => color_palette.background,
                AtomStyleButton::SidebarButton
                | AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::HeaderButtons
                | AtomStyleButton::Neutral => Color::WHITE,
                _ => color_palette.text,
            },
            shadow_offset: Default::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let color_palette = style.appearance(&self);

        button::Appearance {
            background: match style {
                AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::Neutral
                | AtomStyleButton::SidebarButtonActive
                | AtomStyleButton::SidebarButton
                | AtomStyleButton::HeaderButtons => match self {
                    Theme::Default => Some(Background::Color(
                        style.color_offset(color_palette.border, 5.0),
                    )),
                    Theme::Tangerine => Some(Background::Color(
                        style.color_offset(color_palette.border, 5.0),
                    )),
                },
                _ => Some(Background::Color(color_palette.background)),
            },
            border_color: match style {
                AtomStyleButton::Neutral
                | AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::SidebarButton
                | AtomStyleButton::SidebarButtonActive => Color::TRANSPARENT,
                AtomStyleButton::HeaderButtons => style.color_offset(color_palette.border, 40.0),
                AtomStyleButton::RoundButton | AtomStyleButton::PrimaryButton => {
                    color_palette.background
                }
            },
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let color_palette = style.appearance(&self);

        button::Appearance {
            background: match style {
                AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::Neutral
                | AtomStyleButton::SidebarButtonActive
                | AtomStyleButton::SidebarButton
                | AtomStyleButton::HeaderButtons => Some(Background::Color(
                    style.color_offset(color_palette.border, 5.0),
                )),
                _ => Some(Background::Color(color_palette.border)),
            },
            border_color: match style {
                AtomStyleButton::PrimaryButton => color!(80, 80, 80, 0.4),
                _ => self.active(style).border_color,
            },
            text_color: match style {
                AtomStyleButton::SidebarButtonActive => color_palette.background,
                AtomStyleButton::SidebarButton
                | AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::HeaderButtons
                | AtomStyleButton::Neutral => Color::WHITE,
                _ => Color::WHITE,
            },
            shadow_offset: match style {
                AtomStyleButton::Neutral | AtomStyleButton::ShortcutKeyButton => {
                    Vector::new(0.0, 2.0)
                }
                _ => Default::default(),
            },
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}
