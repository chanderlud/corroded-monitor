use std::rc::Rc;

use iced::{Alignment, Length};
use iced::Element;
use iced::widget::{Button, Column, Container, PickList, Row, Space, Text};
use iced_style::theme;

use crate::system::{Data, Hardware, HardwareType, SensorType};
use crate::ui::{chart::StatChart, Message, Route};
use crate::ui::style::buttons::ComponentSelect;
use crate::ui::style::containers::GraphBox;
use crate::ui::style::pick_list::PickList as PickListStyle;

// possible graph types
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
    PowerUsage,
}

impl GraphState {
    // graph states for the first graph
    pub const REGION_ONE: [GraphState; 5] = [
        GraphState::CoreLoad,
        GraphState::MemoryLoad,
        GraphState::FrameBufferLoad,
        GraphState::BusInterfaceLoad,
        GraphState::VideoEngineLoad
    ];

    // graph states for the second graph
    pub const REGION_TWO: [GraphState; 3] = [
        GraphState::CoreClock,
        GraphState::MemoryClock,
        GraphState::ShaderClock
    ];

    // graph states for the third graph
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

// data class
#[derive(Debug, Clone)]
pub struct GpuClock {
    pub(crate) core: Data,
    core_graph: StatChart,
    pub(crate) memory: Data,
    memory_graph: StatChart,
    pub(crate) shader: Data,
    shader_graph: StatChart,
}

impl GpuClock {
    fn default() -> Self {
        Self {
            core: Data::default(),
            core_graph: StatChart::new((255, 190, 125)),
            memory: Data::default(),
            memory_graph: StatChart::new((255, 190, 125)),
            shader: Data::default(),
            shader_graph: StatChart::new((255, 190, 125)),
        }
    }
}

// data class
#[derive(Debug, Clone)]
pub struct GpuMemory {
    pub(crate) free: Data,
    pub(crate) used: Data,
    pub(crate) total: f32,
}

impl GpuMemory {
    fn default() -> Self {
        Self {
            free: Data::default(),
            used: Data::default(),
            total: 0.0,
        }
    }
}

// data class
#[derive(Debug, Clone)]
pub struct GpuLoad {
    pub(crate) core: Data,
    core_graph: StatChart,
    pub(crate) memory: Data,
    memory_graph: StatChart,
    pub(crate) frame_buffer: Data,
    frame_buffer_graph: StatChart,
    pub(crate) video_engine: Data,
    video_engine_graph: StatChart,
    pub(crate) bus_interface: Data,
    bus_interface_graph: StatChart,
}

impl GpuLoad {
    fn default() -> Self {
        Self {
            core: Data::default(),
            core_graph: StatChart::new((255, 190, 125)),
            memory: Data::default(),
            memory_graph: StatChart::new((255, 190, 125)),
            frame_buffer: Data::default(),
            frame_buffer_graph: StatChart::new((255, 190, 125)),
            video_engine: Data::default(),
            video_engine_graph: StatChart::new((255, 190, 125)),
            bus_interface: Data::default(),
            bus_interface_graph: StatChart::new((255, 190, 125)),
        }
    }
}

// GPU widget
#[derive(Debug, Clone)]
pub struct Gpu {
    pub name: String,
    pub(crate) temperature: Data,
    temperature_graph: StatChart,
    pub(crate) fan_speed: Data,
    fan_graph: StatChart,
    pub(crate) power: Data,
    power_graph: StatChart,
    pub(crate) load: GpuLoad,
    pub(crate) memory: GpuMemory,
    pub(crate) clock: GpuClock,
    load_graph: StatChart,
    pub(crate) graph_state_1: GraphState,
    pub(crate) graph_state_2: GraphState,
    pub(crate) graph_state_3: GraphState,

}

impl Gpu {
    pub fn new() -> Self { // create a new GPU widget with default state
        Self {
            name: String::new(),
            temperature: Data::default(),
            temperature_graph: StatChart::new((255, 190, 125)),
            fan_speed: Data::default(),
            fan_graph: StatChart::new((255, 190, 125)),
            power: Data::default(),
            power_graph: StatChart::new((255, 190, 125)),
            load: GpuLoad::default(),
            memory: GpuMemory::default(),
            clock: GpuClock::default(),
            load_graph: StatChart::new((255, 190, 125)),
            graph_state_1: GraphState::CoreLoad,
            graph_state_2: GraphState::CoreClock,
            graph_state_3: GraphState::FanSpeed,
        }
    }

    // parse data for gpu from the OHM API
    pub fn update(&mut self, hardware_data: &Vec<Hardware>) {
        for hardware in hardware_data {
            match hardware.hardware_type {
                HardwareType::GpuNvidia | HardwareType::GpuAmd | HardwareType::GpuIntel => {
                    self.name = hardware.name.clone();

                    for sensor in &hardware.sensors {
                        let data = Data::from(sensor);

                        // TODO shader frequency (should work now cause admin thingy)

                        match sensor.name.as_str() {
                            "GPU Core" => match sensor.sensor_type {
                                SensorType::Temperature => {
                                    self.temperature_graph.push_data(data.current);
                                    self.temperature = data
                                }
                                SensorType::Load => {
                                    self.load.core_graph.push_data(data.current);
                                    self.load_graph.push_data(data.current);
                                    self.load.core = data
                                }
                                SensorType::Clock => {
                                    self.clock.core_graph.push_data(data.current);
                                    self.clock.core = data
                                }
                                _ => {}
                            }
                            "GPU Memory" => match sensor.sensor_type {
                                SensorType::Load => {
                                    self.load.memory_graph.push_data(data.current);
                                    self.load.memory = data
                                }
                                SensorType::Clock => {
                                    self.clock.memory_graph.push_data(data.current);
                                    self.clock.memory = data
                                }
                                _ => {}
                            }
                            "GPU" => match sensor.sensor_type {
                                SensorType::Fan => {
                                    self.fan_graph.push_data(data.current);
                                    self.fan_speed = data
                                }
                                _ => {}
                            }
                            "GPU Frame Buffer" => {
                                self.load.frame_buffer_graph.push_data(data.current);
                                self.load.frame_buffer = data
                            }
                            "GPU Video Engine" => {
                                self.load.video_engine_graph.push_data(data.current);
                                self.load.video_engine = data
                            }
                            "GPU Bus" => {
                                self.load.bus_interface_graph.push_data(data.current);
                                self.load.bus_interface = data
                            }
                            "GPU Power" => {
                                self.power_graph.push_data(data.current);
                                self.power = data
                            }
                            "GPU Memory Used" => self.memory.used = data,
                            "GPU Memory Total" => self.memory.total = data.current,
                            "GPU Memory Free" => self.memory.free = data,
                            // "GPU Shader" => {
                            //     self.clock.shader_graph.push_data(data.current);
                            //     self.clock.shader = data
                            // }
                            "GPU Fan 1" => if sensor.sensor_type == SensorType::Fan {
                                self.fan_graph.push_data(data.current);
                                self.fan_speed = data
                            }
                            _ => {}
                        }
                    }

                    break; // only parse first GPU
                }
                _ => continue
            }
        }
    }

    // small view of the widget located in the sidebar
    pub fn view_small(&self) -> Element<Message> {
        // the entire widget is a button
        Button::new(
            Row::new()
                .align_items(Alignment::Center)
                .push(Space::new(Length::Fixed(5.0), Length::Shrink))
                .push(
                    Container::new(self.load_graph.view()) // it contains the gpu load graph
                        .style(theme::Container::Custom(Box::new(GraphBox { color: (255, 190, 125) })))
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                )
                .push(Space::new(Length::Fixed(10.0), Length::Shrink))
                .push(
                    Column::new().spacing(3) // this is the text on the right side of the graph with stats summary
                        .push(Text::new("GPU"))
                        .push(Text::new(&self.name).size(14))
                        .push(Text::new(format!("{:.0}%  {:.2} GHz  ({:.0}°C)", self.load.core.current, self.clock.core.current / 1000.0, self.temperature.current)).size(14))
                )
        )
            .on_press(Message::Navigate(Route::Gpu)) // opens the gpu page when pressed
            .style(theme::Button::Custom(Box::new(ComponentSelect)))
            .width(Length::Fill)
            .height(Length::Fixed(75.0))
            .into()
    }

    // large view of the widget, the gpu page
    pub fn view_large(&self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push( // the top bar with the name of the gpu
                   Row::new().align_items(Alignment::Center).height(Length::Fixed(30.0))
                       .push(Text::new("GPU").size(28))
                       .push(Space::new(Length::Fill, Length::Shrink))
                       .push(Text::new(&self.name))
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Row::new() // the row with the two large graphs
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .push(Row::new()
                                .push(match self.graph_state_1 { // the label for the graph
                                    GraphState::CoreLoad => Text::new(format!("Core Utilization (0-{}%)", self.load.core_graph.maximum_value)).size(14),
                                    GraphState::MemoryLoad => Text::new(format!("Memory Utilization (0-{}%)", self.load.memory_graph.maximum_value)).size(14),
                                    GraphState::VideoEngineLoad => Text::new(format!("Video Engine Utilization (0-{}%)", self.load.video_engine_graph.maximum_value)).size(14),
                                    GraphState::BusInterfaceLoad => Text::new(format!("Bus Interface Utilization (0-{}%)", self.load.bus_interface_graph.maximum_value)).size(14),
                                    GraphState::FrameBufferLoad => Text::new(format!("Frame Buffer Utilization (0-{}%)", self.load.frame_buffer_graph.maximum_value)).size(14),
                                    _ => Text::new("")
                                })
                                .push(Space::new(Length::Fill, Length::Shrink))
                                .push(PickList::new(&GraphState::REGION_ONE[..], Some(self.graph_state_1), Message::GpuPickChanged) // the picklist for the different graph types
                                    .text_size(14)
                                    .width(Length::Fixed(120.0))
                                    .padding(0)
                                    .style(theme::PickList::Custom(Rc::new(PickListStyle), Rc::new(PickListStyle)))
                                )
                                .width(Length::Fill)
                            )
                            .push(
                                Container::new( // the actual graph
                                                match self.graph_state_1 {
                                                    GraphState::CoreLoad => self.load.core_graph.view(),
                                                    GraphState::MemoryLoad => self.load.memory_graph.view(),
                                                    GraphState::FrameBufferLoad => self.load.frame_buffer_graph.view(),
                                                    GraphState::BusInterfaceLoad => self.load.bus_interface_graph.view(),
                                                    GraphState::VideoEngineLoad => self.load.video_engine_graph.view(),
                                                    _ => unreachable!()
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
                                .push(match self.graph_state_2 { // the label for the graph
                                    GraphState::CoreClock => Text::new(format!("Core Frequency (0-{}Mhz)", self.clock.core_graph.maximum_value)).size(14),
                                    GraphState::MemoryClock => Text::new(format!("Memory Frequency (0-{}Mhz)", self.clock.memory_graph.maximum_value)).size(14),
                                    GraphState::ShaderClock => Text::new(format!("Shader Frequency (0-{}Mhz)", self.clock.memory_graph.maximum_value)).size(14),
                                    _ => Text::new("")
                                })
                                .push(Space::new(Length::Fill, Length::Shrink))
                                .push(PickList::new(&GraphState::REGION_TWO[..], Some(self.graph_state_2), Message::GpuPickChanged) // the picklist for the different graph types
                                    .text_size(14)
                                    .width(Length::Fixed(120.0))
                                    .padding(0)
                                    .style(theme::PickList::Custom(Rc::new(PickListStyle), Rc::new(PickListStyle)))
                                )
                                .width(Length::Fill)
                            )
                            .push(
                                Container::new( // the actual graph
                                                match self.graph_state_2 {
                                                    GraphState::CoreClock => self.clock.core_graph.view(),
                                                    GraphState::MemoryClock => self.clock.memory_graph.view(),
                                                    GraphState::ShaderClock => self.clock.shader_graph.view(),
                                                    _ => unreachable!()
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
                        .push(
                            match self.graph_state_3 { // the label for the graph
                                GraphState::FanSpeed => Text::new(format!("Fan Speed (0-{} RPM)", self.fan_graph.maximum_value)).size(14),
                                GraphState::Temperature => Text::new(format!("Temperature (0-{}°C)", self.temperature_graph.maximum_value)).size(14),
                                GraphState::PowerUsage => Text::new(format!("Power Usage (0-{} Watts)", self.power_graph.maximum_value)).size(14),
                                _ => unreachable!()
                            })
                        .push(Space::new(Length::Fill, Length::Shrink))
                        .push(PickList::new(&GraphState::REGION_THREE[..], Some(self.graph_state_3), Message::GpuPickChanged) // the picklist for the different graph types
                            .text_size(14)
                            .width(Length::Fixed(90.0))
                            .padding(0)
                            .style(theme::PickList::Custom(Rc::new(PickListStyle), Rc::new(PickListStyle)))
                        )
                        .width(Length::Fill)
                    )
                    .push(
                        Container::new(
                            match self.graph_state_3 { // the actual graph
                                GraphState::FanSpeed => self.fan_graph.view(),
                                GraphState::Temperature => self.temperature_graph.view(),
                                GraphState::PowerUsage => self.power_graph.view(),
                                _ => unreachable!()
                            })
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(theme::Container::Custom(Box::new(GraphBox { color: (255, 190, 125) })))
                    )
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Row::new() // the text stats area
                    .spacing(20)
                    .push(
                        Column::new()
                            .spacing(5)
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
                        Column::new()
                            .spacing(5)
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
                        Column::new()
                            .spacing(5)
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
                        Column::new()
                            .spacing(5)
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
                        Column::new()
                            .spacing(5)
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
            )
            .into()
    }
}
