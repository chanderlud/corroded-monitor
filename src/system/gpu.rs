use iced::{Align, button, Button, Column, Container, Element, Length, Row, Space, Text};
use serde_json::Value;

use crate::Data;
use crate::ui::{chart::{StatChart, Size}, Message, Route, style};

#[derive(Debug, Clone)]
enum DataType {
    Temperature,
    Load,
    Frequency,
    Power,
    Fan,
    Memory,
    None
}

#[derive(Debug, Clone)]
pub struct GpuClock {
    pub core: Data,
    core_graph: StatChart,
    pub memory: Data,
    pub shader: Data,
}

impl GpuClock {
    fn default() -> Self {
        Self {
            core: Data::default(),
            core_graph: StatChart::new((255, 190, 125), Size::Large),
            memory: Data::default(),
            shader: Data::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GpuMemory {
    pub free: Data,
    pub used: Data,
    pub total: f32,
}

impl GpuMemory {
    fn default() -> Self {
        Self {
            free: Data::default(),
            used: Data::default(),
            total: 0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct GpuLoad {
    pub core: Data,
    core_graph: StatChart,
    pub memory: Data,
    memory_graph: StatChart,
    pub frame_buffer: Data,
    pub video_engine: Data,
    pub bus_interface: Data,
}

impl GpuLoad {
    fn default() -> Self {
        Self {
            core: Data::default(),
            core_graph: StatChart::new((255, 190, 125), Size::Large),
            memory: Data::default(),
            memory_graph: StatChart::new((255, 190, 125), Size::Large),
            frame_buffer: Data::default(),
            video_engine: Data::default(),
            bus_interface: Data::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gpu {
    pub name: String,
    pub temperature: Data,
    pub fan_speed: Data,
    pub power: Data,
    pub load: GpuLoad,
    pub memory: GpuMemory,
    pub clock: GpuClock,
    pub nav_state: button::State,
    load_graph: StatChart
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            temperature: Data::default(),
            fan_speed: Data::default(),
            power: Data::default(),
            load: GpuLoad::default(),
            memory: GpuMemory::default(),
            clock: GpuClock::default(),
            nav_state: button::State::default(),
            load_graph: StatChart::new((255, 190, 125), Size::Small)
        }
    }

    pub fn update(mut self, data: &Value) -> Self {
        self.data_parser(data);

        self
    }

    fn data_parser(&mut self, data: &Value) {
        for child in data["Children"].as_array().unwrap()[0]["Children"].as_array().unwrap() {
            if child["ImageURL"].as_str().unwrap() == "images_icon/nvidia.png" || child["ImageURL"].as_str().unwrap() == "images_icon/amd.png" {
                for grand_child in child["Children"].as_array().unwrap() {
                    let data_type = match grand_child["Text"].as_str().unwrap() {
                        "Clocks" => DataType::Frequency,
                        "Temperatures" => DataType::Temperature,
                        "Load" => DataType::Load,
                        "Powers" => DataType::Power,
                        "Data" => DataType::Memory,
                        "Fans" => DataType::Fan,
                        _ => DataType::None,
                    };

                    for item in grand_child["Children"].as_array().unwrap() {
                        let label = item["Text"].as_str().unwrap();
                        let data = Data::from_value(&item);

                        match label {
                            "GPU Core" => match data_type {
                                DataType::Load => { self.load.core_graph.push_data(data.current); self.load_graph.push_data(data.current); self.load.core = data },
                                DataType::Temperature => self.temperature = data,
                                DataType::Frequency => { self.clock.core_graph.push_data(data.current); self.clock.core = data },
                                _ => {}
                            }
                            "GPU Memory" => match data_type {
                                DataType::Load => { self.load.memory_graph.push_data(data.current); self.load.memory = data },
                                DataType::Frequency => self.clock.memory = data,
                                _ => {}
                            }
                            "GPU" => match data_type {
                                DataType::Fan => self.fan_speed = data,
                                _ => {}
                            }
                            "GPU Frame Buffer" => self.load.frame_buffer = data,
                            "GPU Video Engine" => self.load.video_engine = data,
                            "GPU Bus Interface" => self.load.bus_interface = data,
                            "GPU Power" => self.power = data,
                            "GPU Memory Used" => { self.memory.used = data },
                            "GPU Memory Total" => self.memory.total = data.current,
                            "GPU Memory Free" => self.memory.free = data,
                            "GPU Shader" => self.clock.shader = data,
                            _ => {}
                        }
                    }
                }

                self.name = child["Text"].as_str().unwrap().replace("NVIDIA NVIDIA", "NVIDIA");
                break
            }
        }
    }

    pub fn view(&mut self) -> (Element<Message>, Element<Message>) {
        let small = Button::new(&mut self.nav_state, Row::new().align_items(Align::Center)
            .push(Space::new(Length::Units(5), Length::Shrink))
            .push(
                Container::new(
                    self.load_graph.view()
                )
                    .style(style::Container::Chart((255, 190, 125)))
                    .width(Length::Units(70))
                    .height(Length::Units(60))
            )
            .push(Space::new(Length::Units(10), Length::Shrink))
            .push(
                Column::new().spacing(3)
                    .push(Text::new("GPU"))
                    .push(Text::new(&self.name).size(14))
                    .push(Text::new(format!("{:.0}%  {:.2} GHz  ({:.0}°C)", self.load.core.current, self.clock.core.current / 1000.0, self.temperature.current)).size(14))
            )
        )
            .on_press(Message::Navigate(Route::Gpu))
            .style(style::Button::ComponentSelect)
            .width(Length::Fill)
            .height(Length::Units(75));

        let large = Column::new().padding(20)
            .push(
                Row::new().align_items(Align::Center).height(Length::Units(30))
                    .push(Text::new("GPU").size(28))
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .push(Text::new(&self.name))
            )
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .push(Text::new(format!("Core Utilization (0-{}%)", self.load.core_graph.maximum_value)).size(14))
                            .push(
                                Container::new(self.load.core_graph.view())
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(style::Container::Chart((255, 190, 125)))
                            )
                    )
                    .push(Space::new(Length::Units(20), Length::Shrink))
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .push(Text::new(format!("Core Frequency (0-{}Mhz)", self.clock.core_graph.maximum_value)).size(14))
                            .push(
                                Container::new(self.clock.core_graph.view())
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(style::Container::Chart((255, 190, 125)))
                            )
                    )
                    .height(Length::FillPortion(2))
            )
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Column::new()
                    .spacing(5)
                    .width(Length::Fill)
                    .height(Length::FillPortion(1))
                    .push(Text::new(format!("Memory Utilization (0-{}%)", self.load.memory_graph.maximum_value)).size(14))
                    .push(
                        Container::new(self.load.memory_graph.view())
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(style::Container::Chart((255, 190, 125)))
                    )

            )
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Row::new()
                    .spacing(20)
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Core Utilization").size(16))
                                    .push(Text::new(&format!("{:.0}%", self.load.core.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Memory Utilization").size(16))
                                    .push(Text::new(&format!("{:.0}%", self.load.memory.current)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Frequency").size(16))
                                    .push(Text::new(&format!("{:.2} Ghz", self.clock.core.current / 1000.0)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Frequency").size(16))
                                    .push(Text::new(&format!("{:.2} Ghz", self.clock.core.maximum / 1000.0)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Temperature").size(16))
                                    .push(Text::new(&format!("{:.0}°C", self.temperature.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Temperature").size(16))
                                    .push(Text::new(&format!("{:.0}°C", self.temperature.maximum)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Power Consumption").size(16))
                                    .push(Text::new(&format!("{:.0} Watts", self.power.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Power Consumption").size(16))
                                    .push(Text::new(&format!("{:.0} Watts", self.power.maximum)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Fan Speed").size(16))
                                    .push(Text::new(&format!("{:.0} RPM", self.fan_speed.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Fan Speed").size(16))
                                    .push(Text::new(&format!("{:.0} RPM", self.fan_speed.maximum)).size(24))
                            )
                    )
            );

        (small.into(), large.into())
    }
}