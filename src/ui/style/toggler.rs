use iced::Color;
use iced::widget::toggler;
use iced_style::Theme;
use iced_style::toggler::Appearance;

pub(crate) struct Toggler;

impl toggler::StyleSheet for Toggler {
    type Style = Theme;

    fn active(&self, style: &Self::Style, _is_active: bool) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Color::from_rgb8(66, 67, 70).into(),
                background_border: None,
                foreground: Color::from_rgb8(0, 255, 255).into(),
                foreground_border: None,
            },
            _ => Appearance {
                background: Color::from_rgb8(0, 255, 255).into(),
                background_border: None,
                foreground: Color::from_rgb8(36, 37, 40).into(),
                foreground_border: None,
            }
        }
    }

    fn hovered(&self, style: &Self::Style, _is_active: bool) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: Color::from_rgb8(30, 30, 30).into(),
                background_border: None,
                foreground: Color::from_rgb8(0, 210, 210).into(),
                foreground_border: None,
            },
            _ => Appearance {
                background: Color::from_rgb8(0, 210, 210).into(),
                background_border: None,
                foreground: Color::from_rgb8(30, 30, 30).into(),
                foreground_border: None,
            }
        }
    }
}
