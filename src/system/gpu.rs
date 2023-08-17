use std::rc::Rc;

use iced::widget::{
    container, horizontal_space, pick_list, text, vertical_space, Button, Column, Row, row, column,
};
use iced::{theme, Element};
use iced::{Alignment, Length};

use crate::system::{Data, Hardware, SensorType};
use crate::ui::style::button::ComponentSelect;
use crate::ui::style::container::GraphBox;
use crate::ui::style::pick_list::PickList as PickListStyle;
use crate::ui::{chart::LineGraph, Message, Route};

// possible graph types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphState {
    CoreClock,
    MemoryClock,
    // ShaderClock,
    CoreLoad,
    MemoryLoad,
    FrameBufferLoad,
    VideoEngineLoad,
    BusInterfaceLoad,
    FanSpeed,
    Temperature,
    PowerUsage,
    HotSpotTemperature,
    PCIeRx,
    PCIeTx,
}

// different graph types and the regions of the GUI they can appear in
impl GraphState {
    // graph states for the first graph
    pub const REGION_ONE: [GraphState; 5] = [
        GraphState::CoreLoad,
        GraphState::MemoryLoad,
        GraphState::FrameBufferLoad,
        GraphState::BusInterfaceLoad,
        GraphState::VideoEngineLoad,
    ];

    // graph states for the second graph
    pub const REGION_TWO: [GraphState; 4] = [
        GraphState::CoreClock,
        GraphState::MemoryClock,
        // GraphState::ShaderClock
        GraphState::PCIeRx,
        GraphState::PCIeTx,
    ];

    // graph states for the third graph
    pub const REGION_THREE: [GraphState; 4] = [
        GraphState::FanSpeed,
        GraphState::PowerUsage,
        GraphState::Temperature,
        GraphState::HotSpotTemperature,
    ];
}

// the text for the pick list
impl std::fmt::Display for GraphState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GraphState::CoreClock => "Core Frequency",
                GraphState::MemoryClock => "Memory Frequency",
                // GraphState::ShaderClock => "Shader Frequency",
                GraphState::CoreLoad => "Core Load",
                GraphState::MemoryLoad => "Memory Load",
                GraphState::BusInterfaceLoad => "Bus Interface Load",
                GraphState::VideoEngineLoad => "Video Engine Load",
                GraphState::FrameBufferLoad => "Frame Buffer Load",
                GraphState::FanSpeed => "Fan Speed",
                GraphState::Temperature => "Temperature",
                GraphState::PowerUsage => "Power Usage",
                GraphState::HotSpotTemperature => "Hot Spot",
                GraphState::PCIeRx => "PCIe Down",
                GraphState::PCIeTx => "PCIe Up",
            }
        )
    }
}

// data class
#[derive(Debug, Clone)]
pub struct GpuClock {
    pub(crate) core: Data,
    core_graph: LineGraph,
    pub(crate) memory: Data,
    memory_graph: LineGraph,
    // pub(crate) shader: Data,
    // shader_graph: StatChart,
}

impl GpuClock {
    fn default() -> Self {
        Self {
            core: Data::default(),
            core_graph: LineGraph::new((255, 190, 125)),
            memory: Data::default(),
            memory_graph: LineGraph::new((255, 190, 125)),
            // shader: Data::default(),
            // shader_graph: StatChart::new((255, 190, 125)),
        }
    }
}

// data class
#[derive(Debug, Clone)]
pub(crate) struct GpuMemory {
    pub(crate) free: Data,
    pub(crate) used: Data,
    pub(crate) total: f32,
}

impl GpuMemory {
    fn default() -> Self {
        Self {
            free: Data::default(),
            used: Data::default(),
            total: 0_f32,
        }
    }
}

// data class
#[derive(Debug, Clone)]
pub(crate) struct GpuLoad {
    pub(crate) core: Data,
    core_graph: LineGraph,
    pub(crate) memory: Data,
    memory_graph: LineGraph,
    pub(crate) frame_buffer: Data,
    frame_buffer_graph: LineGraph,
    pub(crate) video_engine: Data,
    video_engine_graph: LineGraph,
    pub(crate) bus_interface: Data,
    bus_interface_graph: LineGraph,
    pub(crate) pcie_rx: Data,
    pcie_rx_graph: LineGraph,
    pub(crate) pcie_tx: Data,
    pcie_tx_graph: LineGraph,
}

impl GpuLoad {
    fn default() -> Self {
        Self {
            core: Data::default(),
            core_graph: LineGraph::new((255, 190, 125)),
            memory: Data::default(),
            memory_graph: LineGraph::new((255, 190, 125)),
            frame_buffer: Data::default(),
            frame_buffer_graph: LineGraph::new((255, 190, 125)),
            video_engine: Data::default(),
            video_engine_graph: LineGraph::new((255, 190, 125)),
            bus_interface: Data::default(),
            bus_interface_graph: LineGraph::new((255, 190, 125)),
            pcie_rx: Data::default(),
            pcie_rx_graph: LineGraph::new((255, 190, 125)),
            pcie_tx: Data::default(),
            pcie_tx_graph: LineGraph::new((255, 190, 125)),
        }
    }
}

// GPU widget
#[derive(Debug, Clone)]
pub(crate) struct Gpu {
    pub(crate) name: String,
    index: usize,
    temperature: Data,
    temperature_graph: LineGraph,
    hotspot_temperature: Data,
    hotspot_temperature_graph: LineGraph,
    fan_speed: Data,
    fan_graph: LineGraph,
    power: Data,
    power_graph: LineGraph,
    load: GpuLoad,
    memory: GpuMemory,
    clock: GpuClock,
    load_graph: LineGraph,
    pub(crate) graph_state_1: GraphState,
    pub(crate) graph_state_2: GraphState,
    pub(crate) graph_state_3: GraphState,
}

impl Gpu {
    pub(crate) fn new() -> Self {
        // create a new GPU widget with default state
        Self {
            name: String::new(),
            index: 0,
            temperature: Data::default(),
            temperature_graph: LineGraph::new((255, 190, 125)),
            hotspot_temperature: Data::default(),
            hotspot_temperature_graph: LineGraph::new((255, 190, 125)),
            fan_speed: Data::default(),
            fan_graph: LineGraph::new((255, 190, 125)),
            power: Data::default(),
            power_graph: LineGraph::new((255, 190, 125)),
            load: GpuLoad::default(),
            memory: GpuMemory::default(),
            clock: GpuClock::default(),
            load_graph: LineGraph::new((255, 190, 125)),
            graph_state_1: GraphState::CoreLoad,
            graph_state_2: GraphState::CoreClock,
            graph_state_3: GraphState::FanSpeed,
        }
    }

    // parse data for gpu from the OHM API
    pub fn update(&mut self, hardware_data: &Hardware, index: usize) {
        self.name = hardware_data.name.clone();
        self.index = index;

        for sensor in &hardware_data.sensors {
            let data = Data::from(sensor);

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
                },
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
                },
                "GPU" => {
                    if sensor.sensor_type == SensorType::Fan {
                        self.fan_graph.push_data(data.current);
                        self.fan_speed = data
                    }
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
                "GPU Hot Spot" => {
                    self.hotspot_temperature_graph.push_data(data.current);
                    self.hotspot_temperature = data
                }
                "GPU Fan 1" => {
                    if sensor.sensor_type == SensorType::Fan {
                        self.fan_graph.push_data(data.current);
                        self.fan_speed = data
                    }
                }
                "GPU PCIe Rx" => {
                    self.load.pcie_rx_graph.push_data(data.current);
                    self.load.pcie_rx = data
                }
                "GPU PCIe Tx" => {
                    self.load.pcie_tx_graph.push_data(data.current);
                    self.load.pcie_tx = data
                }
                _ => {}
            }
        }
    }

    // small view of the widget located in the sidebar
    // TODO last line is clipped
    pub fn view_small(&self, celsius: bool) -> Element<Message> {
        // the entire widget is a button
        Button::new(
            Row::new()
                .align_items(Alignment::Center)
                .push(horizontal_space(Length::Fixed(5_f32)))
                .push(
                    container(self.load_graph.view()) // it contains the gpu load graph
                        .style(theme::Container::Custom(Box::new(GraphBox::new((255, 190, 125)))))
                        .width(Length::Fixed(70_f32))
                        .height(Length::Fixed(60_f32))
                )
                .push(horizontal_space(Length::Fixed(10_f32)))
                .push(
                    Column::new()
                        .spacing(3) // this is the text on the right side of the graph with stats summary
                        .push(text(format!("GPU {}", self.index)))
                        .push(text(&self.name).size(14))
                        .push(text(
                            if celsius {
                                format!("{:.0}%  {:.2} GHz  ({:.0}°C)", self.load.core.current, self.clock.core.current / 1000_f32, self.temperature.current)
                            } else {
                                format!("{:.0}%  {:.2} GHz  ({:.0}°F)", self.load.core.current, self.clock.core.current / 1000_f32, self.temperature.current * 1.8 + 32_f32)
                            }
                        ).size(14))
                )
        )
            .on_press(Message::Navigate(Route::Gpu(self.index))) // opens the gpu page when pressed
            .style(theme::Button::Custom(Box::new(ComponentSelect)))
            .width(Length::Fill)
            .height(Length::Fixed(75_f32))
            .into()
    }

    // large view of the widget, the gpu page
    pub fn view_large(&self, celsius: bool) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(
                // the top bar with the name of the gpu
                Row::new()
                    .align_items(Alignment::Center)
                    .height(Length::Fixed(30_f32))
                    .push(text("GPU").size(28))
                    .push(horizontal_space(Length::Fill))
                    .push(text(&self.name)),
            )
            .push(vertical_space(Length::Fixed(20_f32)))
            .push(
                Row::new() // the row with the two large graphs
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .push(
                                Row::new()
                                    .push(match self.graph_state_1 {
                                        // the label for the graph
                                        GraphState::CoreLoad => text(format!(
                                            "Core Utilization (0-{}%)",
                                            self.load.core_graph.maximum_value
                                        ))
                                        .size(14),
                                        GraphState::MemoryLoad => text(format!(
                                            "Memory Utilization (0-{}%)",
                                            self.load.memory_graph.maximum_value
                                        ))
                                        .size(14),
                                        GraphState::VideoEngineLoad => text(format!(
                                            "Video Engine Utilization (0-{}%)",
                                            self.load.video_engine_graph.maximum_value
                                        ))
                                        .size(14),
                                        GraphState::BusInterfaceLoad => text(format!(
                                            "Bus Interface Utilization (0-{}%)",
                                            self.load.bus_interface_graph.maximum_value
                                        ))
                                        .size(14),
                                        GraphState::FrameBufferLoad => text(format!(
                                            "Frame Buffer Utilization (0-{}%)",
                                            self.load.frame_buffer_graph.maximum_value
                                        ))
                                        .size(14),
                                        _ => text(""),
                                    })
                                    .push(horizontal_space(Length::Fill))
                                    .push(
                                        pick_list(
                                            &GraphState::REGION_ONE[..],
                                            Some(self.graph_state_1),
                                            Message::GpuPickChanged,
                                        ) // the picklist for the different graph types
                                        .text_size(14)
                                        .width(Length::Fixed(120_f32))
                                        .padding(0)
                                        .style(
                                            theme::PickList::Custom(
                                                Rc::new(PickListStyle),
                                                Rc::new(PickListStyle),
                                            ),
                                        ),
                                    )
                                    .width(Length::Fill),
                            )
                            .push(
                                container(
                                    // the actual graph
                                    match self.graph_state_1 {
                                        GraphState::CoreLoad => self.load.core_graph.view(),
                                        GraphState::MemoryLoad => self.load.memory_graph.view(),
                                        GraphState::FrameBufferLoad => {
                                            self.load.frame_buffer_graph.view()
                                        }
                                        GraphState::BusInterfaceLoad => {
                                            self.load.bus_interface_graph.view()
                                        }
                                        GraphState::VideoEngineLoad => {
                                            self.load.video_engine_graph.view()
                                        }
                                        _ => unreachable!(),
                                    },
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .style(theme::Container::Custom(Box::new(GraphBox::new((
                                    255, 190, 125,
                                ))))),
                            ),
                    )
                    .push(horizontal_space(Length::Fixed(20_f32)))
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .push(
                                Row::new()
                                    .push(match self.graph_state_2 {
                                        // the label for the graph
                                        GraphState::CoreClock => text(format!(
                                            "Core Frequency (0-{}Mhz)",
                                            self.clock.core_graph.maximum_value
                                        ))
                                        .size(14),
                                        GraphState::MemoryClock => text(format!(
                                            "Memory Frequency (0-{}Mhz)",
                                            self.clock.memory_graph.maximum_value
                                        ))
                                        .size(14),
                                        // GraphState::ShaderClock => text(format!("Shader Frequency (0-{}Mhz)", self.clock.memory_graph.maximum_value)).size(14),
                                        GraphState::PCIeRx => text(format!(
                                            "PCIe Down (0-{} MB/s)",
                                            self.load.pcie_rx_graph.maximum_value / 1_000_000
                                        ))
                                        .size(14),
                                        GraphState::PCIeTx => text(format!(
                                            "PCIe Up (0-{} MB/s)",
                                            self.load.pcie_tx_graph.maximum_value / 1_000_000
                                        ))
                                        .size(14),
                                        _ => unreachable!(),
                                    })
                                    .push(horizontal_space(Length::Fill))
                                    .push(
                                        pick_list(
                                            &GraphState::REGION_TWO[..],
                                            Some(self.graph_state_2),
                                            Message::GpuPickChanged,
                                        ) // the picklist for the different graph types
                                        .text_size(14)
                                        .width(Length::Fixed(120_f32))
                                        .padding(0)
                                        .style(
                                            theme::PickList::Custom(
                                                Rc::new(PickListStyle),
                                                Rc::new(PickListStyle),
                                            ),
                                        ),
                                    )
                                    .width(Length::Fill),
                            )
                            .push(
                                container(
                                    // the actual graph
                                    match self.graph_state_2 {
                                        GraphState::CoreClock => self.clock.core_graph.view(),
                                        GraphState::MemoryClock => self.clock.memory_graph.view(),
                                        // GraphState::ShaderClock => self.clock.shader_graph.view(),
                                        GraphState::PCIeRx => self.load.pcie_rx_graph.view(),
                                        GraphState::PCIeTx => self.load.pcie_tx_graph.view(),
                                        _ => unreachable!(),
                                    },
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .style(theme::Container::Custom(Box::new(GraphBox::new((
                                    255, 190, 125,
                                ))))),
                            ),
                    )
                    .height(Length::FillPortion(2)),
            )
            .push(vertical_space(Length::Fixed(20_f32)))
            .push(
                Column::new()
                    .spacing(5)
                    .width(Length::Fill)
                    .height(Length::FillPortion(1))
                    .push(
                        Row::new()
                            .push(match self.graph_state_3 {
                                // the label for the graph
                                GraphState::FanSpeed => text(format!(
                                    "Fan Speed (0-{} RPM)",
                                    self.fan_graph.maximum_value
                                ))
                                .size(14),
                                GraphState::Temperature => text(if celsius {
                                    format!(
                                        "Temperature (0-{}°C)",
                                        self.temperature_graph.maximum_value
                                    )
                                } else {
                                    format!(
                                        "Temperature (0-{}°F)",
                                        self.temperature_graph.maximum_value
                                    )
                                })
                                .size(14),
                                GraphState::HotSpotTemperature => text(if celsius {
                                    format!(
                                        "Hot Spot Temperature (0-{}°C)",
                                        self.hotspot_temperature_graph.maximum_value
                                    )
                                } else {
                                    format!(
                                        "Hot Spot Temperature (0-{}°F)",
                                        self.hotspot_temperature_graph.maximum_value
                                    )
                                })
                                .size(14),
                                GraphState::PowerUsage => text(format!(
                                    "Power Usage (0-{} Watts)",
                                    self.power_graph.maximum_value
                                ))
                                .size(14),
                                _ => unreachable!(),
                            })
                            .push(horizontal_space(Length::Fill))
                            .push(
                                pick_list(
                                    &GraphState::REGION_THREE[..],
                                    Some(self.graph_state_3),
                                    Message::GpuPickChanged,
                                ) // the picklist for the different graph types
                                .text_size(14)
                                .width(Length::Fixed(90_f32))
                                .padding(0)
                                .style(theme::PickList::Custom(
                                    Rc::new(PickListStyle),
                                    Rc::new(PickListStyle),
                                )),
                            )
                            .width(Length::Fill),
                    )
                    .push(
                        container(match self.graph_state_3 {
                            // the actual graph
                            GraphState::FanSpeed => self.fan_graph.view(),
                            GraphState::Temperature => self.temperature_graph.view(),
                            GraphState::HotSpotTemperature => self.hotspot_temperature_graph.view(),
                            GraphState::PowerUsage => self.power_graph.view(),
                            _ => unreachable!(),
                        })
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(theme::Container::Custom(Box::new(GraphBox::new((
                            255, 190, 125,
                        ))))),
                    ),
            )
            .push(vertical_space(Length::Fixed(20_f32)))
            .push(
                Row::new() // the text stats area
                    .spacing(20)
                    .push(
                        Column::new()
                            .spacing(5)
                            .push(
                                Column::new()
                                    .push(text("Core Utilization").size(16))
                                    .push(text(format!("{:.0}%", self.load.core.current)).size(24)),
                            )
                            .push(
                                Column::new()
                                    .push(text("Memory Utilization").size(16))
                                    .push(
                                        text(format!("{:.0}%", self.load.memory.current)).size(24),
                                    ),
                            ),
                    )
                    .push(
                        Column::new()
                            .spacing(5)
                            .push(
                                Column::new().push(text("Frequency").size(16)).push(
                                    text(format!("{:.2} Ghz", self.clock.core.current / 1000_f32))
                                        .size(24),
                                ),
                            )
                            .push(
                                Column::new().push(text("Max Frequency").size(16)).push(
                                    text(format!("{:.2} Ghz", self.clock.core.maximum / 1000_f32))
                                        .size(24),
                                ),
                            ),
                    )
                    .push(
                        Column::new()
                            .spacing(5)
                            .push(
                                Column::new().push(text("Temperature").size(16)).push(
                                    text(if celsius {
                                        format!("{:.0}°C", self.temperature.current)
                                    } else {
                                        format!("{:.0}°F", self.temperature.current * 1.8 + 32_f32)
                                    })
                                    .size(24),
                                ),
                            )
                            .push(
                                Column::new().push(text("Max Temperature").size(16)).push(
                                    text(if celsius {
                                        format!("{:.0}°C", self.temperature.maximum)
                                    } else {
                                        format!("{:.0}°F", self.temperature.maximum * 1.8 + 32_f32)
                                    })
                                    .size(24),
                                ),
                            ),
                    )
                    .push(
                        Column::new()
                            .spacing(5)
                            .push(
                                Column::new().push(text("Power Consumption").size(16)).push(
                                    text(format!("{:.0} Watts", self.power.current)).size(24),
                                ),
                            )
                            .push(
                                Column::new()
                                    .push(text("Max Power Consumption").size(16))
                                    .push(
                                        text(format!("{:.0} Watts", self.power.maximum)).size(24),
                                    ),
                            ),
                    )
                    .push(
                        Column::new()
                            .spacing(5)
                            .push(
                                Column::new().push(text("Fan Speed").size(16)).push(
                                    text(format!("{:.0} RPM", self.fan_speed.current)).size(24),
                                ),
                            )
                            .push(
                                Column::new().push(text("Max Fan Speed").size(16)).push(
                                    text(format!("{:.0} RPM", self.fan_speed.maximum)).size(24),
                                ),
                            ),
                    ),
            )
            .into()
    }
}
