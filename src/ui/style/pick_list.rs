use iced::overlay::menu;
use iced::widget::pick_list;
use iced::widget::pick_list::Appearance;
use iced::{color, BorderRadius, Color, Theme};

pub(crate) struct PickList;

impl pick_list::StyleSheet for PickList {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: Color::BLACK,
                placeholder_color: Default::default(),
                handle_color: Color::BLACK,
                background: color!(234, 234, 234).into(),
                border_radius: BorderRadius::from(4_f32),
                border_width: 0_f32,
                border_color: Default::default(),
            },
            _ => Appearance {
                text_color: Color::WHITE,
                placeholder_color: Default::default(),
                handle_color: Color::WHITE,
                background: color!(46, 47, 50).into(),
                border_radius: BorderRadius::from(4_f32),
                border_width: 0_f32,
                border_color: Default::default(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: Color::BLACK,
                placeholder_color: Default::default(),
                handle_color: Color::BLACK,
                background: color!(234, 234, 234).into(),
                border_radius: BorderRadius::from(4_f32),
                border_width: 0_f32,
                border_color: Default::default(),
            },
            _ => Appearance {
                text_color: Color::WHITE,
                placeholder_color: Default::default(),
                handle_color: Color::WHITE,
                background: color!(46, 47, 50).into(),
                border_radius: BorderRadius::from(4_f32),
                border_width: 0_f32,
                border_color: Default::default(),
            },
        }
    }
}

impl menu::StyleSheet for PickList {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
        match style {
            Theme::Light => menu::Appearance {
                text_color: Color::BLACK,
                background: color!(242, 242, 249).into(),
                border_width: 1_f32,
                border_radius: BorderRadius::from(0_f32),
                border_color: color!(200, 200, 200),
                selected_text_color: Color::BLACK,
                selected_background: color!(220, 220, 220).into(),
            },
            _ => menu::Appearance {
                text_color: Color::WHITE,
                background: color!(34, 34, 38).into(),
                border_width: 1_f32,
                border_radius: BorderRadius::from(0_f32),
                border_color: color!(30, 30, 32),
                selected_text_color: Color::WHITE,
                selected_background: color!(63, 62, 65).into(),
            },
        }
    }
}
