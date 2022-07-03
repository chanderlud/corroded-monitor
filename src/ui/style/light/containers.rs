use iced::{container, Color};
use iced::container::Style;

pub struct MainBox;

impl container::StyleSheet for MainBox {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Color::WHITE.into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default()
        }
    }
}

pub struct SecondaryBox;

impl container::StyleSheet for SecondaryBox {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Color::from_rgb8(242, 242, 249).into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default()
        }
    }
}

pub struct GraphBox {
    pub color: (u8, u8, u8)
}

impl container::StyleSheet for GraphBox {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Color::WHITE.into(),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(self.color.0, self.color.1, self.color.2).into()
        }
    }
}