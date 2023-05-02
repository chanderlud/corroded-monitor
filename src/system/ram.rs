use iced::{Alignment, Element, Length};
use iced::widget::{Button, Column, Container, Row, Space, Text};
use iced_style::theme;
use serde_json::Value;

use crate::Data;
use crate::ui::{chart::StatChart, Message, Route};
use crate::ui::style::buttons::ComponentSelect;
use crate::ui::style::containers::GraphBox;

// ram widget
#[derive(Debug, Clone)]
pub struct Ram {
    pub usage: Data,
    pub used: Data,
    pub available: Data,
    pub total: f32,
    load_graph: StatChart,
}

impl Ram {
    pub fn new() -> Self { // ram widget with default state
        Self {
            usage: Data::default(),
            used: Data::default(),
            available: Data::default(),
            total: 0.0,
            load_graph: StatChart::new((183, 53, 90)),
        }
    }

    // parse data for gpu from the OHM API
    pub fn update(&mut self, data: &Value) {
        for child in data["Children"].as_array().unwrap()[0]["Children"].as_array().unwrap() {
            match child["ImageURL"].as_str().unwrap() {
                "images_icon/ram.png" => {
                    for grand_child in child["Children"].as_array().unwrap() {
                        match grand_child["Text"].as_str().unwrap() {
                            "Load" => {
                                for item in grand_child["Children"].as_array().unwrap() {
                                    self.usage = Data::from_value(item);
                                    self.load_graph.push_data(self.usage.current);
                                }
                            }
                            "Data" => {
                                for item in grand_child["Children"].as_array().unwrap() {
                                    let data = Data::from_value(item);
                                    match item["Text"].as_str().unwrap() {
                                        "Used Memory" => self.used = data,
                                        "Available Memory" => self.available = data,
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    break;
                }
                _ => {}
            }
        }

        self.total = self.used.current + self.available.current;
    }

    // small view of the widget located in the sidebar
    pub fn view_small(&self) -> Element<Message> {
        // the entire widget is a button
        Button::new(
            Row::new()
                .align_items(Alignment::Center)
                .push(Space::new(Length::Fixed(5.0), Length::Shrink))
                .push(
                    Container::new(self.load_graph.view()) // it contains the ram load graph
                        .style(theme::Container::Custom(Box::new(GraphBox { color: (183, 53, 90) })))
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                )
                .push(Space::new(Length::Fixed(10.0), Length::Shrink))
                .push(
                    Column::new() // this is the text on the right side of the graph with stats summary
                        .spacing(3)
                        .push(Text::new("RAM"))
                        .push(Text::new(format!("{:.1}/{:.1} GB  {:.0}%", self.used.current, self.total, self.usage.current)).size(14))
                )
        )
            .on_press(Message::Navigate(Route::Ram))
            .style(theme::Button::Custom(Box::new(ComponentSelect)))
            .width(Length::Fill)
            .height(Length::Fixed(75.0))
            .into()
    }

    // large view of the widget, the ram page
    pub fn view_large(&self) -> Element<Message> {
        Column::new().padding(20)
            .push( // the top bar, no name for ram
                   Row::new()
                       .align_items(Alignment::Center)
                       .height(Length::Fixed(30.0))
                       .push(Text::new("RAM").size(28))
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Column::new() // only the single graph for ram and no variants
                    .spacing(5)
                    .width(Length::Fill)
                    .height(Length::FillPortion(1))
                    .push(Text::new("Memory Utilization").size(14))
                    .push(
                        Container::new(self.load_graph.view())
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(theme::Container::Custom(Box::new(GraphBox { color: (183, 53, 90) })))
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
                                    .push(Text::new("Utilization").size(16))
                                    .push(Text::new(format!("{:.0}%", self.usage.current)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Available").size(16))
                                    .push(Text::new(format!("{:.2} GB", self.available.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Used").size(16))
                                    .push(Text::new(format!("{:.2} GB", self.used.current)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Total").size(16))
                                    .push(Text::new(format!("{:.0} GB", self.total)).size(24))
                            )
                    )
            )
            .into()
    }
}
