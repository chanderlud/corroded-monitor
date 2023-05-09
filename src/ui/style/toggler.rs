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
                background: Color::from_rgb8(183, 53, 90).into(),
                background_border: None,
                foreground: Color::from_rgb8(200, 200, 200).into(),
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
                background: Color::from_rgb8(153, 23, 60).into(),
                background_border: None,
                foreground: Color::from_rgb8(170, 170, 170).into(),
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

pub(crate) struct VisibilityToggler;

impl toggler::StyleSheet for VisibilityToggler {
    type Style = Theme;

    fn active(&self, style: &Self::Style, is_active: bool) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: if is_active { Color::from_rgb8(183, 53, 90).into() } else { Color::from_rgb8(180, 180, 180).into() },
                background_border: None,
                foreground: Color::from_rgb8(220, 220, 220).into(),
                foreground_border: None,
            },
            _ => Appearance {
                background: if is_active { Color::from_rgb8(0, 255, 255).into() } else { Color::from_rgb8(180, 180, 180).into() },
                background_border: None,
                foreground: Color::from_rgb8(36, 37, 40).into(),
                foreground_border: None,
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: if is_active { Color::from_rgb8(153, 23, 60).into() } else { Color::from_rgb8(180, 180, 180).into() },
                background_border: None,
                foreground: Color::from_rgb8(250, 250, 250).into(),
                foreground_border: None,
            },
            _ => Appearance {
                background: if is_active { Color::from_rgb8(0, 225, 225).into() } else { Color::from_rgb8(150, 150, 150).into() },
                background_border: None,
                foreground: Color::from_rgb8(30, 30, 30).into(),
                foreground_border: None,
            }
        }
    }
}
