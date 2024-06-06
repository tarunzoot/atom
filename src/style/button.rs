use super::Theme;
use crate::color;
use iced::{border::Radius, widget::button, Background, Border, Color, Shadow, Vector};

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
                background: theme.accent(),
                border: color!(30, 30, 30, 1),
                text: Color::BLACK,
            },
            Theme::Tangerine => ColorPalette {
                background: theme.accent(),
                border: color!(50, 58, 65, 1),
                text: Color::BLACK,
            },
            Theme::Light => ColorPalette {
                background: theme.accent(),
                border: color!(150, 150, 150, 0.1),
                text: Color::WHITE,
            },
            Theme::Hari => ColorPalette {
                background: theme.accent(),
                border: Color {
                    a: 0.0,
                    ..theme.accent()
                },
                // text: color!(0xF7F7F2),
                text: color!(0x2a3345),
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
        let color_palette = style.appearance(self);

        button::Appearance {
            background: match style {
                AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::Neutral
                | AtomStyleButton::SidebarButtonActive
                | AtomStyleButton::SidebarButton
                | AtomStyleButton::HeaderButtons => None,
                _ => Some(Background::Color(color_palette.background)),
            },
            border: Border {
                radius: match style {
                    AtomStyleButton::RoundButton => Radius::from(50.0),
                    AtomStyleButton::HeaderButtons => Radius::from(0.0),
                    _ => Radius::from(5.0),
                },
                width: match style {
                    AtomStyleButton::HeaderButtons => 0.0,
                    _ => 1.0,
                },
                color: match style {
                    AtomStyleButton::Neutral
                    | AtomStyleButton::ShortcutKeyButton
                    | AtomStyleButton::SidebarButton
                    | AtomStyleButton::SidebarButtonActive => Color::TRANSPARENT,
                    AtomStyleButton::RoundButton | AtomStyleButton::HeaderButtons => {
                        style.color_offset(color_palette.border, 20.0)
                    }
                    AtomStyleButton::PrimaryButton => color_palette.border,
                },
            },
            text_color: match style {
                AtomStyleButton::SidebarButtonActive => color_palette.background,
                AtomStyleButton::SidebarButton
                | AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::HeaderButtons
                | AtomStyleButton::Neutral => match self {
                    Theme::Light => color_palette.background,
                    _ => Color::WHITE,
                },
                AtomStyleButton::RoundButton => match self {
                    Theme::Hari => color_palette.text,
                    _ => color_palette.text,
                },
                _ => color_palette.text,
            },
            shadow_offset: Default::default(),
            shadow: match style {
                AtomStyleButton::ShortcutKeyButton => Shadow {
                    color: match self {
                        Theme::Default | Theme::Tangerine | Theme::Hari => Color {
                            a: 0.2,
                            ..Color::BLACK
                        },
                        _ => color_palette.border,
                    },
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 2.0,
                },
                _ => Shadow::default(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let color_palette = style.appearance(self);

        button::Appearance {
            background: match style {
                AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::Neutral
                | AtomStyleButton::SidebarButtonActive
                | AtomStyleButton::SidebarButton
                | AtomStyleButton::HeaderButtons => match self {
                    Theme::Hari => Some(Background::Color(
                        style.color_offset(color!(0x30394c), 25.0),
                    )),
                    _ => Some(Background::Color(
                        style.color_offset(color_palette.border, 5.0),
                    )),
                },
                _ => Some(Background::Color(color_palette.background)),
            },
            border: Border {
                color: match style {
                    AtomStyleButton::Neutral
                    | AtomStyleButton::ShortcutKeyButton
                    | AtomStyleButton::SidebarButton
                    | AtomStyleButton::SidebarButtonActive => Color::TRANSPARENT,
                    AtomStyleButton::HeaderButtons => {
                        style.color_offset(color_palette.border, 40.0)
                    }
                    AtomStyleButton::RoundButton | AtomStyleButton::PrimaryButton => {
                        color_palette.background
                    }
                },
                ..self.active(style).border
            },
            text_color: match self {
                Theme::Hari => match style {
                    AtomStyleButton::HeaderButtons
                    | AtomStyleButton::SidebarButton
                    | AtomStyleButton::SidebarButtonActive
                    | AtomStyleButton::Neutral => {
                        color!(0xF7F7F2)
                    }
                    _ => color!(0x30394c),
                },
                _ => self.active(style).text_color,
            },
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let color_palette = style.appearance(self);

        button::Appearance {
            background: match style {
                AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::Neutral
                | AtomStyleButton::SidebarButtonActive
                | AtomStyleButton::SidebarButton
                | AtomStyleButton::HeaderButtons => match self {
                    Theme::Hari => Some(Background::Color(
                        style.color_offset(color_palette.text, 5.0),
                    )),
                    _ => Some(Background::Color(
                        style.color_offset(color_palette.border, 5.0),
                    )),
                },
                _ => match self {
                    Theme::Hari => Some(Background::Color(Color {
                        a: 0.2,
                        ..Color::BLACK
                    })),
                    _ => Some(Background::Color(color_palette.border)),
                },
            },
            border: Border {
                color: match style {
                    AtomStyleButton::PrimaryButton => color!(80, 80, 80, 0.4),
                    AtomStyleButton::ShortcutKeyButton => match self {
                        Theme::Hari => color!(80, 80, 80, 0.4),
                        _ => self.active(style).border.color,
                    },
                    _ => self.active(style).border.color,
                },
                ..self.active(style).border
            },
            text_color: match style {
                AtomStyleButton::SidebarButtonActive => color_palette.background,
                AtomStyleButton::SidebarButton
                | AtomStyleButton::ShortcutKeyButton
                | AtomStyleButton::HeaderButtons
                | AtomStyleButton::Neutral => color_palette.background,
                _ => match self {
                    Theme::Light => color_palette.background,
                    Theme::Hari => color!(0xF7F7F2),
                    _ => Color::WHITE,
                },
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
