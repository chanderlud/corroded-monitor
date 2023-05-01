use std::rc::Rc;
use iced::{Length, Alignment};
use iced::Element;
use iced::widget::{Button, Container, Column, Row, Space, Text, PickList};
use iced_style::theme;
use serde_json::Value;

use crate::Data;
use crate::ui::{chart::{StatChart, Size}, Message, Route};
use crate::ui::style::buttons::ComponentSelect;
use crate::ui::style::containers::GraphBox;
use crate::ui::style::pick_list::PickList as PickListStyle;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphState {
    CoreClock,
    MemoryClock,
    ShaderClock,
    CoreLoad,
    MemoryLoad,
    FrameBufferLoad,
    VideoEngineLoad,
    BusInterfaceLoad,
    FanSpeed,
    Temperature,
    PowerUsage
}

impl GraphState {
    pub const REGION_TWO: [GraphState; 3] = [
        GraphState::CoreClock,
        GraphState::MemoryClock,
        GraphState::ShaderClock
    ];

    pub const REGION_ONE: [GraphState; 5] = [
        GraphState::CoreLoad,
        GraphState::MemoryLoad,
        GraphState::FrameBufferLoad,
        GraphState::BusInterfaceLoad,
        GraphState::VideoEngineLoad
    ];

    pub const REGION_THREE: [GraphState; 3] = [
        GraphState::FanSpeed,
        GraphState::PowerUsage,
        GraphState::Temperature
    ];
}

impl std::fmt::Display for GraphState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   GraphState::CoreClock => "Core Frequency",
                   GraphState::MemoryClock => "Memory Frequency",
                   GraphState::ShaderClock => "Shader Frequency",
                   GraphState::CoreLoad => "Core Load",
                   GraphState::MemoryLoad => "Memory Load",
                   GraphState::BusInterfaceLoad => "Bus Interface Load",
                   GraphState::VideoEngineLoad => "Video Engine Load",
                   GraphState::FrameBufferLoad => "Frame Buffer Load",
                   GraphState::FanSpeed => "Fan Speed",
                   GraphState::Temperature => "Temperature",
                   GraphState::PowerUsage => "Power Usage"
               }
        )
    }
}

#[derive(Debug, Clone)]
pub struct GpuClock {
    pub core: Data,
    core_graph: StatChart,
    pub memory: Data,
    memory_graph: StatChart,
    pub shader: Data,
    shader_graph: StatChart
}

impl GpuClock {
    fn default() -> Self {
        Self {
            core: Data::default(),
            core_graph: StatChart::new((255, 190, 125), Size::Large),
            memory: Data::default(),
            memory_graph: StatChart::new((255, 190, 125), Size::Large),
            shader: Data::default(),
            shader_graph: StatChart::new((255, 190, 125), Size::Large),
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
    frame_buffer_graph: StatChart,
    pub video_engine: Data,
    video_engine_graph: StatChart,
    pub bus_interface: Data,
    bus_interface_graph: StatChart
}

impl GpuLoad {
    fn default() -> Self {
        Self {
            core: Data::default(),
            core_graph: StatChart::new((255, 190, 125), Size::Large),
            memory: Data::default(),
            memory_graph: StatChart::new((255, 190, 125), Size::Large),
            frame_buffer: Data::default(),
            frame_buffer_graph: StatChart::new((255, 190, 125), Size::Large),
            video_engine: Data::default(),
            video_engine_graph: StatChart::new((255, 190, 125), Size::Large),
            bus_interface: Data::default(),
            bus_interface_graph: StatChart::new((255, 190, 125), Size::Large),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gpu {
    pub name: String,
    pub temperature: Data,
    temperature_graph: StatChart,
    pub fan_speed: Data,
    fan_graph: StatChart,
    pub power: Data,
    power_graph: StatChart,
    pub load: GpuLoad,
    pub memory: GpuMemory,
    pub clock: GpuClock,
    load_graph: StatChart,
    pub graph_state_1: GraphState,
    pub graph_state_2: GraphState,
    pub graph_state_3: GraphState,

}

impl Gpu {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            temperature: Data::default(),
            temperature_graph: StatChart::new((255, 190, 125), Size::Large),
            fan_speed: Data::default(),
            fan_graph: StatChart::new((255, 190, 125), Size::Large),
            power: Data::default(),
            power_graph: StatChart::new((255, 190, 125), Size::Large),
            load: GpuLoad::default(),
            memory: GpuMemory::default(),
            clock: GpuClock::default(),
            load_graph: StatChart::new((255, 190, 125), Size::Small),
            graph_state_1: GraphState::CoreLoad,
            graph_state_2: GraphState::CoreClock,
            graph_state_3: GraphState::FanSpeed
        }
    }

    pub fn update(&mut self, data: &Value) {
        self.data_parser(data);
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
                                DataType::Load => {
                                    self.load.core_graph.push_data(data.current);
                                    self.load_graph.push_data(data.current);
                                    self.load.core = data
                                },
                                DataType::Temperature => self.temperature = data,
                                DataType::Frequency => {
                                    self.clock.core_graph.push_data(data.current);
                                    self.clock.core = data
                                },
                                _ => {}
                            }
                            "GPU Memory" => match data_type {
                                DataType::Load => {
                                    self.load.memory_graph.push_data(data.current);
                                    self.load.memory = data
                                },
                                DataType::Frequency => {
                                    self.clock.memory_graph.push_data(data.current);
                                    self.clock.memory = data
                                },
                                _ => {}
                            }
                            "GPU" => match data_type {
                                DataType::Fan => {
                                    self.fan_graph.push_data(data.current);
                                    self.fan_speed = data
                                },
                                _ => {}
                            }
                            "GPU Frame Buffer" => {
                                self.load.frame_buffer_graph.push_data(data.current);
                                self.load.frame_buffer = data
                            },
                            "GPU Video Engine" => {
                                self.load.video_engine_graph.push_data(data.current);
                                self.load.video_engine = data
                            },
                            "GPU Bus Interface" => {
                                self.load.bus_interface_graph.push_data(data.current);
                                self.load.bus_interface = data
                            },
                            "GPU Power" => self.power = data,
                            "GPU Memory Used" => { self.memory.used = data },
                            "GPU Memory Total" => self.memory.total = data.current,
                            "GPU Memory Free" => self.memory.free = data,
                            "GPU Shader" => {
                                self.clock.shader_graph.push_data(data.current);
                                self.clock.shader = data
                            },
                            _ => {}
                        }
                    }
                }

                self.name = child["Text"].as_str().unwrap().replace("NVIDIA NVIDIA", "NVIDIA");
                break
            }
        }
    }

    pub fn view(&self) -> (Element<Message>, Element<Message>) {
        let small = Button::new(Row::new().align_items(Alignment::Center)
            .push(Space::new(Length::Fixed(5.0), Length::Shrink))
            .push(
                Container::new(
                    self.load_graph.view()
                )
                    .style(theme::Container::Custom(Box::new(GraphBox { color: (255, 190, 125) })))
                    .width(Length::Fixed(70.0))
                    .height(Length::Fixed(60.0))
            )
            .push(Space::new(Length::Fixed(10.0), Length::Shrink))
            .push(
                Column::new().spacing(3)
                    .push(Text::new("GPU"))
                    .push(Text::new(&self.name).size(14))
                    .push(Text::new(format!("{:.0}%  {:.2} GHz  ({:.0}°C)", self.load.core.current, self.clock.core.current / 1000.0, self.temperature.current)).size(14))
            )
        )
            .on_press(Message::Navigate(Route::Gpu))
            .style(theme::Button::Custom(Box::new(ComponentSelect)))
            .width(Length::Fill)
            .height(Length::Fixed(75.0));

        let large = Column::new().padding(20)
            .push(
                Row::new().align_items(Alignment::Center).height(Length::Fixed(30.0))
                    .push(Text::new("GPU").size(28))
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .push(Text::new(&self.name))
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .push(Row::new()
                                .push(match self.graph_state_1 {
                                    GraphState::CoreLoad => Text::new(format!("Core Utilization (0-{}%)", self.load.core_graph.maximum_value)).size(14),
                                    GraphState::MemoryLoad => Text::new(format!("Memory Utilization (0-{}%)", self.load.memory_graph.maximum_value)).size(14),
                                    GraphState::VideoEngineLoad => Text::new(format!("Video Engine Utilization (0-{}%)", self.load.video_engine_graph.maximum_value)).size(14),
                                    GraphState::BusInterfaceLoad => Text::new(format!("Bus Interface Utilization (0-{}%)", self.load.bus_interface_graph.maximum_value)).size(14),
                                    GraphState::FrameBufferLoad => Text::new(format!("Frame Buffer Utilization (0-{}%)", self.load.frame_buffer_graph.maximum_value)).size(14),
                                    _ => Text::new("")
                                })
                                .push(Space::new(Length::Fill, Length::Shrink))
                                .push(PickList::new(&GraphState::REGION_ONE[..], Some(self.graph_state_1), Message::GpuPickChanged)
                                    .text_size(14)
                                    .width(Length::Fixed(120.0))
                                    .padding(0)
                                    .style(theme::PickList::Custom(Rc::new(PickListStyle), Rc::new(PickListStyle)))
                                )
                                .width(Length::Fill)
                            )
                            .push(
                                Container::new(match self.graph_state_1 {
                                    GraphState::CoreLoad => self.load.core_graph.view(),
                                    GraphState::MemoryLoad => self.load.memory_graph.view(),
                                    GraphState::FrameBufferLoad => self.load.frame_buffer_graph.view(),
                                    GraphState::BusInterfaceLoad => self.load.bus_interface_graph.view(),
                                    GraphState::VideoEngineLoad => self.load.video_engine_graph.view(),
                                     _ => self.load.core_graph.view()
                                })
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(theme::Container::Custom(Box::new(GraphBox { color: (255, 190, 125) })))
                            )
                    )
                    .push(Space::new(Length::Fixed(20.0), Length::Shrink))
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .push(Row::new()
                                .push(match self.graph_state_2 {
                                    GraphState::CoreClock => Text::new(format!("Core Frequency (0-{}Mhz)", self.clock.core_graph.maximum_value)).size(14),
                                    GraphState::MemoryClock => Text::new(format!("Memory Frequency (0-{}Mhz)", self.clock.memory_graph.maximum_value)).size(14),
                                    GraphState::ShaderClock => Text::new(format!("Shader Frequency (0-{}Mhz)", self.clock.memory_graph.maximum_value)).size(14),
                                    _ => Text::new("")
                                })
                                .push(Space::new(Length::Fill, Length::Shrink))
                                .push(PickList::new(&GraphState::REGION_TWO[..], Some(self.graph_state_2), Message::GpuPickChanged)
                                    .text_size(14)
                                    .width(Length::Fixed(120.0))
                                    .padding(0)
                                    .style(theme::PickList::Custom(Rc::new(PickListStyle), Rc::new(PickListStyle)))
                                )
                                .width(Length::Fill)
                            )
                            .push(
                                Container::new(match self.graph_state_2 {
                                    GraphState::CoreClock => self.clock.core_graph.view(),
                                    GraphState::MemoryClock => self.clock.memory_graph.view(),
                                    GraphState::ShaderClock => self.clock.shader_graph.view(),
                                    _ => self.clock.core_graph.view()
                                })
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(theme::Container::Custom(Box::new(GraphBox { color: (255, 190, 125) })))
                            )
                    )
                    .height(Length::FillPortion(2))
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Column::new()
                    .spacing(5)
                    .width(Length::Fill)
                    .height(Length::FillPortion(1))
                    .push(Row::new()
                        .push(match self.graph_state_3 {
                            GraphState::FanSpeed => Text::new(format!("Fan Speed (0-{} RPM)", self.fan_graph.maximum_value)).size(14),
                            GraphState::Temperature => Text::new(format!("Temperature (0-{}°C)", self.temperature_graph.maximum_value)).size(14),
                            GraphState::PowerUsage => Text::new(format!("Power Usage (0-{} Watts)", self.power_graph.maximum_value)).size(14),
                            _ => Text::new("")
                        })
                        .push(Space::new(Length::Fill, Length::Shrink))
                        .push(PickList::new(&GraphState::REGION_THREE[..], Some(self.graph_state_3), Message::GpuPickChanged)
                            .text_size(14)
                            .width(Length::Fixed(90.0))
                            .padding(0)
                            .style(theme::PickList::Custom(Rc::new(PickListStyle), Rc::new(PickListStyle)))
                        )
                        .width(Length::Fill)
                    )
                    .push(
                        Container::new(match self.graph_state_3 {
                            GraphState::FanSpeed => self.fan_graph.view(),
                            GraphState::Temperature => self.temperature_graph.view(),
                            GraphState::PowerUsage => self.power_graph.view(),
                            _ => self.fan_graph.view()
                        })
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(theme::Container::Custom(Box::new(GraphBox { color: (255, 190, 125) })))
                    )

            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Row::new()
                    .spacing(20)
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Core Utilization").size(16))
                                    .push(Text::new(format!("{:.0}%", self.load.core.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Memory Utilization").size(16))
                                    .push(Text::new(format!("{:.0}%", self.load.memory.current)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Frequency").size(16))
                                    .push(Text::new(format!("{:.2} Ghz", self.clock.core.current / 1000.0)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Frequency").size(16))
                                    .push(Text::new(format!("{:.2} Ghz", self.clock.core.maximum / 1000.0)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Temperature").size(16))
                                    .push(Text::new(format!("{:.0}°C", self.temperature.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Temperature").size(16))
                                    .push(Text::new(format!("{:.0}°C", self.temperature.maximum)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Power Consumption").size(16))
                                    .push(Text::new(format!("{:.0} Watts", self.power.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Power Consumption").size(16))
                                    .push(Text::new(format!("{:.0} Watts", self.power.maximum)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Fan Speed").size(16))
                                    .push(Text::new(format!("{:.0} RPM", self.fan_speed.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Fan Speed").size(16))
                                    .push(Text::new(format!("{:.0} RPM", self.fan_speed.maximum)).size(24))
                            )
                    )
            );

        (small.into(), large.into())
    }
}
