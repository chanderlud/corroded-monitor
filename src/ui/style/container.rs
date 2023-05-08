use iced::Color;
use iced::widget::container;
use iced_style::container::Appearance;
use iced_style::Theme;

pub(crate) struct MainBox;

impl container::StyleSheet for MainBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Color::from_rgb8(242, 242, 249).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            },
            _ => Appearance {
                text_color: Color::WHITE.into(),
                background: Color::from_rgb8(20, 20, 23).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            }
        }
    }
}

pub(crate) struct SecondaryBox;

impl container::StyleSheet for SecondaryBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Color::from_rgb8(230, 230, 230).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            },
            _ => Appearance {
                text_color: Color::WHITE.into(),
                background: Color::from_rgb8(36, 37, 40).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            }
        }
    }
}

pub(crate) struct TertiaryBox;

impl container::StyleSheet for TertiaryBox {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE.into(),
            background: Color::from_rgb8(34, 34, 38).into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
        }
    }
}

pub(crate) struct GraphBox {
    color: Color,
}

impl GraphBox {
    pub(crate) fn new(color: (u8, u8, u8)) -> Self {
        Self {
            color: Color::from_rgb8(color.0, color.1, color.2),
        }
    }
}

impl container::StyleSheet for GraphBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Color::TRANSPARENT.into(),
                border_radius: 4.0,
                border_width: 2.0,
                border_color: self.color.into(),
            },
            _ => Appearance {
                text_color: None,
                background: Color::from_rgb8(34, 34, 38).into(),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: self.color.into(),
            }
        }
    }
}
