use iced::widget::scrollable;
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::{color, BorderRadius, Theme};

pub(crate) struct Scrollable;

impl scrollable::StyleSheet for Scrollable {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Scrollbar {
        match style {
            Theme::Light => Scrollbar {
                background: None,
                border_radius: BorderRadius::from(0_f32),
                border_width: 0_f32,
                border_color: Default::default(),
                scroller: Scroller {
                    color: color!(200, 200, 200),
                    border_radius: BorderRadius::from(4_f32),
                    border_width: 0_f32,
                    border_color: Default::default(),
                },
            },
            _ => Scrollbar {
                background: None,
                border_radius: BorderRadius::from(0_f32),
                border_width: 0_f32,
                border_color: Default::default(),
                scroller: Scroller {
                    color: color!(66, 67, 70),
                    border_radius: BorderRadius::from(4_f32),
                    border_width: 0_f32,
                    border_color: Default::default(),
                },
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        match style {
            Theme::Light => Scrollbar {
                background: None,
                border_radius: BorderRadius::from(0_f32),
                border_width: 0_f32,
                border_color: Default::default(),
                scroller: Scroller {
                    color: if is_mouse_over_scrollbar {
                        color!(170, 170, 170)
                    } else {
                        color!(200, 200, 200)
                    },
                    border_radius: BorderRadius::from(4_f32),
                    border_width: 0_f32,
                    border_color: Default::default(),
                },
            },
            _ => Scrollbar {
                background: None,
                border_radius: BorderRadius::from(0_f32),
                border_width: 0_f32,
                border_color: Default::default(),
                scroller: Scroller {
                    color: if is_mouse_over_scrollbar {
                        color!(96, 97, 100)
                    } else {
                        color!(66, 67, 70)
                    },
                    border_radius: BorderRadius::from(4_f32),
                    border_width: 0_f32,
                    border_color: Default::default(),
                },
            },
        }
    }
}
