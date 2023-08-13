use iced::widget::toggler;
use iced::widget::toggler::Appearance;
use iced::{color, Theme};

pub(crate) struct Toggler;

impl toggler::StyleSheet for Toggler {
    type Style = Theme;

    fn active(&self, style: &Self::Style, _is_active: bool) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: color!(183, 53, 90),
                background_border: None,
                foreground: color!(200, 200, 200),
                foreground_border: None,
            },
            _ => Appearance {
                background: color!(0, 255, 255),
                background_border: None,
                foreground: color!(36, 37, 40),
                foreground_border: None,
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_active: bool) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: color!(153, 23, 60),
                background_border: None,
                foreground: color!(170, 170, 170),
                foreground_border: None,
            },
            _ => Appearance {
                background: color!(0, 210, 210),
                background_border: None,
                foreground: color!(30, 30, 30),
                foreground_border: None,
            },
        }
    }
}

pub(crate) struct VisibilityToggler;

impl toggler::StyleSheet for VisibilityToggler {
    type Style = Theme;

    fn active(&self, style: &Self::Style, is_active: bool) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: if is_active {
                    color!(183, 53, 90)
                } else {
                    color!(180, 180, 180)
                },
                background_border: None,
                foreground: color!(220, 220, 220),
                foreground_border: None,
            },
            _ => Appearance {
                background: if is_active {
                    color!(0, 255, 255)
                } else {
                    color!(180, 180, 180)
                },
                background_border: None,
                foreground: color!(36, 37, 40),
                foreground_border: None,
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> Appearance {
        match style {
            Theme::Light => Appearance {
                background: if is_active {
                    color!(153, 23, 60)
                } else {
                    color!(180, 180, 180)
                },
                background_border: None,
                foreground: color!(250, 250, 250),
                foreground_border: None,
            },
            _ => Appearance {
                background: if is_active {
                    color!(0, 225, 225)
                } else {
                    color!(150, 150, 150)
                },
                background_border: None,
                foreground: color!(30, 30, 30),
                foreground_border: None,
            },
        }
    }
}
