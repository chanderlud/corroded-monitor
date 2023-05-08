use std::rc::Rc;

use iced::{Alignment, Element, Length};
use iced::widget::{Button, Column, Container, PickList, Row, Space, Text};
use iced_style::theme;

use crate::system::{Data, Hardware};
use crate::ui::{Message, Route};
use crate::ui::chart::LineGraph;
use crate::ui::style::button::ComponentSelect;
use crate::ui::style::container::GraphBox;
use crate::ui::style::pick_list::PickList as PickListStyle;

// possible graph types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphState {
    Temperature,
    Activity,
}

impl GraphState {
    pub const ALL: [Self; 2] = [
        Self::Activity,
        Self::Temperature
    ];
}

// the text for the pick list
impl std::fmt::Display for GraphState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   Self::Activity => "Activity",
                   Self::Temperature => "Temperature",
               }
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Storage {
    name: String,
    index: usize,
    read_rate: Data,
    read_graph: LineGraph,
    write_rate: Data,
    write_graph: LineGraph,
    temperature: Data,
    temperature_graph: LineGraph,
    used_capacity: Data,
    activity: Data,
    activity_graph: LineGraph,
    data_read: Data,
    data_written: Data,
    pub(crate) graph_state: GraphState,
}

impl Storage {
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            index: 0,
            read_rate: Data::default(),
            read_graph: LineGraph::new((119, 221, 119)),
            write_rate: Data::default(),
            write_graph: LineGraph::new((119, 221, 119)),
            temperature: Data::default(),
            temperature_graph: LineGraph::new((119, 221, 119)),
            used_capacity: Data::default(),
            activity: Data::default(),
            activity_graph: LineGraph::new((119, 221, 119)),
            data_read: Data::default(),
            data_written: Data::default(),
            graph_state: GraphState::Activity,
        }
    }

    pub(crate) fn update(&mut self, hardware_data: &Hardware, index: usize) {
        self.name = hardware_data.name.clone();
        self.index = index;

        for sensor in &hardware_data.sensors {
            let data = Data::from(sensor);

            match sensor.name.as_str() {
                "Read Rate" => {
                    self.read_graph.push_data(data.current);
                    self.read_rate = data;
                }
                "Write Rate" => {
                    self.write_graph.push_data(data.current);
                    self.write_rate = data;
                }
                "Temperature" => {
                    self.temperature_graph.push_data(data.current);
                    self.temperature = data;
                }
                "Used Space" => {
                    self.used_capacity = data;
                }
                "Total Activity" => {
                    self.activity_graph.push_data(data.current);
                    self.activity = data;
                }
                "Data Read" => {
                    self.data_read = data;
                }
                "Data Written" => {
                    self.data_written = data;
                }
                _ => {}
            }
        }
    }

    // small view of the widget located in the sidebar
    pub fn view_small(&self, celsius: bool) -> Element<Message> {
        // the entire widget is a button
        Button::new(
            Row::new()
                .align_items(Alignment::Center)
                .push(Space::new(Length::Fixed(5.0), Length::Shrink))
                .push(
                    Container::new(self.activity_graph.view()) // it contains the gpu load graph
                        .style(theme::Container::Custom(Box::new(GraphBox::new((119, 221, 119)))))
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                )
                .push(Space::new(Length::Fixed(10.0), Length::Shrink))
                .push(
                    Column::new().spacing(3) // this is the text on the right side of the graph with stats summary
                        .push(Text::new(format!("Disk {}", self.index)))
                        .push(Text::new(&self.name).size(14))
                        .push(Text::new(
                            if celsius {
                                format!("{:.2}% {:.2} MB/s ({:.0}°C)", self.activity.current, (self.read_rate.current + self.write_rate.current) / 1_000_000_f32, self.temperature.current)
                            } else {
                                format!("{:.2}% {:.2} MB/s ({:.0}°F)", self.activity.current, (self.read_rate.current + self.write_rate.current) / 1_000_000_f32, self.temperature.current * 1.8 + 32.0)
                            }
                        )
                            .size(14)
                        )
                )
        )
            .on_press(Message::Navigate(Route::Storage(self.index))) // opens the gpu page when pressed
            .style(theme::Button::Custom(Box::new(ComponentSelect)))
            .width(Length::Fill)
            .height(Length::Fixed(75.0))
            .into()
    }

    // large view of the widget, the storage page
    pub(crate) fn view_large(&self, celsius: bool) -> Element<Message> {
        Column::new().padding(20)
            .push( // the top bar
                   Row::new()
                       .align_items(Alignment::Center)
                       .height(Length::Fixed(30.0))
                       .push(Text::new(format!("Disk {}", self.index)).size(28))
                       .push(Space::new(Length::Fill, Length::Shrink))
                       .push(Text::new(&self.name))
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Column::new()
                    .spacing(5)
                    .width(Length::Fill)
                    .height(Length::FillPortion(1))
                    .push(Row::new()
                        .push(
                            match self.graph_state {
                                GraphState::Activity => Text::new(format!("Activity (0-{:.2}%)", self.activity_graph.maximum_value)).size(14),
                                GraphState::Temperature => Text::new(
                                    if celsius {
                                        format!("Temperature (0-{:.0}°C)", self.temperature_graph.maximum_value)
                                    } else {
                                        format!("Temperature (0-{:.0}°F)", self.temperature_graph.maximum_value as f32 * 1.8 + 32.0)
                                    }
                                ).size(14),
                            }
                        )
                        .push(Space::new(Length::Fill, Length::Shrink))
                        .push(PickList::new(&GraphState::ALL[..], Some(self.graph_state), Message::StoragePickChanged) // the picklist for the different graph types
                            .text_size(14)
                            .width(Length::Fixed(120.0))
                            .padding(0)
                            .style(theme::PickList::Custom(Rc::new(PickListStyle), Rc::new(PickListStyle)))
                        )
                        .width(Length::Fill)
                    )
                    .push(
                        Container::new( // the actual graph
                                        match self.graph_state {
                                            GraphState::Activity => self.activity_graph.view(),
                                            GraphState::Temperature => self.temperature_graph.view(),
                                        }
                        )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(theme::Container::Custom(Box::new(GraphBox::new((119, 221, 119)))))
                    )
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Row::new()
                    .height(Length::FillPortion(1))
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .push(Text::new(format!("Read Rate (0-{:.2} MB/s)", self.read_graph.maximum_value / 1_000_000)).size(14))
                            .push(
                                Container::new(self.read_graph.view())
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(theme::Container::Custom(Box::new(GraphBox::new((119, 221, 119)))))
                            )
                    )
                    .push(Space::new(Length::Fixed(20.0), Length::Shrink))
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(Length::Fill)
                            .height(Length::FillPortion(1))
                            .push(Text::new(format!("Write Rate (0-{:.2} MB/s)", self.write_graph.maximum_value / 1_000_000)).size(14))
                            .push(
                                Container::new(self.write_graph.view())
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(theme::Container::Custom(Box::new(GraphBox::new((119, 221, 119)))))
                            )
                    )
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Row::new() // the text stats area
                    .spacing(20)
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Used Capacity").size(16))
                                    .push(Text::new(format!("{:.2}%", self.used_capacity.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Temperature").size(16))
                                    .push(Text::new(
                                        if celsius {
                                            format!("{:.0}°C", self.temperature.current)
                                        } else {
                                            format!("{:.0}°F", self.temperature.current * 1.8 + 32.0)
                                        }
                                    ).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Data Read").size(16))
                                    .push(Text::new(format!("{:.0} TB", self.data_read.current / 1_000_f32)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Data Written").size(16))
                                    .push(Text::new(format!("{:.0} TB", self.data_written.current / 1_000_f32)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Read Rate").size(16))
                                    .push(Text::new(format!("{:.2} MB/s", self.read_rate.current / 1_000_000_f32)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Write Rate").size(16))
                                    .push(Text::new(format!("{:.2} MB/s", self.write_rate.current / 1_000_000_f32)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Disk Activity").size(16))
                                    .push(Text::new(format!("{:.2}%", self.activity.current)).size(24))
                            )
                    )
            )
            .into()
    }
}
