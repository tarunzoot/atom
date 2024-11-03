use super::AtomTheme;
use crate::color;
use iced::{border::Radius, widget::text_input, Background, Border, Color};

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
    Dimmed,
}

impl AtomStyleInput {
    fn appearance(&self, theme: &AtomTheme) -> ColorPalette {
        let accent = theme.accent();
        match theme {
            AtomTheme::Default => ColorPalette {
                accent,
                background: Color::TRANSPARENT,
                hover_border: Color { a: 0.8, ..accent },
                placeholder: color!(250, 250, 250, 0.4),
                text: Color::WHITE,
                disabled_border: color!(100, 100, 100, 0.3),
            },
            AtomTheme::Tangerine => ColorPalette {
                accent,
                background: color!(49, 59, 69, 0.5),
                hover_border: Color { a: 0.8, ..accent },
                placeholder: color!(250, 250, 250, 0.4),
                text: Color::WHITE,
                disabled_border: Color::TRANSPARENT,
            },
            AtomTheme::Light => ColorPalette {
                accent,
                background: color!(255, 255, 255, 0.6),
                hover_border: Color { a: 0.8, ..accent },
                placeholder: color!(50, 50, 50, 0.6),
                text: Color::BLACK,
                disabled_border: color!(198, 202, 210, 1),
            },
            AtomTheme::Hari => ColorPalette {
                accent,
                // background: color!(255, 255, 255, 0.6),
                background: Color {
                    a: 0.8,
                    ..color!(0x30394c)
                },
                hover_border: Color { a: 0.8, ..accent },
                placeholder: Color {
                    a: 0.7,
                    ..color!(0xF7F7F2)
                },
                text: color!(0xF7F7F2),
                disabled_border: color!(198, 202, 210, 1),
            },
        }
    }
}

impl text_input::Catalog for AtomTheme {
    type Class<'a> = AtomStyleInput;

    fn default<'a>() -> Self::Class<'a> {
        AtomStyleInput::Default
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        match status {
            text_input::Status::Active => {
                let appearance = class.appearance(self);

                text_input::Style {
                    background: Background::Color(appearance.background),
                    icon: appearance.accent,
                    border: Border {
                        color: match self {
                            AtomTheme::Tangerine => Color::TRANSPARENT,
                            AtomTheme::Default => match class {
                                AtomStyleInput::Search => Color {
                                    a: 0.1,
                                    ..appearance.placeholder
                                },
                                AtomStyleInput::Dimmed => Color {
                                    a: 0.05,
                                    ..appearance.placeholder
                                },
                                _ => appearance.accent,
                            },
                            _ => match class {
                                AtomStyleInput::Search => Color {
                                    a: 0.4,
                                    ..appearance.placeholder
                                },
                                AtomStyleInput::Dimmed => Color {
                                    a: 0.5,
                                    ..appearance.placeholder
                                },
                                _ => appearance.accent,
                            },
                        },
                        width: 1.0,
                        radius: match class {
                            AtomStyleInput::Search => Radius::from(20.0),
                            _ => Radius::from(5.0),
                        },
                    },
                    value: appearance.text,
                    placeholder: appearance.placeholder,
                    selection: appearance.accent,
                }
            }
            text_input::Status::Hovered => {
                let appearance = class.appearance(self);
                text_input::Style {
                    background: Background::Color(appearance.background),
                    border: Border {
                        color: appearance.hover_border,
                        ..self.style(class, text_input::Status::Active).border
                    },
                    icon: appearance.accent,
                    placeholder: appearance.placeholder,
                    value: appearance.text,
                    selection: appearance.accent,
                }
            }
            text_input::Status::Focused => {
                let appearance = class.appearance(self);

                text_input::Style {
                    border: Border {
                        color: appearance.accent,
                        ..self.style(class, text_input::Status::Active).border
                    },
                    ..self.style(class, text_input::Status::Active)
                }
            }
            text_input::Status::Disabled => {
                let appearance = class.appearance(self);
                text_input::Style {
                    border: Border {
                        color: appearance.disabled_border,
                        ..self.style(class, text_input::Status::Active).border
                    },
                    icon: appearance.accent,
                    background: Background::Color(appearance.background),
                    placeholder: appearance.placeholder,
                    value: appearance.placeholder,
                    selection: appearance.accent,
                }
            }
        }
    }
}
