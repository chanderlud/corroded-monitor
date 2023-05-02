use iced::{Color};
use iced::widget::pick_list;
use iced_style::{menu, Theme};
use iced_style::pick_list::Appearance;

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE.into(),
            placeholder_color: Default::default(),
            handle_color: Color::WHITE.into(),
            background: Color::TRANSPARENT.into(),
            border_radius: 4.0,
            border_width: 0.0,
            border_color: Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE.into(),
            placeholder_color: Default::default(),
            handle_color: Color::WHITE.into(),
            background: Color::TRANSPARENT.into(),
            border_radius: 4.0,
            border_width: 0.0,
            border_color: Default::default()
        }
    }
}

impl menu::StyleSheet for PickList {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: Color::WHITE.into(),
            background: Color::from_rgb8(34, 34, 38).into(),
            border_width: 1.0,
            border_radius: 0.0,
            border_color: Color::from_rgb8(30, 30, 32).into(),
            selected_text_color: Color::WHITE.into(),
            selected_background: Color::from_rgb8(63, 62, 65).into()
        }
    }
}
