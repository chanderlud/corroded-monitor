use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

use iced::window::icon::from_rgba;
use iced::window::{PlatformSpecific, Settings as Window};
use iced::Settings;
use iced::{Application, Font};
use image::load_from_memory;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::config::Config;
use crate::system::{HardwareMonitor, SystemStats};
use crate::ui::app::App;

mod app;
pub(crate) mod chart;
pub(crate) mod style;

const ICON: &[u8] = include_bytes!("../../icon.ico");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum Theme {
    // use system theme
    System,
    // force dark theme
    Dark,
    // force light theme
    Light,
}

impl Theme {
    pub const ALL: [Self; 3] = [Self::System, Self::Dark, Self::Light];
}

// implement display for theme dropdown
impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::System => "System Default",
                Self::Dark => "Dark",
                Self::Light => "Light",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    // emitted every second to update the stats
    Update,
    // message contains the updated stats object
    UpdateCompleted(Box<(SystemStats, HashMap<String, bool>)>),
    // message contains the hardware monitor reference
    MonitorCreated(Arc<Mutex<HardwareMonitor>>),
    // message for navigating between pages
    Navigate(Route),
    // cpu pick list changed
    CpuPickChanged(crate::system::cpu::GraphState),
    // gpu pick list changed
    GpuPickChanged(crate::system::gpu::GraphState),
    // storage pick list changed
    StoragePickChanged(crate::system::storage::GraphState),
    // app theme changed
    ThemeChanged(Theme),
    // temperature unit changed
    TemperatureUnitChanged,
    // visibility changed
    VisibilityChanged((String, bool)),
}

// GUI routes
#[derive(Debug, Clone)]
pub(crate) enum Route {
    Cpu,
    Gpu(usize),
    Ram,
    Storage(usize),
    Network(usize),
    Settings,
}

// main GUI function
pub(crate) fn main() -> iced::Result {
    App::run(settings())
}

// GUI settings
fn settings() -> Settings<Config> {
    let icon = load_from_memory(ICON).unwrap();

    Settings {
        id: None,
        window: Window {
            size: (1300, 700),
            position: Default::default(),
            min_size: Some((1060, 500)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: Some(from_rgba(icon.to_rgba8().into_raw(), 32, 32).unwrap()),
            platform_specific: PlatformSpecific {
                parent: None,
                drag_and_drop: false, // allows the OHM wrapper to work
            },
            level: Default::default(),
        },
        flags: Config::load().expect("failed to load config"), // load config
        default_font: Font::DEFAULT,
        default_text_size: 20.0,
        exit_on_close_request: true,
        antialiasing: true,
    }
}
