use serde::Deserialize;

pub mod gpu;
pub mod ram;
pub mod cpu;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Hardware {
    hardware_type: HardwareType,
    name: String,
    sub_hardware: Vec<Hardware>,
    sensors: Vec<Sensor>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Sensor {
    sensor_type: SensorType,
    name: String,
    index: i32,
    pub(crate) value: Option<f32>,
    pub(crate) max: Option<f32>,
    min: Option<f32>,
}

#[derive(Debug)]
pub enum HardwareType {
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

impl<'de> Deserialize<'de> for HardwareType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
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

#[derive(Debug, PartialEq)]
pub enum SensorType {
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

impl<'de> Deserialize<'de> for SensorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
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
            _ => Err(serde::de::Error::custom("Unexpected integer value for SensorType"))
        }
    }
}
