use super::AtomTheme;
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
    fn appearance(&self, theme: &AtomTheme) -> ColorPalette {
        match theme {
            AtomTheme::Default => ColorPalette {
                background: theme.accent(),
                border: color!(30, 30, 30, 1),
                text: Color::BLACK,
            },
            AtomTheme::Tangerine => ColorPalette {
                background: theme.accent(),
                border: color!(50, 58, 65, 1),
                text: Color::BLACK,
            },
            AtomTheme::Light => ColorPalette {
                background: theme.accent(),
                border: color!(150, 150, 150, 0.1),
                text: Color::WHITE,
            },
            AtomTheme::Hari => ColorPalette {
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

impl button::Catalog for AtomTheme {
    type Class<'a> = AtomStyleButton;

    fn default<'a>() -> Self::Class<'a> {
        AtomStyleButton::PrimaryButton
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => {
                let color_palette = class.appearance(self);

                button::Style {
                    background: match class {
                        AtomStyleButton::ShortcutKeyButton
                        | AtomStyleButton::Neutral
                        | AtomStyleButton::SidebarButtonActive
                        | AtomStyleButton::SidebarButton
                        | AtomStyleButton::HeaderButtons => None,
                        _ => Some(Background::Color(color_palette.background)),
                    },
                    border: Border {
                        radius: match class {
                            AtomStyleButton::RoundButton => Radius::from(50.0),
                            AtomStyleButton::HeaderButtons => Radius::from(0.0),
                            _ => Radius::from(5.0),
                        },
                        width: match class {
                            AtomStyleButton::HeaderButtons => 0.0,
                            _ => 1.0,
                        },
                        color: match class {
                            AtomStyleButton::Neutral
                            | AtomStyleButton::ShortcutKeyButton
                            | AtomStyleButton::SidebarButton
                            | AtomStyleButton::SidebarButtonActive => Color::TRANSPARENT,
                            AtomStyleButton::RoundButton | AtomStyleButton::HeaderButtons => {
                                class.color_offset(color_palette.border, 20.0)
                            }
                            AtomStyleButton::PrimaryButton => color_palette.border,
                        },
                    },
                    text_color: match class {
                        AtomStyleButton::SidebarButtonActive => color_palette.background,
                        AtomStyleButton::SidebarButton
                        | AtomStyleButton::ShortcutKeyButton
                        | AtomStyleButton::HeaderButtons
                        | AtomStyleButton::Neutral => match self {
                            AtomTheme::Light => color_palette.background,
                            _ => Color::WHITE,
                        },
                        AtomStyleButton::RoundButton => match self {
                            AtomTheme::Hari => color_palette.text,
                            _ => color_palette.text,
                        },
                        _ => color_palette.text,
                    },
                    shadow: match class {
                        AtomStyleButton::ShortcutKeyButton => Shadow {
                            color: match self {
                                AtomTheme::Default | AtomTheme::Tangerine | AtomTheme::Hari => {
                                    Color {
                                        a: 0.2,
                                        ..Color::BLACK
                                    }
                                }
                                _ => color_palette.border,
                            },
                            offset: Vector::new(0.0, 4.0),
                            blur_radius: 2.0,
                        },
                        _ => Shadow::default(),
                    },
                }
            }
            button::Status::Hovered => {
                let color_palette = class.appearance(self);

                button::Style {
                    background: match class {
                        AtomStyleButton::ShortcutKeyButton
                        | AtomStyleButton::Neutral
                        | AtomStyleButton::SidebarButtonActive
                        | AtomStyleButton::SidebarButton
                        | AtomStyleButton::HeaderButtons => match self {
                            AtomTheme::Hari => Some(Background::Color(
                                class.color_offset(color!(0x30394c), 25.0),
                            )),
                            _ => Some(Background::Color(
                                class.color_offset(color_palette.border, 5.0),
                            )),
                        },
                        _ => Some(Background::Color(color_palette.background)),
                    },
                    border: Border {
                        color: match class {
                            AtomStyleButton::Neutral
                            | AtomStyleButton::ShortcutKeyButton
                            | AtomStyleButton::SidebarButton
                            | AtomStyleButton::SidebarButtonActive => Color::TRANSPARENT,
                            AtomStyleButton::HeaderButtons => {
                                class.color_offset(color_palette.border, 40.0)
                            }
                            AtomStyleButton::RoundButton | AtomStyleButton::PrimaryButton => {
                                color_palette.background
                            }
                        },
                        ..self.style(class, button::Status::Active).border
                    },
                    text_color: match self {
                        AtomTheme::Hari => match class {
                            AtomStyleButton::HeaderButtons
                            | AtomStyleButton::SidebarButton
                            | AtomStyleButton::SidebarButtonActive
                            | AtomStyleButton::Neutral => {
                                color!(0xF7F7F2)
                            }
                            _ => color!(0x30394c),
                        },
                        _ => self.style(class, button::Status::Active).text_color,
                    },
                    ..self.style(class, button::Status::Active)
                }
            }
            button::Status::Disabled => {
                let color_palette = class.appearance(self);

                button::Style {
                    background: match class {
                        AtomStyleButton::ShortcutKeyButton
                        | AtomStyleButton::Neutral
                        | AtomStyleButton::SidebarButtonActive
                        | AtomStyleButton::SidebarButton
                        | AtomStyleButton::HeaderButtons => match self {
                            AtomTheme::Hari => Some(Background::Color(
                                class.color_offset(color_palette.text, 5.0),
                            )),
                            _ => Some(Background::Color(
                                class.color_offset(color_palette.border, 5.0),
                            )),
                        },
                        _ => match self {
                            AtomTheme::Hari => Some(Background::Color(Color {
                                a: 0.2,
                                ..Color::BLACK
                            })),
                            _ => Some(Background::Color(color_palette.border)),
                        },
                    },
                    border: Border {
                        color: match class {
                            AtomStyleButton::PrimaryButton => color!(80, 80, 80, 0.4),
                            AtomStyleButton::ShortcutKeyButton => match self {
                                AtomTheme::Hari => color!(80, 80, 80, 0.4),
                                _ => self.style(class, button::Status::Active).border.color,
                            },
                            _ => self.style(class, button::Status::Active).border.color,
                        },
                        ..self.style(class, button::Status::Active).border
                    },
                    text_color: match class {
                        AtomStyleButton::SidebarButtonActive => color_palette.background,
                        AtomStyleButton::SidebarButton
                        | AtomStyleButton::ShortcutKeyButton
                        | AtomStyleButton::HeaderButtons
                        | AtomStyleButton::Neutral => color_palette.background,
                        _ => match self {
                            AtomTheme::Light => color_palette.background,
                            AtomTheme::Hari => color!(0xF7F7F2),
                            _ => Color::WHITE,
                        },
                    },
                    shadow: Shadow {
                        offset: match class {
                            AtomStyleButton::Neutral | AtomStyleButton::ShortcutKeyButton => {
                                Vector::new(0.0, 2.0)
                            }
                            _ => Default::default(),
                        },
                        ..Default::default()
                    },
                }
            }
            button::Status::Pressed => button::Style {
                ..self.style(class, button::Status::Active)
            },
        }
    }
}
