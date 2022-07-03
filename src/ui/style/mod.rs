mod light;
mod dark;

use dark_light;
use iced::{button, container, pick_list};

pub enum Button {
    ComponentSelect
}

pub enum Container {
    Main,
    Secondary,
    Tertiary,
    Chart((u8, u8, u8))
}

pub enum PickList {
    Main
}

impl<'a> From<Button> for Box<dyn button::StyleSheet + 'a> {
    fn from(b: Button) -> Self {
        let mode = dark_light::detect();

        match mode {
            dark_light::Mode::Dark => {
                match b {
                    Button::ComponentSelect => dark::buttons::ComponentSelect.into(),
                }
            },
            dark_light::Mode::Light => {
                match b {
                    Button::ComponentSelect => light::buttons::ComponentSelect.into(),
                }
            },
        }
    }
}

impl<'a> From<PickList> for Box<dyn pick_list::StyleSheet + 'a> {
    fn from(b: PickList) -> Self {
        let mode = dark_light::detect();

        match mode {
            dark_light::Mode::Dark => {
                match b {
                    PickList::Main => dark::pick_list::PickList.into(),
                }
            },
            dark_light::Mode::Light => {
                match b {
                    PickList::Main => light::pick_list::PickList.into(),
                }
            },
        }
    }
}

impl<'a> From<Container> for Box<dyn container::StyleSheet + 'a> {
    fn from(b: Container) -> Self {
        let mode = dark_light::detect();

        match mode {
            dark_light::Mode::Dark => {
                match b {
                    Container::Main => dark::containers::MainBox.into(),
                    Container::Secondary => dark::containers::TertiaryBox.into(),
                    Container::Tertiary => dark::containers::SecondaryBox.into(),
                    Container::Chart(c) => dark::containers::GraphBox { color: c }.into(),
                }
            },
            dark_light::Mode::Light => {
                match b {
                    Container::Main => light::containers::MainBox.into(),
                    Container::Secondary => light::containers::SecondaryBox.into(),
                    Container::Tertiary => light::containers::MainBox.into(),
                    Container::Chart(c) => light::containers::GraphBox { color: c }.into(),
                }
            },
        }
    }
}

