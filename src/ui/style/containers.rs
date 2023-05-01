use iced::{Color};
use iced::widget::container;
use iced_style::container::Appearance;
use iced_style::Theme;

pub struct MainBox;

impl container::StyleSheet for MainBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Color::WHITE.into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            },
            _ => Appearance {
                text_color: Color::WHITE.into(),
                background: Color::from_rgb8(20, 20, 23).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            }
        }
    }
}

pub struct SecondaryBox;

impl container::StyleSheet for SecondaryBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Color::from_rgb8(242, 242, 249).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            },
            _ => Appearance {
                text_color: Color::WHITE.into(),
                background: Color::from_rgb8(36, 37, 40).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            }
        }
    }
}

pub struct TertiaryBox;

impl container::StyleSheet for TertiaryBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            _ => Appearance {
                text_color: Color::WHITE.into(),
                background: Color::from_rgb8(34, 34, 38).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            }
        }
    }
}

pub struct GraphBox {
    pub color: (u8, u8, u8)
}

impl container::StyleSheet for GraphBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Color::WHITE.into(),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: Color::from_rgb8(self.color.0, self.color.1, self.color.2).into()
            },
            _ => Appearance {
                text_color: None,
                background: Color::from_rgb8(34, 34, 38).into(),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: Color::from_rgb8(self.color.0, self.color.1, self.color.2).into()
            }
        }
    }
}
