use iced::Color;
use iced::widget::scrollable;
use iced_style::scrollable::{Scrollbar, Scroller};
use iced_style::Theme;

pub(crate) struct Scrollable;

impl scrollable::StyleSheet for Scrollable {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Scrollbar {
        match style {
            Theme::Light => Scrollbar {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                scroller: Scroller {
                    color: Color::from_rgb8(200, 200, 200).into(),
                    border_radius: 4.0,
                    border_width: 0.0,
                    border_color: Default::default()
                }
            },
            _ => Scrollbar {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                scroller: Scroller {
                    color: Color::from_rgb8(66, 67, 70).into(),
                    border_radius: 4.0,
                    border_width: 0.0,
                    border_color: Default::default()
                }
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        match style {
            Theme::Light => Scrollbar {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                scroller: Scroller {
                    color: if is_mouse_over_scrollbar { Color::from_rgb8(170, 170, 170).into() } else { Color::from_rgb8(200, 200, 200).into() },
                    border_radius: 4.0,
                    border_width: 0.0,
                    border_color: Default::default()
                }
            },
            _ => Scrollbar {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                scroller: Scroller {
                    color: if is_mouse_over_scrollbar { Color::from_rgb8(96, 97, 100).into() } else { Color::from_rgb8(66, 67, 70).into() },
                    border_radius: 4.0,
                    border_width: 0.0,
                    border_color: Default::default()
                }
            }
        }
    }
}
