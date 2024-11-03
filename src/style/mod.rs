pub mod button;
pub mod container;
pub mod input;
pub mod progress;
pub mod slider;
pub mod toggle;

use iced::{
    application,
    border::Radius,
    overlay::menu,
    widget::{
        pick_list,
        scrollable::{self, Rail, Scroller},
        text,
    },
    Background, Border, Color,
};

#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color::from_rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::from_rgba(
            $r as f32 / 255.0,
            $g as f32 / 255.0,
            $b as f32 / 255.0,
            $a as f32,
        )
    };
    ($hex:expr) => {{
        let hex = $hex as u32;
        let r = (hex & 0xff0000) >> 16;
        let g = (hex & 0xff00) >> 8;
        let b = (hex & 0xff);
        Color::from_rgb8(r as u8, g as u8, b as u8)
    }};
    ($hex:expr, $a:expr) => {{
        let hex = $hex as u32;
        let r = (hex & 0xff0000) >> 16;
        let g = (hex & 0xff00) >> 8;
        let b = (hex & 0xff);
        Color::from_rgba8(r as u8, g as u8, b as u8, $a)
    }};
}

struct OverlayPalette {
    background: Color,
    _text: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomTheme {
    #[default]
    Default,
    Tangerine,
    Light,
    Hari,
}

impl AtomTheme {
    pub fn variants(&self) -> Vec<String> {
        vec![
            "Default".to_owned(),
            "Tangerine".to_owned(),
            "Light".to_owned(),
            "Hari".to_owned(),
        ]
    }

    pub fn accent(&self) -> Color {
        match self {
            AtomTheme::Default => color!(215, 252, 112),
            AtomTheme::Tangerine => color!(254, 161, 47, 1),
            AtomTheme::Light => color!(23, 29, 39, 1),
            AtomTheme::Hari => color!(0xE4E6C3), // moss green
                                                 // Theme::hari => color!(0x6ef7ff),
                                                 // Theme::Hari => color!(0xfb295d),
        }
    }

    fn palette(&self) -> OverlayPalette {
        match self {
            AtomTheme::Default => OverlayPalette {
                background: color!(10, 10, 10, 1),
                _text: Color::WHITE,
            },
            AtomTheme::Tangerine => OverlayPalette {
                background: color!(20, 24, 27, 1),
                _text: color!(192, 200, 201, 1),
            },
            AtomTheme::Light => OverlayPalette {
                background: color!(250, 250, 250, 1),
                _text: color!(23, 29, 39, 1),
            },
            AtomTheme::Hari => OverlayPalette {
                background: color!(0x30394c),
                _text: color!(0xF7F7F2),
            },
        }
    }
}

impl From<String> for AtomTheme {
    fn from(value: String) -> Self {
        match &value[..] {
            "Tangerine" => Self::Tangerine,
            "Light" => Self::Light,
            "Hari" => Self::Hari,
            _ => Self::Default,
        }
    }
}

impl application::DefaultStyle for AtomTheme {
    fn default_style(&self) -> application::Appearance {
        match self {
            AtomTheme::Default => application::Appearance {
                background_color: Color::TRANSPARENT,
                text_color: Color::WHITE,
            },
            AtomTheme::Tangerine => application::Appearance {
                background_color: color!(0x262e34),
                text_color: color!(0xffffff),
            },
            _ => application::Appearance {
                background_color: color!(0x262e34),
                text_color: color!(0x000000),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleText {
    #[default]
    Default,
    Dimmed,
    Accented,
}

impl text::Catalog for AtomTheme {
    type Class<'a> = AtomStyleText;

    fn default<'a>() -> Self::Class<'a> {
        AtomStyleText::Default
    }

    fn style(&self, item: &Self::Class<'_>) -> text::Style {
        match item {
            AtomStyleText::Default => text::Style {
                ..Default::default()
            },
            AtomStyleText::Dimmed => text::Style {
                color: Some(Color::from_rgb(180.0 / 255.0, 180.0 / 255.0, 180.0 / 255.0)),
            },
            AtomStyleText::Accented => text::Style {
                color: Some(AtomTheme::accent(self)),
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum AtomStyleScrollbar {
    #[default]
    Default,
}

impl scrollable::Catalog for AtomTheme {
    type Class<'a> = AtomStyleScrollbar;

    fn default<'a>() -> Self::Class<'a> {
        AtomStyleScrollbar::Default
    }

    fn style(&self, _class: &Self::Class<'_>, _status: scrollable::Status) -> scrollable::Style {
        let accent = self.accent();

        scrollable::Style {
            container: Default::default(),
            gap: Default::default(),
            vertical_rail: Rail {
                background: None,
                border: Border {
                    radius: Radius::from(5.0),
                    width: 2.0,
                    // border_color: ATOM_BUTTON_BACKGROUND,
                    // background: Some(Background::Color(ATOM_BUTTON_BACKGROUND)),
                    // color: accent,
                    color: Color::TRANSPARENT,
                },
                scroller: Scroller {
                    color: accent.scale_alpha(0.0),
                    border: Border {
                        radius: Radius::from(5.0),
                        width: 3.0,
                        color: accent.scale_alpha(0.0),
                    },
                },
            },
            horizontal_rail: Rail {
                background: None,
                border: Border {
                    radius: Radius::from(5.0),
                    width: 2.0,
                    // border_color: ATOM_BUTTON_BACKGROUND,
                    // background: Some(Background::Color(ATOM_BUTTON_BACKGROUND)),
                    color: Color::TRANSPARENT,
                },
                scroller: Scroller {
                    color: color!(50, 50, 50, 0),
                    border: Border {
                        radius: Radius::from(5.0),
                        width: 3.0,
                        color: color!(50, 50, 50, 0),
                    },
                },
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStylePickList {
    #[default]
    Default,
}

impl AtomStylePickList {
    fn color_palette(theme: &AtomTheme) -> (Color, Color) {
        match theme {
            AtomTheme::Default => (theme.accent(), color!(250, 250, 250, 0.4)),
            AtomTheme::Tangerine => (theme.accent(), color!(250, 250, 250, 0.4)),
            AtomTheme::Light => (theme.accent(), color!(250, 250, 250, 0.4)),
            AtomTheme::Hari => (theme.accent(), color!(0xF7F7F2)),
        }
    }
}

impl pick_list::Catalog for AtomTheme {
    type Class<'a> = AtomStylePickList;

    fn default<'a>() -> <Self as pick_list::Catalog>::Class<'a> {
        AtomStylePickList::Default
    }

    fn style(
        &self,
        _class: &<Self as pick_list::Catalog>::Class<'_>,
        status: pick_list::Status,
    ) -> pick_list::Style {
        let color_palette = AtomStylePickList::color_palette(self);

        match status {
            pick_list::Status::Active => pick_list::Style {
                text_color: match self {
                    AtomTheme::Light => color_palette.0,
                    _ => Color::WHITE,
                },
                placeholder_color: color_palette.1,
                background: Background::Color(Color::TRANSPARENT),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: color_palette.0,
                },
                handle_color: color_palette.0,
            },
            pick_list::Status::Hovered => {
                let active_style = self.style(_class, pick_list::Status::Active);
                pick_list::Style {
                    border: Border {
                        color: Color {
                            a: 0.8,
                            ..active_style.border.color
                        },
                        ..active_style.border
                    },
                    ..active_style
                }
            }
            pick_list::Status::Opened => self.style(_class, pick_list::Status::Active),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleOverlayMenu {
    #[default]
    Default,
}

impl menu::Catalog for AtomTheme {
    type Class<'a> = AtomStyleOverlayMenu;

    fn default<'a>() -> <Self as menu::Catalog>::Class<'a> {
        AtomStyleOverlayMenu::Default
    }

    fn style(&self, _class: &<Self as menu::Catalog>::Class<'_>) -> menu::Style {
        let accent = self.accent();
        let palette = self.palette();

        menu::Style {
            text_color: match self {
                AtomTheme::Light | AtomTheme::Hari => accent,
                _ => Color::WHITE,
            },
            background: palette.background.into(),
            border: Border {
                width: 1.0,
                radius: 5.0.into(),
                color: accent,
            },
            selected_text_color: match self {
                AtomTheme::Light => Color::WHITE,
                _ => Color::BLACK,
            },
            selected_background: Background::Color(accent),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleRule {
    #[default]
    Default,
}

impl iced::widget::rule::Catalog for AtomTheme {
    type Class<'a> = AtomStyleRule;

    fn default<'a>() -> Self::Class<'a> {
        AtomStyleRule::Default
    }

    fn style(&self, _class: &Self::Class<'_>) -> iced::widget::rule::Style {
        iced::widget::rule::Style {
            color: self.accent(),
            width: 5,
            radius: 5.0.into(),
            fill_mode: iced::widget::rule::FillMode::Full,
        }
    }
}
