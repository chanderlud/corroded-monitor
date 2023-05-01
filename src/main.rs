#![windows_subsystem = "windows"]

mod ui;
mod system;

use serde_json::{Value};
use reqwest::Client;
use system::{cpu, gpu, ram};

#[derive(Debug, Clone)]
pub struct Data {
    // minimum: f32,
    maximum: f32,
    current: f32
}

impl Data {
    pub fn from_value(value: &Value) -> Self {
        Self {
            // minimum: strip_label(value["Min"].as_str().unwrap()),
            maximum: strip_label(value["Max"].as_str().unwrap()),
            current: strip_label(value["Value"].as_str().unwrap())
        }
    }
    
    pub fn default() -> Self {
        Self {
            // minimum: 0.0,
            maximum: 0.0,
            current: 0.0
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
    client: Client
}

impl SystemStats {
    fn new() -> Self {
        let client = Client::new();

        Self {
            cpu: cpu::Cpu::new(),
            gpu: gpu::Gpu::new(),
            ram: ram::Ram::new(),
            client
        }
    }

    async fn update(mut self) -> Self {
        let data = self.get_data().await;

        match data {
            Ok(data) => {
                self.cpu.update(&data);
                self.gpu.update(&data);
                self.ram.update(&data);
            }
            Err(e) => {
                println!("an error occurred while fetching data from OHM API: {}", e)
            }
        }

        self
    }

    async fn get_data(&self) -> Result<Value, reqwest::Error> {
        Ok(
            self.client
                .get("http://127.0.0.1:8085/data.json")
                .send().await?
                .json().await?
        )
    }

}

fn main() {
    //std::process::Command::new(format!("{}\\ohm\\OpenHardwareMonitor.exe", std::env::current_dir().unwrap().to_str().unwrap()))
    //    .spawn()
    //    .expect("failed to run");

    let r = ui::main();

    match r {
        Ok(_) => {}
        Err(e) => { println!("An error occurred: {:?}", e); std::thread::sleep(std::time::Duration::from_secs(60)); }
    }
}
