use super::{AtomSidebar, SideBarActiveButton, SideBarState, SidebarButton};
use crate::{
    elements::GuiElements,
    font::{icon, CustomFont},
    messages::SidebarMessage,
    style::{button::AtomStyleButton, container::AtomStyleContainer, AtomTheme},
    utils::helpers::SIDEBAR_WIDTH,
};
use iced::{
    widget::{button, column as col, container, row, text, tooltip, Column},
    Alignment, Element,
    Length::{Fill, Fixed, Shrink},
    Padding, Renderer,
};

impl AtomSidebar<'_> {
    fn render_icon_container<'a>(
        icon_char: char,
        size: f32,
    ) -> Element<'a, SidebarMessage, AtomTheme, Renderer> {
        container(icon(icon_char, CustomFont::IcoFont).size(size))
            .padding(Padding::new(15.0).right(20))
            .class(AtomStyleContainer::ButtonContainer)
            .into()
    }

    fn render_vertical_bar<'a>(
        is_active: bool,
    ) -> Element<'a, SidebarMessage, AtomTheme, Renderer> {
        container(text("").width(Fixed(0.0)))
            .padding(0)
            .height(Fixed(25.0))
            .width(Fixed(5.0))
            .class(if is_active {
                AtomStyleContainer::MenuBarActiveContainer
            } else {
                AtomStyleContainer::MenuBarInActiveContainer
            })
            .into()
    }

    fn build_sidebar_button<'a>(
        &'a self,
        is_active: bool,
        icon_char: char,
        label: &'a str,
        tooltip_text: &'a str,
        on_press_msg: SidebarMessage,
        downloads_list_is_empty: bool,
    ) -> Element<'a, SidebarMessage, AtomTheme, Renderer> {
        let content_row = row![
            Self::render_vertical_bar(is_active),
            Self::render_icon_container(icon_char, 20.0),
            text(label).width(Fill),
        ]
        .align_y(Alignment::Center)
        .spacing(0);

        let mut btn = button(
            container(content_row)
                .class(AtomStyleContainer::ButtonContainer)
                .center(Shrink)
                .padding(0),
        )
        .width(Shrink)
        .padding(1);

        if downloads_list_is_empty {
            btn = btn.on_press(SidebarMessage::GotoHomePage);
        } else {
            btn = btn.on_press(on_press_msg);
        }

        btn = btn.class(if is_active {
            AtomStyleButton::SidebarButtonActive
        } else {
            AtomStyleButton::SidebarButton
        });

        tooltip(btn, text(tooltip_text).size(12), tooltip::Position::Right)
            .gap(10)
            .padding(10)
            .class(AtomStyleContainer::ToolTipContainer)
            .into()
    }

    fn get_button<'a>(
        &'a self,
        data: &SidebarButton<'a>,
        downloads_list_is_empty: bool,
    ) -> Element<'a, SidebarMessage, AtomTheme, Renderer> {
        let is_active = self.active == data.name;
        let label = if matches!(self.state, SideBarState::Expanded) {
            data.text
        } else {
            ""
        };
        self.build_sidebar_button(
            is_active,
            data.icon,
            label,
            data.tooltip,
            data.message.clone(),
            downloads_list_is_empty,
        )
    }

    fn get_tertiary_button<'a>(&'a self) -> Element<'a, SidebarMessage, AtomTheme, Renderer> {
        let (icon_char, tooltip, msg) = if matches!(self.state, SideBarState::Expanded)
            && self.button_tertiary.text == "Collapse"
        {
            ('\u{eabf}', "Collapse sidebar", SidebarMessage::Collapse)
        } else {
            (
                self.button_tertiary.icon,
                self.button_tertiary.tooltip,
                self.button_tertiary.message.clone(),
            )
        };

        let label = if matches!(self.state, SideBarState::Collapsed) {
            ""
        } else if self.button_tertiary.text == "Collapse" {
            "Collapse"
        } else {
            "Expand"
        };

        col![self.build_sidebar_button(false, icon_char, label, tooltip, msg, false)]
            .spacing(10)
            .height(Shrink)
            .align_x(Alignment::Center)
            .into()
    }

    fn build_button_column<'a>(
        &'a self,
        buttons: &'a [SidebarButton<'a>],
        downloads_list_is_empty: bool,
    ) -> Column<'a, SidebarMessage, AtomTheme, Renderer> {
        buttons
            .iter()
            .fold(col!().spacing(10).height(Shrink), |col, b| {
                col.push(self.get_button(b, downloads_list_is_empty))
            })
    }

    pub fn view(
        &self,
        downloads_list_is_empty: bool,
        has_empty_search_bar: bool,
        scrollbar_visible: bool,
    ) -> Element<SidebarMessage, AtomTheme, Renderer> {
        let primary = self.build_button_column(&self.buttons_primary, false);
        let secondary = self.build_button_column(&self.buttons_secondary, downloads_list_is_empty);

        let sidebar_content = col![
            GuiElements::scrollbar(
                col!()
                    .spacing(10)
                    .push(primary)
                    .push(GuiElements::horizontal_separator())
                    .push(secondary)
                    .push(GuiElements::horizontal_separator()),
                scrollbar_visible
            ),
            self.get_tertiary_button()
        ]
        .spacing(10)
        .height(Fill);

        let sidebar_container = container(sidebar_content)
            .center(Shrink)
            .padding(0)
            .height(Fill)
            .width(Fill)
            .max_width(if matches!(self.state, SideBarState::Collapsed) {
                60
            } else {
                SIDEBAR_WIDTH
            })
            .class(AtomStyleContainer::ListContainer);

        if self.show_dialog {
            let delete_msg = format!(
                "Are you sure you want to delete {} downloads?",
                if has_empty_search_bar {
                    <SideBarActiveButton as Into<String>>::into(self.active.to_owned())
                } else {
                    "filtered".to_string()
                }
            );

            GuiElements::modal(
                sidebar_container,
                text(delete_msg).size(24),
                row![
                    GuiElements::primary_button(vec![
                        icon('\u{ec53}', CustomFont::IcoFont),
                        text("delete")
                    ])
                    .width(Fixed(170.0))
                    .on_press(SidebarMessage::DeleteAll),
                    GuiElements::primary_button(vec![
                        icon('\u{eedd}', CustomFont::IcoFont),
                        text("cancel")
                    ])
                    .width(Fixed(170.0))
                    .on_press(SidebarMessage::HideDialog),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                SidebarMessage::HideDialog,
            )
        } else {
            sidebar_container.into()
        }
    }
}
