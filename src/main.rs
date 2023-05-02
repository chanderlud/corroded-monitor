#![windows_subsystem = "windows"]

use std::process::Command;

use reqwest::Client;
use serde_json::Value;

use system::{cpu, gpu, ram};

mod ui;
mod system;

#[derive(Debug, Clone)]
pub struct Data {
    // minimum: f32,
    maximum: f32,
    current: f32,
}

impl Data {
    pub fn from_value(value: &Value) -> Self {
        Self {
            // minimum: strip_label(value["Min"].as_str().unwrap()),
            maximum: strip_label(value["Max"].as_str().unwrap()),
            current: strip_label(value["Value"].as_str().unwrap()),
        }
    }

    pub fn default() -> Self {
        Self {
            // minimum: 0.0,
            maximum: 0.0,
            current: 0.0,
        }
    }
}

fn strip_label(value: &str) -> f32 {
    match value.split(" ").collect::<Vec<&str>>()[0].parse::<f32>() {
        Ok(v) => v,
        Err(_) => 0.0,
    }
}

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu: cpu::Cpu,
    pub gpu: gpu::Gpu,
    pub ram: ram::Ram,
    client: Client,
}

impl SystemStats {
    fn new() -> Self {
        let client = Client::new();

        Self {
            cpu: cpu::Cpu::new(),
            gpu: gpu::Gpu::new(),
            ram: ram::Ram::new(),
            client,
        }
    }

    // update stats
    async fn update(mut self) -> Self {
        if let Ok(data) = self.get_data().await {
            self.cpu.update(&data);
            self.gpu.update(&data);
            self.ram.update(&data);
        } else {
            println!("an error occurred while fetching data from OHM API: {}", e)
        }

        self
    }

    // fetch OHM data from API
    async fn get_data(&self) -> Result<Value, reqwest::Error> {
        self.client
            .get("http://127.0.0.1:8085/data.json")
            .send().await?
            .json().await
    }
}

fn main() {
    Command::new(format!("{}\\ohm\\OpenHardwareMonitor.exe", std::env::current_dir().unwrap().to_str().unwrap()))
        .spawn()
        .expect("failed to run");

    if let Err(error) = ui::main() {
        println!("An error occurred: {:?}", error);
    }
}
