use iced::Color;
use iced::widget::button;
use iced_style::button::Appearance;
use iced_style::Theme;

pub struct ComponentSelect;

impl button::StyleSheet for ComponentSelect {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: None,
                border_radius: 8.0,
                border_width: 0.0,
                text_color: Color::from_rgb8(10, 10, 10).into(),
                ..Appearance::default()
            },
            _ => Appearance {
                background: None,
                border_radius: 8.0,
                border_width: 0.0,
                text_color: Color::WHITE,
                ..Appearance::default()
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Color::from_rgb8(234, 234, 234).into(),
                border_radius: 8.0,
                border_width: 0.0,
                text_color: Color::from_rgb8(10, 10, 10).into(),
                ..Appearance::default()
            },
            _ => Appearance {
                background: Color::from_rgb8(76, 78, 84).into(),
                border_radius: 8.0,
                border_width: 0.0,
                text_color: Color::WHITE,
                ..Appearance::default()
            }
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Color::from_rgb8(234, 234, 234).into(),
                border_radius: 8.0,
                border_width: 0.0,
                text_color: Color::from_rgb8(10, 10, 10).into(),
                ..Appearance::default()
            },
            _ => Appearance {
                background: Color::from_rgb8(76, 78, 84).into(),
                border_radius: 8.0,
                border_width: 0.0,
                text_color: Color::WHITE,
                ..Appearance::default()
            }
        }
    }
}
