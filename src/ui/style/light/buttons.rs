use iced::{button, Color};

pub struct ComponentSelect;

impl button::StyleSheet for ComponentSelect {
    fn active(&self) -> button::Style {
        button::Style {
            background: None,
            border_radius: 8.0,
            border_width: 0.0,
            text_color: Color::from_rgb8(10, 10, 10).into(),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Color::from_rgb8(234, 234, 234).into(),
            border_radius: 8.0,
            border_width: 0.0,
            text_color: Color::from_rgb8(10, 10, 10).into(),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Color::from_rgb8(234, 234, 234).into(),
            border_radius: 8.0,
            border_width: 0.0,
            text_color: Color::from_rgb8(10, 10, 10).into(),
            ..button::Style::default()
        }
    }
}