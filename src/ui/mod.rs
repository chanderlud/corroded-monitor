use std::ffi::c_void;
use std::sync::Arc;

use iced::Application;
use iced::Settings;
use iced::window::{PlatformSpecific, Settings as Window};
use iced::window::icon::from_rgba;
use image::load_from_memory;
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;

use crate::CreateHardwareMonitor;
use crate::SystemStats;
use crate::ui::app::App;

pub mod style;
mod app;
pub mod chart;

const ICON: &[u8] = include_bytes!("../../icon.ico");

// a wrapper around the hardware monitor reference
#[derive(Debug)]
pub struct HardwareMonitor {
    pub(crate) inner: *mut c_void,
}

// this is okay because the hardware monitor is always inside a Arc<Mutex<T>>
unsafe impl Send for HardwareMonitor {}

impl HardwareMonitor {
    // asynchronously create a new hardware monitor reference
    async fn new() -> Arc<Mutex<Self>> {
        spawn_blocking(|| {
            let inner = unsafe { CreateHardwareMonitor() };
            Arc::new(Mutex::new(Self { inner }))
        }).await.unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    // emitted every second to update the stats
    UpdateCompleted(SystemStats),
    // message contains the updated stats object
    MonitorCreated(Arc<Mutex<HardwareMonitor>>),
    // message contains the hardware monitor reference
    Navigate(Route),
    // message for navigating between pages
    CpuPickChanged(crate::system::cpu::GraphState),
    // cpu pick list changed
    GpuPickChanged(crate::system::gpu::GraphState),  // gpu pick list changed
}

// GUI routes
#[derive(Debug, Clone)]
pub enum Route {
    Cpu,
    Gpu,
    Ram,
}

// main GUI function
pub fn main() -> iced::Result {
    App::run(settings())
}

// GUI settings
fn settings() -> Settings<()> {
    let icon = load_from_memory(ICON).unwrap();

    Settings {
        id: None,
        window: Window {
            size: (1300, 700),
            position: Default::default(),
            min_size: Some((1000, 500)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: Some(from_rgba(icon.to_rgba8().into_raw(), 32, 32).unwrap()),
            platform_specific: PlatformSpecific {
                parent: None,
                drag_and_drop: false, // allows the OHM wrapper to work
            },
        },
        flags: (),
        default_font: None,
        default_text_size: 20.0,
        exit_on_close_request: true,
        antialiasing: true,
        text_multithreading: true,
        try_opengles_first: false,
    }
}
