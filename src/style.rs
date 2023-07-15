use iced::{
    application,
    widget::{
        button, container, pick_list, progress_bar,
        scrollable::{self, Scrollbar, Scroller},
        slider::{self, Handle, Rail},
        text, text_input, toggler,
    },
    Background, Color,
};
use iced_native::{overlay::menu, Vector};

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

const ACCENT_COLOR_BORDER: Color = color!(100, 100, 100);
const ATOM_WHITE: Color = color!(255, 255, 255, 1);
const ATOM_MAIN_CONTAINER_COLOR: Color = color!(20, 20, 20, 0.0); // #36393f
const ATOM_BUTTON_BACKGROUND: Color = color!(50, 50, 50);
const _ATOM_BUTTON_PRESSED: Color = color!(0x2c, 0xd6, 0x2c);
const ATOM_INPUT_BACK_COLOR: Color = color!(255, 255, 255, 1);
const ATOM_INPUT_BORDER_COLOR: Color = color!(215, 252, 112);
const ATOM_INPUT_BORDER_COLOR_HOVERED: Color = color!(215, 252, 112, 0.8);
const ATOM_INPUT_PLACEHOLDER_COLOR: Color = color!(250, 250, 250, 0.4);

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme {}

impl application::StyleSheet for Theme {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: ATOM_MAIN_CONTAINER_COLOR,
            text_color: ATOM_WHITE,
        }
    }
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
    DownloadFiltersButton,
}

impl button::StyleSheet for Theme {
    type Style = AtomStyleButton;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            AtomStyleButton::PrimaryButton => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(ATOM_INPUT_BORDER_COLOR)),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(30, 30, 30, 1),
                text_color: color!(0, 0, 0, 1),
            },
            AtomStyleButton::HeaderButtons => button::Appearance {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: color!(50, 50, 50),
                text_color: ATOM_WHITE,
            },
            AtomStyleButton::RoundButton => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(ATOM_INPUT_BORDER_COLOR)),
                border_radius: 50.0,
                border_width: 1.0,
                border_color: color!(50, 50, 50),
                text_color: color!(0, 0, 0),
            },
            AtomStyleButton::SidebarButton => button::Appearance {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(50, 50, 50, 0),
                text_color: ATOM_WHITE,
            },
            AtomStyleButton::SidebarButtonActive => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(color!(30, 30, 30, 0))),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(40, 40, 40, 0),
                text_color: ATOM_INPUT_BORDER_COLOR,
            },
            AtomStyleButton::Neutral => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(color!(50, 50, 50, 0))),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(50, 50, 50, 0),
                text_color: ATOM_WHITE,
            },
            AtomStyleButton::ShortcutKeyButton => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(color!(50, 50, 50, 0))),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(50, 50, 50, 0),
                text_color: ATOM_WHITE,
            },
            AtomStyleButton::DownloadFiltersButton => button::Appearance {
                background: Some(Background::Color(ATOM_INPUT_BORDER_COLOR)),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(30, 30, 30, 1),
                text_color: color!(0, 0, 0, 1),
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            AtomStyleButton::PrimaryButton => button::Appearance {
                background: Some(Background::Color(ATOM_INPUT_BORDER_COLOR_HOVERED)),
                border_color: ATOM_INPUT_BORDER_COLOR,
                ..self.active(style)
            },
            AtomStyleButton::HeaderButtons => button::Appearance {
                background: Some(Background::Color(color!(50, 50, 50))),
                border_color: color!(70, 70, 70),
                border_width: 0.0,
                text_color: color!(200, 200, 200),
                ..self.active(style)
            },
            AtomStyleButton::RoundButton => button::Appearance {
                background: Some(Background::Color(ATOM_INPUT_BORDER_COLOR_HOVERED)),
                border_color: color!(70, 70, 70),
                border_width: 2.0,
                ..self.active(style)
            },
            AtomStyleButton::SidebarButton => button::Appearance {
                background: Some(Background::Color(color!(30, 30, 30))),
                border_color: color!(40, 40, 40),
                text_color: ATOM_WHITE,
                ..self.active(style)
            },
            AtomStyleButton::SidebarButtonActive => button::Appearance {
                background: Some(Background::Color(color!(30, 30, 30))),
                border_color: color!(40, 40, 40),
                ..self.active(style)
            },
            AtomStyleButton::Neutral => button::Appearance {
                background: Some(Background::Color(color!(40, 40, 40))),
                ..self.active(style)
            },
            AtomStyleButton::ShortcutKeyButton => button::Appearance {
                background: Some(Background::Color(color!(40, 40, 40))),
                ..self.active(style)
            },
            AtomStyleButton::DownloadFiltersButton => button::Appearance {
                ..self.active(style)
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        match style {
            AtomStyleButton::PrimaryButton => button::Appearance {
                background: Some(Background::Color(color!(30, 30, 30))),
                border_color: color!(80, 80, 80, 0.4),
                text_color: ATOM_WHITE,
                ..self.active(style)
            },
            AtomStyleButton::HeaderButtons => button::Appearance {
                ..self.active(style)
            },
            AtomStyleButton::RoundButton => button::Appearance {
                background: Some(Background::Color(color!(50, 50, 50))),
                border_color: color!(70, 70, 70),
                border_width: 2.0,
                text_color: color!(200, 200, 200),
                ..self.active(style)
            },
            AtomStyleButton::SidebarButton => button::Appearance {
                background: Some(Background::Color(color!(50, 50, 50))),
                border_color: color!(50, 50, 50),
                text_color: ATOM_WHITE,
                ..self.active(style)
            },
            AtomStyleButton::SidebarButtonActive => button::Appearance {
                background: Some(Background::Color(color!(50, 50, 50))),
                border_color: color!(50, 50, 50),
                text_color: ATOM_WHITE,
                ..self.active(style)
            },
            AtomStyleButton::Neutral => button::Appearance {
                background: Some(Background::Color(color!(40, 40, 40))),
                shadow_offset: Vector::new(0.0, 2.0),
                ..self.active(style)
            },
            AtomStyleButton::ShortcutKeyButton => button::Appearance {
                background: Some(Background::Color(color!(40, 40, 40))),
                shadow_offset: Vector::new(0.0, 2.0),
                ..self.active(style)
            },
            AtomStyleButton::DownloadFiltersButton => button::Appearance {
                background: Some(Background::Color(color!(30, 30, 30))),
                border_color: color!(80, 80, 80, 0.4),
                text_color: ATOM_WHITE,
                ..self.active(style)
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        match style {
            AtomStyleButton::PrimaryButton => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(color!(50, 50, 50, 0))),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(50, 50, 50, 0),
                text_color: ATOM_WHITE,
            },
            AtomStyleButton::HeaderButtons => button::Appearance {
                ..self.active(style)
            },
            AtomStyleButton::RoundButton => button::Appearance {
                background: Some(Background::Color(color!(50, 50, 50))),
                border_color: color!(70, 70, 70),
                border_width: 2.0,
                text_color: ATOM_WHITE,
                ..self.active(style)
            },
            AtomStyleButton::SidebarButton => button::Appearance {
                background: Some(Background::Color(color!(50, 50, 50))),
                border_color: color!(50, 50, 50),
                text_color: ATOM_WHITE,
                ..self.active(style)
            },
            AtomStyleButton::SidebarButtonActive => button::Appearance {
                background: Some(Background::Color(color!(50, 50, 50))),
                border_color: color!(50, 50, 50),
                text_color: ATOM_WHITE,
                ..self.active(style)
            },
            AtomStyleButton::Neutral => button::Appearance {
                background: Some(Background::Color(color!(40, 40, 40))),
                ..self.active(style)
            },
            AtomStyleButton::ShortcutKeyButton => button::Appearance {
                background: Some(Background::Color(color!(40, 40, 40))),
                ..self.active(style)
            },
            AtomStyleButton::DownloadFiltersButton => button::Appearance {
                ..self.active(style)
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleContainer {
    MainBorderedContainer,
    #[default]
    MainContainer,
    LogoContainer,
    ListContainer,
    ListItemContainer,
    ErrorContainer,
    PreviewContainer,
    ButtonContainer,
    HeaderContainer,
    HeaderButtonsContainer,
    Transparent,
    ListHeaderContainer,
    ToolTipContainer,
    MenuBarActiveContainer,
    MenuBarInActiveContainer,
}

impl container::StyleSheet for Theme {
    type Style = AtomStyleContainer;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            AtomStyleContainer::MainBorderedContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: Some(iced::Background::Color(color!(10, 10, 10))),
                border_radius: 0.0,
                border_width: 1.0,
                border_color: color!(255, 255, 255, 0.02),
            },
            AtomStyleContainer::Transparent => container::Appearance {
                text_color: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: color!(30, 30, 30),
            },
            AtomStyleContainer::ButtonContainer => container::Appearance {
                text_color: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: color!(30, 30, 30),
            },
            AtomStyleContainer::HeaderContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: Some(iced::Background::Color(color!(20, 20, 20))),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: color!(30, 30, 30),
            },
            AtomStyleContainer::LogoContainer => container::Appearance {
                text_color: Some(color!(0, 0, 0)),
                background: Some(iced::Background::Color(ATOM_INPUT_BORDER_COLOR)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: ACCENT_COLOR_BORDER,
            },
            AtomStyleContainer::ListContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: Some(iced::Background::Color(color!(15, 15, 15))),
                border_radius: 10.0,
                border_width: 2.0,
                border_color: color!(28, 28, 28),
            },
            AtomStyleContainer::ListItemContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: Some(iced::Background::Color(color!(25, 25, 25))),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(40, 40, 40),
            },
            AtomStyleContainer::MainContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: Some(iced::Background::Color(color!(10, 10, 10))),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: ACCENT_COLOR_BORDER,
            },
            AtomStyleContainer::ErrorContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: Some(iced::Background::Color(color!(30, 30, 30))),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: color!(251, 50, 50, 0.7),
            },
            AtomStyleContainer::PreviewContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                // background: Some(iced::Background::Color(color!(232, 50, 116, 0.4))), // red background
                background: Some(iced::Background::Color(color!(100, 100, 100, 0.1))),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: ACCENT_COLOR_BORDER,
            },
            AtomStyleContainer::HeaderButtonsContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: color!(48, 48, 48),
            },
            AtomStyleContainer::ListHeaderContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: Some(iced::Background::Color(color!(37, 37, 37, 1))),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: ATOM_INPUT_BORDER_COLOR,
            },
            AtomStyleContainer::ToolTipContainer => container::Appearance {
                text_color: Some(ATOM_WHITE),
                background: Some(iced::Background::Color(color!(27, 27, 27, 1))),
                border_radius: 8.0,
                border_width: 0.5,
                border_color: color!(50, 50, 50),
            },
            AtomStyleContainer::MenuBarActiveContainer => container::Appearance {
                text_color: Some(ATOM_INPUT_BORDER_COLOR),
                background: Some(iced::Background::Color(ATOM_INPUT_BORDER_COLOR)),
                border_radius: 8.0,
                border_width: 0.1,
                border_color: color!(50, 50, 50),
            },
            AtomStyleContainer::MenuBarInActiveContainer => container::Appearance {
                text_color: None,
                background: None,
                border_color: color!(50, 50, 50, 0),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleText {
    #[default]
    Default,
    Dimmed,
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
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum AtomStyleInput {
    #[default]
    Default,
    Search,
    Disabled,
}

impl text_input::StyleSheet for Theme {
    type Style = AtomStyleInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            AtomStyleInput::Default => text_input::Appearance {
                background: Background::Color(Color::TRANSPARENT),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: ATOM_INPUT_BORDER_COLOR,
                icon_color: ATOM_INPUT_BORDER_COLOR,
            },
            AtomStyleInput::Search => text_input::Appearance {
                background: Background::Color(Color::TRANSPARENT),
                border_radius: 20.0,
                border_width: 1.0,
                border_color: color!(80, 80, 80),
                icon_color: ATOM_INPUT_BORDER_COLOR,
            },
            AtomStyleInput::Disabled => text_input::Appearance {
                background: Background::Color(color!(10, 10, 10, 1)),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: color!(100, 100, 100, 0.3),
                icon_color: ATOM_INPUT_BORDER_COLOR,
            },
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        color!(255, 255, 255, 0.3)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            AtomStyleInput::Default => text_input::Appearance {
                background: Background::Color(Color::TRANSPARENT),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: ATOM_INPUT_BORDER_COLOR,
                icon_color: ATOM_INPUT_BORDER_COLOR,
            },
            AtomStyleInput::Search => text_input::Appearance {
                background: Background::Color(Color::TRANSPARENT),
                border_radius: 20.0,
                border_width: 1.0,
                border_color: color!(80, 80, 80),
                icon_color: ATOM_INPUT_BORDER_COLOR,
            },
            AtomStyleInput::Disabled => text_input::Appearance {
                background: Background::Color(color!(10, 10, 10, 1)),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: color!(100, 100, 100, 0.3),
                icon_color: ATOM_INPUT_BORDER_COLOR,
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border_color: ATOM_INPUT_BORDER_COLOR_HOVERED,
            ..self.active(style)
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        ATOM_INPUT_PLACEHOLDER_COLOR
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        match style {
            AtomStyleInput::Default => ATOM_INPUT_BACK_COLOR,
            AtomStyleInput::Search => color!(200, 200, 200, 0.5),
            AtomStyleInput::Disabled => color!(100, 100, 100, 0.3),
        }
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        ATOM_BUTTON_BACKGROUND
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            AtomStyleInput::Default => text_input::Appearance {
                border_color: ATOM_INPUT_BORDER_COLOR_HOVERED,
                ..self.active(style)
            },
            AtomStyleInput::Search => text_input::Appearance {
                border_color: ATOM_INPUT_BORDER_COLOR_HOVERED,
                ..self.active(style)
            },
            AtomStyleInput::Disabled => text_input::Appearance {
                ..self.active(style)
            },
        }
    }
}

impl slider::StyleSheet for Theme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: Rail {
                colors: (ATOM_INPUT_BORDER_COLOR, ATOM_INPUT_BORDER_COLOR),
                width: 5.0,
            },
            handle: Handle {
                shape: slider::HandleShape::Circle { radius: 10.0 },
                color: color!(30, 30, 30),
                border_width: 2.0,
                border_color: ATOM_INPUT_BORDER_COLOR,
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: Rail {
                colors: (
                    ATOM_INPUT_BORDER_COLOR_HOVERED,
                    ATOM_INPUT_BORDER_COLOR_HOVERED,
                ),
                width: 5.0,
            },
            handle: Handle {
                shape: slider::HandleShape::Circle { radius: 10.0 },
                color: ATOM_BUTTON_BACKGROUND,
                border_width: 2.0,
                border_color: ATOM_INPUT_BORDER_COLOR,
            },
        }
    }

    fn dragging(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: Rail {
                colors: (
                    ATOM_INPUT_BORDER_COLOR_HOVERED,
                    ATOM_INPUT_BORDER_COLOR_HOVERED,
                ),
                width: 5.0,
            },
            handle: Handle {
                shape: slider::HandleShape::Circle { radius: 10.0 },
                color: ATOM_BUTTON_BACKGROUND,
                border_width: 2.0,
                border_color: ATOM_INPUT_BORDER_COLOR_HOVERED,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleScrollbar;

impl scrollable::StyleSheet for Theme {
    type Style = AtomStyleScrollbar;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        Scrollbar {
            border_radius: 5.0,
            border_width: 2.0,
            border_color: ATOM_BUTTON_BACKGROUND,
            background: Some(Background::Color(ATOM_BUTTON_BACKGROUND)),
            scroller: Scroller {
                // color: ATOM_INPUT_BORDER_COLOR,
                color: color!(50, 50, 50, 0),
                border_radius: 5.0,
                border_width: 3.0,
                // border_color: ATOM_INPUT_BORDER_COLOR,
                border_color: color!(50, 50, 50, 0),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> Scrollbar {
        Scrollbar {
            ..self.active(style)
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleProgressBar;

impl progress_bar::StyleSheet for Theme {
    type Style = AtomStyleProgressBar;

    fn appearance(&self, _style: &Self::Style) -> progress_bar::Appearance {
        progress_bar::Appearance {
            background: Background::Color(color!(100, 100, 100)),
            bar: Background::Color(ATOM_INPUT_BORDER_COLOR),
            border_radius: 5.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStylePickList;
impl pick_list::StyleSheet for Theme {
    type Style = Theme;

    fn active(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: ATOM_WHITE,
            placeholder_color: ATOM_INPUT_PLACEHOLDER_COLOR,
            background: Background::Color(Color::TRANSPARENT),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: ATOM_INPUT_BORDER_COLOR,
            handle_color: ATOM_INPUT_BORDER_COLOR,
        }
    }

    fn hovered(&self, style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            border_color: ATOM_INPUT_BORDER_COLOR_HOVERED,
            ..self.active(style)
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleOverlayMenu;

impl menu::StyleSheet for Theme {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: ATOM_WHITE,
            background: Background::Color(Color::TRANSPARENT),
            border_width: 1.0,
            border_radius: 2.0,
            border_color: ATOM_INPUT_BORDER_COLOR,
            selected_text_color: color!(0, 0, 0),
            selected_background: Background::Color(ATOM_INPUT_BORDER_COLOR),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleToggler;
impl toggler::StyleSheet for Theme {
    type Style = AtomStyleToggler;

    fn active(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        if is_active {
            toggler::Appearance {
                background: ATOM_INPUT_BORDER_COLOR,
                background_border: Some(ATOM_INPUT_BORDER_COLOR),
                foreground: color!(0, 0, 0, 1),
                foreground_border: Some(color!(200, 200, 200, 0.0)),
            }
        } else {
            toggler::Appearance {
                background: color!(200, 200, 200, 0),
                background_border: Some(ATOM_INPUT_BORDER_COLOR),
                foreground: ATOM_INPUT_BORDER_COLOR,
                foreground_border: Some(color!(230, 230, 230, 0.0)),
            }
        }
    }

    fn hovered(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        if is_active {
            toggler::Appearance {
                background: ATOM_INPUT_BORDER_COLOR,
                background_border: Some(ATOM_INPUT_BORDER_COLOR),
                foreground: color!(0, 0, 0, 1),
                foreground_border: Some(color!(0, 0, 0, 1)),
            }
        } else {
            toggler::Appearance {
                background: color!(200, 200, 200, 0),
                background_border: Some(ATOM_INPUT_BORDER_COLOR),
                foreground: ATOM_INPUT_BORDER_COLOR,
                foreground_border: Some(ATOM_INPUT_BORDER_COLOR),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtomStyleRule;

impl iced::widget::rule::StyleSheet for Theme {
    type Style = AtomStyleRule;

    fn appearance(&self, _style: &Self::Style) -> iced_native::widget::rule::Appearance {
        iced_native::widget::rule::Appearance {
            color: ATOM_INPUT_BORDER_COLOR,
            width: 5,
            radius: 5.0,
            fill_mode: iced_native::widget::rule::FillMode::Full,
        }
    }
}
