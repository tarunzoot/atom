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
        scrollable::{self, Scrollbar, Scroller},
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

#[derive(Debug, Clone, Copy, Default)]
pub enum Theme {
    #[default]
    Default,
    Tangerine,
    Light,
    Hari,
}

impl Theme {
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
            Theme::Default => color!(215, 252, 112),
            Theme::Tangerine => color!(254, 161, 47, 1),
            Theme::Light => color!(23, 29, 39, 1),
            Theme::Hari => color!(0xE4E6C3), // moss green
                                             // Theme::hari => color!(0x6ef7ff),
                                             // Theme::Hari => color!(0xfb295d),
        }
    }
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match &value[..] {
            "Tangerine" => Self::Tangerine,
            "Light" => Self::Light,
            "Hari" => Self::Hari,
            _ => Self::Default,
        }
    }
}

impl application::StyleSheet for Theme {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> application::Appearance {
        match self {
            Theme::Default => application::Appearance {
                background_color: Color::TRANSPARENT,
                text_color: Color::WHITE,
            },
            Theme::Tangerine => application::Appearance {
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

impl text::StyleSheet for Theme {
    type Style = AtomStyleText;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            AtomStyleText::Default => text::Appearance {
                ..Default::default()
            },
            AtomStyleText::Dimmed => text::Appearance {
                color: Some(Color::from_rgb(180.0 / 255.0, 180.0 / 255.0, 180.0 / 255.0)),
            },
            AtomStyleText::Accented => text::Appearance {
                color: Some(Theme::accent(self)),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleScrollbar;

impl scrollable::StyleSheet for Theme {
    type Style = AtomStyleScrollbar;

    fn active(&self, _style: &Self::Style) -> scrollable::Appearance {
        scrollable::Appearance {
            container: Default::default(),
            scrollbar: Scrollbar {
                border: Border {
                    radius: Radius::from(5.0),
                    width: 2.0,
                    // border_color: ATOM_BUTTON_BACKGROUND,
                    // background: Some(Background::Color(ATOM_BUTTON_BACKGROUND)),
                    color: Color::TRANSPARENT,
                },
                background: None,
                scroller: Scroller {
                    color: color!(50, 50, 50, 0),
                    border: Border {
                        radius: Radius::from(5.0),
                        width: 3.0,
                        color: color!(50, 50, 50, 0),
                    },
                },
            },
            gap: Default::default(),
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        self.active(style)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStylePickList;

impl AtomStylePickList {
    fn color_palette(theme: &Theme) -> (Color, Color) {
        match theme {
            Theme::Default => (theme.accent(), color!(250, 250, 250, 0.4)),
            Theme::Tangerine => (theme.accent(), color!(250, 250, 250, 0.4)),
            Theme::Light => (theme.accent(), color!(250, 250, 250, 0.4)),
            Theme::Hari => (theme.accent(), color!(0xF7F7F2)),
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = Theme;

    fn active(&self, _: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        let color_palette = AtomStylePickList::color_palette(self);

        pick_list::Appearance {
            text_color: match self {
                Theme::Light => color_palette.0,
                _ => Color::WHITE,
            },
            placeholder_color: color_palette.1,
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                radius: Radius::from(5.0),
                width: 1.0,
                color: color_palette.0,
            },
            handle_color: color_palette.0,
        }
    }

    fn hovered(&self, style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            border: Border {
                color: Color {
                    a: 0.8,
                    ..self.active(style).border.color
                },
                ..self.active(style).border
            },
            ..self.active(style)
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleOverlayMenu;

impl menu::StyleSheet for Theme {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> menu::Appearance {
        let color_palette = self.accent();

        menu::Appearance {
            text_color: match self {
                Theme::Light | Theme::Hari => color_palette,
                _ => Color::WHITE,
            },
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                width: 1.0,
                radius: Radius::from(2.0),
                color: color_palette,
            },
            selected_text_color: match self {
                Theme::Light => Color::WHITE,
                _ => Color::BLACK,
            },
            selected_background: Background::Color(color_palette),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleRule;

impl iced::widget::rule::StyleSheet for Theme {
    type Style = AtomStyleRule;

    fn appearance(&self, _: &Self::Style) -> iced::widget::rule::Appearance {
        iced::widget::rule::Appearance {
            color: self.accent(),
            width: 5,
            radius: Radius::from(5.0),
            fill_mode: iced::widget::rule::FillMode::Full,
        }
    }
}
