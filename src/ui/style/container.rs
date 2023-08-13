use iced::widget::container;
use iced::widget::container::Appearance;
use iced::{color, BorderRadius, Color, Theme};

pub(crate) struct MainBox;

impl container::StyleSheet for MainBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Some(color!(242, 242, 249).into()),
                border_radius: BorderRadius::from(0_f32),
                border_width: 0_f32,
                border_color: Default::default(),
            },
            _ => Appearance {
                text_color: Color::WHITE.into(),
                background: Some(color!(20, 20, 23).into()),
                border_radius: BorderRadius::from(0_f32),
                border_width: 0_f32,
                border_color: Default::default(),
            },
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
                background: Some(color!(230, 230, 230).into()),
                border_radius: BorderRadius::from(0_f32),
                border_width: 0_f32,
                border_color: Default::default(),
            },
            _ => Appearance {
                text_color: Color::WHITE.into(),
                background: Some(color!(36, 37, 40).into()),
                border_radius: BorderRadius::from(0_f32),
                border_width: 0_f32,
                border_color: Default::default(),
            },
        }
    }
}

pub(crate) struct TertiaryBox;

impl container::StyleSheet for TertiaryBox {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE.into(),
            background: Some(color!(34, 34, 38).into()),
            border_radius: BorderRadius::from(0_f32),
            border_width: 0_f32,
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
            color: color!(color.0, color.1, color.2),
        }
    }
}

impl container::StyleSheet for GraphBox {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Some(Color::TRANSPARENT.into()),
                border_radius: BorderRadius::from(4_f32),
                border_width: 2_f32,
                border_color: self.color,
            },
            _ => Appearance {
                text_color: None,
                background: Some(color!(34, 34, 38).into()),
                border_radius: BorderRadius::from(4_f32),
                border_width: 1_f32,
                border_color: self.color,
            },
        }
    }
}
