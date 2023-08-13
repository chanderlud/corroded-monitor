use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{color, BorderRadius, Color, Theme};

pub(crate) struct ComponentSelect;

impl button::StyleSheet for ComponentSelect {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: None,
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: color!(10, 10, 10),
                ..Appearance::default()
            },
            _ => Appearance {
                background: None,
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: Color::WHITE,
                ..Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Some(color!(214, 214, 214).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: color!(10, 10, 10),
                ..Appearance::default()
            },
            _ => Appearance {
                background: Some(color!(76, 78, 84).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: Color::WHITE,
                ..Appearance::default()
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Some(color!(234, 234, 234).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: color!(10, 10, 10),
                ..Appearance::default()
            },
            _ => Appearance {
                background: Some(color!(76, 78, 84).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: Color::WHITE,
                ..Appearance::default()
            },
        }
    }
}

pub(crate) struct SettingsButton;

impl button::StyleSheet for SettingsButton {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Some(color!(204, 204, 204).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: color!(10, 10, 10),
                ..Appearance::default()
            },
            _ => Appearance {
                background: Some(color!(46, 47, 50).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: Color::WHITE,
                ..Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Some(color!(180, 180, 180).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: color!(10, 10, 10),
                ..Appearance::default()
            },
            _ => Appearance {
                background: Some(color!(76, 78, 84).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: Color::WHITE,
                ..Appearance::default()
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Some(color!(180, 180, 180).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: color!(10, 10, 10),
                ..Appearance::default()
            },
            _ => Appearance {
                background: Some(color!(66, 68, 74).into()),
                border_radius: BorderRadius::from(8_f32),
                border_width: 0_f32,
                text_color: Color::WHITE,
                ..Appearance::default()
            },
        }
    }
}
