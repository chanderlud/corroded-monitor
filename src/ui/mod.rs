use iced::Settings;
use iced::pure::Application;
use iced::window::{Settings as Window, Icon};
use image::load_from_memory;

use crate::SystemStats;

pub mod style;
mod app;
pub mod chart;

const ICON: &[u8] = include_bytes!("../../icon.ico");

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    Result(SystemStats),
    Navigate(Route),
    PickChanged(crate::system::cpu::GraphState)
}

#[derive(Debug, Clone)]
pub enum Route {
    Cpu,
    Gpu,
    Ram
}

pub fn main() -> iced::Result {
    app::App::run(settings())
}

fn settings() -> Settings<()> {
    let icon = load_from_memory(ICON).unwrap();

    Settings {
        id: None,
        window: Window {
            size: (1300, 700),
            min_size: Some((1000, 500)),
            resizable: true,
            decorations: true,
            icon: Some(Icon::from_rgba(icon.to_rgba8().into_raw(), 32, 32).unwrap()),
            ..Default::default()
        },
        flags: (),
        default_font: None,
        default_text_size: 20,
        exit_on_close_request: true,
        antialiasing: true,
        text_multithreading: true,
        try_opengles_first: false
    }
}