// #![windows_subsystem = "windows"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_char, CStr};
use std::sync::Arc;

use serde_json::from_slice;
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;

use system::{cpu, gpu, ram};

use crate::system::{Hardware, Sensor};
use crate::ui::HardwareMonitor;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod ui;
mod system;

#[derive(Default, Debug, Clone)]
pub struct Data {
    // minimum: f32,
    maximum: f32,
    current: f32,
}

impl Data {
    fn from(value: &Sensor) -> Self {
        Self {
            // minimum: value.min.unwrap_or(0.0),
            maximum: value.max.unwrap_or(0.0),
            current: value.value.unwrap_or(0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu: cpu::Cpu,
    pub gpu: gpu::Gpu,
    pub ram: ram::Ram,
}

impl SystemStats {
    fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            gpu: gpu::Gpu::new(),
            ram: ram::Ram::new(),
        }
    }

    // update stats
    async fn update(mut self, monitor: Arc<Mutex<HardwareMonitor>>) -> Self {
        let hardware_data: serde_json::Result<Vec<Hardware>> = spawn_blocking({
            move || {
                let ptr = monitor.blocking_lock().inner;
                unsafe { UpdateHardwareMonitor(ptr) };

                let mut buffer: Vec<c_char> = vec![0; 20000];

                unsafe { GetReport(ptr, buffer.as_mut_ptr(), buffer.len() as i32) };

                let report = unsafe { CStr::from_ptr(buffer.as_ptr()) };
                from_slice(report.to_bytes())
            }
        }).await.unwrap();

        match hardware_data {
            Ok(data) => {
                self.cpu.update(&data);
                self.gpu.update(&data);
                self.ram.update(&data);
            }
            Err(e) => println!("an error occurred while fetching data from OHM API: {:?}", e)
        }

        self
    }
}

/*
impl Drop for SystemStats {
    fn drop(&mut self) {
        unsafe {
            DestroyHardwareMonitor(self.monitor);
        }
    }
}
 */

fn main() { // TODO static linking
    if let Err(error) = ui::main() {
        println!("An error occurred: {:?}", error);
    }
}
