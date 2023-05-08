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
use crate::storage::Storage;
use crate::system::network::NetworkAdapter;

pub(crate) mod gpu;
pub(crate) mod ram;
pub(crate) mod cpu;
pub(crate) mod storage;
pub(crate) mod network;

// a wrapper around the hardware monitor reference
#[derive(Debug)]
pub(crate) struct HardwareMonitor {
    pub(crate) inner: *mut c_void,
}

// this is okay because the hardware monitor is always used inside a Arc<Mutex<T>>
unsafe impl Send for HardwareMonitor {}

impl HardwareMonitor {
    // asynchronously create a new hardware monitor reference
    pub(crate) async fn new() -> Arc<Mutex<Self>> {
        spawn_blocking(|| {
            let inner = unsafe { CreateHardwareMonitor() };
            Arc::new(Mutex::new(Self { inner }))
        }).await.unwrap()
    }
}

// the main structure that contains the hardware widgets
#[derive(Debug, Clone)]
pub(crate) struct SystemStats {
    // multiple CPUs is rare, not supported
    pub(crate) cpu: Cpu,
    // support multiple GPUs
    pub(crate) gpus: Vec<Gpu>,
    pub(crate) ram: Ram,
    pub(crate) disks: Vec<Storage>, // support multiple disks
    pub(crate) network_adapters: Vec<NetworkAdapter>
}

impl SystemStats {
    pub(crate) fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            gpus: Vec::new(),
            ram: Ram::new(),
            disks: Vec::new(),
            network_adapters: Vec::new(),
        }
    }

    // update stats
    pub(crate) async fn update(mut self, monitor: Arc<Mutex<HardwareMonitor>>) -> Self {
        // fetch data from OHM API, spawn_blocking is used to prevent blocking
        let hardware_data: serde_json::Result<Vec<Hardware>> = spawn_blocking({
            move || {
                let ptr = monitor.blocking_lock().inner; // lock pointer to OHM
                unsafe { UpdateHardwareMonitor(ptr) }; // update OHM

                let mut buffer: Vec<c_char> = vec![0; 20000]; // allocate buffer for data

                unsafe { GetReport(ptr, buffer.as_mut_ptr(), buffer.len() as i32) }; // load data into buffer

                let report = unsafe { CStr::from_ptr(buffer.as_ptr()) }; // convert buffer to CStr
                from_slice(report.to_bytes()) // deserialize CStr to Vec<Hardware>
            }
        }).await.unwrap(); // unwrap thread, should never panic

        match hardware_data {
            Ok(data) => {
                let mut disk_index = 0;
                let mut gpu_index = 0;
                let mut network_index = 0;

                // iterate over hardware devices
                for device in data {
                    match device.hardware_type {
                        HardwareType::Cpu => {
                            self.cpu.update(&device);
                        }
                        HardwareType::GpuNvidia | HardwareType::GpuAmd | HardwareType::GpuIntel => {
                            // create new gpu if needed
                            if self.gpus.len() == gpu_index {
                                self.gpus.push(Gpu::new());
                            }

                            self.gpus[gpu_index].update(&device, gpu_index);
                            gpu_index += 1;
                        }
                        HardwareType::Memory => {
                            self.ram.update(&device);
                        }
                        HardwareType::Storage => {
                            // create new disk if needed
                            if self.disks.len() == disk_index {
                                self.disks.push(Storage::new());
                            }

                            self.disks[disk_index].update(&device, disk_index);
                            disk_index += 1;
                        }
                        HardwareType::Network => {
                            // skip non-ethernet and non-wifi adapters
                            if device.name != "Ethernet" && device.name != "Wi-Fi" {
                                continue
                            }

                            // create network adapter if needed
                            if self.network_adapters.len() == network_index {
                                self.network_adapters.push(NetworkAdapter::new());
                            }

                            self.network_adapters[network_index].update(&device, network_index);
                            network_index += 1;
                        }
                        _ => {}
                    }
                }
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
pub(crate) struct Hardware {
    // CPU, GPU, Memory, etc
    hardware_type: HardwareType,
    // a human readable name for the hardware
    name: String,
    // some hardware have sub-hardware
    // sub_hardware: Vec<Hardware>,
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
            if value.to_lowercase() == "nan" || value.to_lowercase() == "infinity" {
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
            _ => Err(de::Error::custom("Unexpected integer value for HardwareType"))
        }
    }
}

// sensor types
#[derive(Debug, PartialEq)]
pub(crate) enum SensorType {
    // V
    Voltage,
    // A
    Current,
    // W
    Power,
    // MHz
    Clock,
    // °C
    Temperature,
    // %
    Load,
    // Hz
    Frequency,
    // RPM
    Fan,
    // L/h
    Flow,
    // %
    Control,
    // %
    Level,
    // 1
    Factor,
    // GB = 2^30 Bytes
    Data,
    // MB = 2^20 Bytes
    SmallData,
    // B/s
    Throughput,
    // Seconds
    TimeSpan,
    // milliwatt-hour (mWh)
    Energy,
    // dBA
    Noise,
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
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct Data {
    // minimum: f32,
    maximum: f32,
    current: f32,
}

impl Data {
    // data from sensor
    fn from(value: &Sensor) -> Self {
        Self {
            // minimum: value.min,
            maximum: value.max,
            current: value.value,
        }
    }
}
