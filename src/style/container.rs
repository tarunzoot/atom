use super::Theme;
use crate::color;
use iced::{border::Radius, widget::container, Background, Border, Color, Shadow};

struct ColorPalette {
    accent: Color,
    background: Color,
    border: Color,
    text: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleContainer {
    #[default]
    MainContainer,
    LogoContainer,
    ListContainer,
    ListItemContainer,
    PreviewContainer,
    ButtonContainer,
    HeaderContainer,
    HeaderButtonsContainer,
    Transparent,
    ListHeaderContainer,
    ToolTipContainer,
    MenuBarActiveContainer,
    MenuBarInActiveContainer,
    PillSuccess,
    PillError,
}

impl AtomStyleContainer {
    fn appearance(&self, theme: &Theme) -> ColorPalette {
        let accent = theme.accent();
        match theme {
            Theme::Default => ColorPalette {
                accent,
                background: color!(10, 10, 10, 1),
                border: color!(100, 100, 100, 1.0),
                text: Color::WHITE,
            },
            Theme::Tangerine => ColorPalette {
                accent,
                background: color!(20, 24, 27, 1),
                border: color!(44, 52, 61, 1),
                text: color!(192, 200, 201, 1),
            },
            Theme::Light => ColorPalette {
                accent,
                background: color!(250, 250, 250, 1),
                border: color!(150, 150, 150, 0.2),
                text: accent,
            },
            Theme::RedLight => ColorPalette {
                accent,
                background: color!(250, 250, 250, 1),
                border: color!(150, 150, 150, 0.2),
                text: Color {
                    a: 0.85,
                    ..Color::BLACK
                },
            },
        }
    }

    fn color_offset(&self, color: Color, offset: f32) -> Color {
        let new_offset = offset / 255.0;
        Color {
            r: if color.r + new_offset > 1.0 {
                color.r - new_offset
            } else {
                color.r + new_offset
            },
            g: if color.g + new_offset > 1.0 {
                color.g - new_offset
            } else {
                color.g + new_offset
            },
            b: if color.b + new_offset > 1.0 {
                color.b - new_offset
            } else {
                color.b + new_offset
            },
            a: color.a,
        }
    }
}

impl container::StyleSheet for Theme {
    type Style = AtomStyleContainer;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let appearance = style.appearance(self);

        container::Appearance {
            text_color: match style {
                AtomStyleContainer::LogoContainer => match self {
                    Theme::Light => Some(color!(255, 255, 255, 1)),
                    _ => Some(color!(0, 0, 0, 1)),
                },
                AtomStyleContainer::MenuBarActiveContainer => Some(color!(215, 252, 112)),
                AtomStyleContainer::MenuBarInActiveContainer
                | AtomStyleContainer::ButtonContainer => None,
                AtomStyleContainer::PillError => Some(appearance.text),
                AtomStyleContainer::PillSuccess => Some(style.color_offset(appearance.text, 180.0)),
                _ => Some(appearance.text),
            },
            shadow: Shadow::default(),
            background: match style {
                AtomStyleContainer::LogoContainer | AtomStyleContainer::MenuBarActiveContainer => {
                    Some(Background::Color(appearance.accent))
                }
                AtomStyleContainer::ListContainer => match self {
                    Theme::Default => Some(Background::Color(
                        style.color_offset(appearance.background, 5.0),
                    )),
                    Theme::Tangerine => Some(Background::Color(
                        style.color_offset(appearance.background, 10.0),
                    )),
                    Theme::Light => Some(Background::Color(Color {
                        a: 0.01,
                        ..appearance.border
                    })),
                    Theme::RedLight => Some(Background::Color(Color {
                        a: 0.01,
                        ..appearance.border
                    })),
                },
                AtomStyleContainer::HeaderContainer => match self {
                    Theme::Light => Some(Background::Color(
                        style.color_offset(appearance.background, 10.0),
                    )),
                    _ => Some(Background::Color(
                        style.color_offset(appearance.background, 10.0),
                    )),
                },
                AtomStyleContainer::PillSuccess => Some(Background::Color(appearance.accent)),
                AtomStyleContainer::PillError => Some(Background::Color(color!(251, 50, 50, 0.7))),
                AtomStyleContainer::Transparent
                | AtomStyleContainer::ButtonContainer
                | AtomStyleContainer::MenuBarInActiveContainer
                | AtomStyleContainer::HeaderButtonsContainer => None,
                AtomStyleContainer::ListItemContainer => Some(Background::Color(
                    style.color_offset(appearance.background, 15.0),
                )),
                AtomStyleContainer::PreviewContainer => Some(Background::Color(
                    style.color_offset(appearance.background, 20.0),
                )),
                AtomStyleContainer::ListHeaderContainer => Some(Background::Color(
                    style.color_offset(appearance.background, 30.0),
                )),
                AtomStyleContainer::ToolTipContainer => Some(Background::Color(
                    style.color_offset(appearance.background, 20.0),
                )),
                _ => Some(Background::Color(appearance.background)),
            },
            border: Border {
                radius: match style {
                    AtomStyleContainer::MenuBarActiveContainer
                    | AtomStyleContainer::ListContainer
                    | AtomStyleContainer::ToolTipContainer => Radius::from(10.0),
                    AtomStyleContainer::PreviewContainer
                    | AtomStyleContainer::ListHeaderContainer
                    | AtomStyleContainer::ListItemContainer => Radius::from(5.0),
                    AtomStyleContainer::PillError | AtomStyleContainer::PillSuccess => {
                        Radius::from(20.0)
                    }
                    _ => Radius::from(0.0),
                },
                width: match style {
                    AtomStyleContainer::ListContainer => 2.0,
                    AtomStyleContainer::ListItemContainer => 1.0,
                    AtomStyleContainer::ToolTipContainer => 0.5,
                    AtomStyleContainer::MenuBarActiveContainer => 0.1,
                    _ => 0.0,
                },
                color: match style {
                    AtomStyleContainer::MainContainer
                    | AtomStyleContainer::LogoContainer
                    | AtomStyleContainer::PreviewContainer => appearance.border,
                    AtomStyleContainer::ListItemContainer => {
                        style.color_offset(appearance.background, 30.0)
                    }
                    AtomStyleContainer::ListHeaderContainer => appearance.accent,
                    AtomStyleContainer::ToolTipContainer
                    | AtomStyleContainer::HeaderButtonsContainer
                    | AtomStyleContainer::MenuBarActiveContainer
                    | AtomStyleContainer::MenuBarInActiveContainer => {
                        style.color_offset(appearance.background, 40.0)
                    }
                    _ => style.color_offset(appearance.background, 20.0),
                },
            },
        }
    }
}
