use std::ffi::{c_char, c_void, CStr};
use std::fmt;
use std::sync::Arc;

use serde::{de, Deserialize, Deserializer};
use serde::de::Visitor;
use serde_json::from_slice;
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;

use crate::{CreateHardwareMonitor, GetReport, UpdateHardwareMonitor};
use crate::cpu::Cpu;
use crate::gpu::Gpu;
use crate::ram::Ram;

pub mod gpu;
pub mod ram;
pub mod cpu;

// a wrapper around the hardware monitor reference
#[derive(Debug)]
pub struct HardwareMonitor {
    pub(crate) inner: *mut c_void,
}

// this is okay because the hardware monitor is always inside a Arc<Mutex<T>>
unsafe impl Send for HardwareMonitor {}

impl HardwareMonitor {
    // asynchronously create a new hardware monitor reference
    pub async fn new() -> Arc<Mutex<Self>> {
        spawn_blocking(|| {
            let inner = unsafe { CreateHardwareMonitor() };
            Arc::new(Mutex::new(Self { inner }))
        }).await.unwrap()
    }
}

// the main structure that contains the hardware widgets
#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu: Cpu,
    pub gpu: Gpu,
    pub ram: Ram,
}

impl SystemStats {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            ram: Ram::new(),
        }
    }

    // update stats
    pub async fn update(mut self, monitor: Arc<Mutex<HardwareMonitor>>) -> Self {
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


// the main data structure for the OHM API, represents a single hardware component
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Hardware {
    // CPU, GPU, Memory, etc
    hardware_type: HardwareType,
    // a human readable name for the hardware
    name: String,
    // some hardware have sub-hardware
    sub_hardware: Vec<Hardware>,
    // the sensors for this hardware
    sensors: Vec<Sensor>,
}

// a sensor for a hardware component
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Sensor {
    // Temperature, Fan, Voltage, etc
    sensor_type: SensorType,
    // human readable name
    name: String,
    // index used for components like CPU cores
    index: usize,
    #[serde(deserialize_with = "deserialize_f32_or_nan_as_zero")]
    pub(crate) value: f32,
    #[serde(deserialize_with = "deserialize_f32_or_nan_as_zero")]
    pub(crate) max: f32,
    // #[serde(deserialize_with = "deserialize_f32_or_nan_as_zero")]
    // pub(crate) min: f32,
}

fn deserialize_f32_or_nan_as_zero<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
{
    struct F32OrNaNAsZeroVisitor;

    impl<'de> Visitor<'de> for F32OrNaNAsZeroVisitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a float or the string \"NaN\"")
        }

        fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            if value.to_lowercase() == "nan" {
                Ok(0.0)
            } else {
                Err(E::custom(format!("expected \"NaN\" or a float, got {}", value)))
            }
        }
    }

    deserializer.deserialize_any(F32OrNaNAsZeroVisitor)
}

// hardware types
#[derive(Debug)]
pub(crate) enum HardwareType {
    Motherboard,
    SuperIO,
    Cpu,
    Memory,
    GpuNvidia,
    GpuAmd,
    GpuIntel,
    Storage,
    Network,
    Cooler,
    EmbeddedController,
    Psu,
    Battery,
}

// deserialize hardware type from int
impl<'de> Deserialize<'de> for HardwareType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let num = i32::deserialize(deserializer)?;

        match num {
            0 => Ok(Self::Motherboard),
            1 => Ok(Self::SuperIO),
            2 => Ok(Self::Cpu),
            3 => Ok(Self::Memory),
            4 => Ok(Self::GpuNvidia),
            5 => Ok(Self::GpuAmd),
            6 => Ok(Self::GpuIntel),
            7 => Ok(Self::Storage),
            8 => Ok(Self::Network),
            9 => Ok(Self::Cooler),
            10 => Ok(Self::EmbeddedController),
            11 => Ok(Self::Psu),
            12 => Ok(Self::Battery),
            _ => Err(serde::de::Error::custom("Unexpected integer value for HardwareType"))
        }
    }
}

// sensor types
#[derive(Debug, PartialEq)]
pub(crate) enum SensorType {
    Voltage,
    // V
    Current,
    // A
    Power,
    // W
    Clock,
    // MHz
    Temperature,
    // °C
    Load,
    // %
    Frequency,
    // Hz
    Fan,
    // RPM
    Flow,
    // L/h
    Control,
    // %
    Level,
    // %
    Factor,
    // 1
    Data,
    // GB = 2^30 Bytes
    SmallData,
    // MB = 2^20 Bytes
    Throughput,
    // B/s
    TimeSpan,
    // Seconds
    Energy,
    // milliwatt-hour (mWh)
    Noise, // dBA
}

// deserialize sensor type from int
impl<'de> Deserialize<'de> for SensorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let num = i32::deserialize(deserializer)?;

        match num {
            0 => Ok(Self::Voltage),
            1 => Ok(Self::Current),
            2 => Ok(Self::Power),
            3 => Ok(Self::Clock),
            4 => Ok(Self::Temperature),
            5 => Ok(Self::Load),
            6 => Ok(Self::Frequency),
            7 => Ok(Self::Fan),
            8 => Ok(Self::Flow),
            9 => Ok(Self::Control),
            10 => Ok(Self::Level),
            11 => Ok(Self::Factor),
            12 => Ok(Self::Data),
            13 => Ok(Self::SmallData),
            14 => Ok(Self::Throughput),
            15 => Ok(Self::TimeSpan),
            16 => Ok(Self::Energy),
            17 => Ok(Self::Noise),
            _ => Err(de::Error::custom("Unexpected integer value for SensorType"))
        }
    }
}

// used in the hardware widgets to store data
#[derive(Default, Debug, Clone)]
pub(crate) struct Data {
    // minimum: f32,
    maximum: f32,
    current: f32,
}

impl Data {
    // data from sensor
    fn from(value: &Sensor) -> Self {
        Self {
            // minimum: value.min.unwrap_or(0.0),
            maximum: value.max,
            current: value.value,
        }
    }
}
