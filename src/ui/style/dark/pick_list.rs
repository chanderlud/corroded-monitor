use iced::{pick_list, Color};
use iced::pick_list::{Menu, Style};

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    fn menu(&self) -> Menu {
        Menu {
            text_color: Color::WHITE.into(),
            background: Color::from_rgb8(36, 37, 40).into(),
            border_width: 0.5,
            border_color: Color::from_rgb8(36, 37, 40).into(),
            selected_text_color: Color::WHITE,
            selected_background: Color::from_rgb8(63, 62, 65).into()
        }
    }

    fn active(&self) -> Style {
        Style {
            text_color: Color::WHITE.into(),
            background: Color::TRANSPARENT.into(),
            border_radius: 4.0,
            border_width: 0.0,
            border_color: Default::default(),
            icon_size: 0.8
        }
    }

    fn hovered(&self) -> Style {
        Style {
            text_color: Color::WHITE.into(),
            background: Color::TRANSPARENT.into(),
            border_radius: 4.0,
            border_width: 0.0,
            border_color: Default::default(),
            icon_size: 0.8
        }
    }
}