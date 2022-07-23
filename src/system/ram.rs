use iced::{Alignment, Length};
use iced::pure::Element;
use iced::pure::widget::{Button, Column, Container, Row, Space, Text};
use serde_json::Value;

use crate::Data;
use crate::ui::{Message, Route, style, chart::{StatChart, Size}};

#[derive(Debug, Clone)]
pub struct Ram {
    pub usage: Data,
    pub used: Data,
    pub available: Data,
    pub total: f32,
    load_graph: StatChart
}

impl Ram {
    pub fn new() -> Self {
        Self {
            usage: Data::default(),
            used: Data::default(),
            available: Data::default(),
            total: 0.0,
            load_graph: StatChart::new((183, 53, 90), Size::Small)
        }
    }

    pub fn update(&mut self, data: &Value) {
        self.data_parser(data);
    }

    fn data_parser(&mut self, data: &Value) {
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
                    break
                },
                _ => {}
            }
        }

        self.total = self.used.current + self.available.current;
    }

    pub fn view(&self) -> (Element<Message>, Element<Message>) {
        let small = Button::new(Row::new().align_items(Alignment::Center)
            .push(Space::new(Length::Units(5), Length::Shrink))
            .push(
                Container::new(
                    self.load_graph.view()
                )
                    .style(style::Container::Chart((183, 53, 90)))
                    .width(Length::Units(70))
                    .height(Length::Units(60))
            )
            .push(Space::new(Length::Units(10), Length::Shrink))
            .push(
                Column::new().spacing(3)
                    .push(Text::new("RAM"))
                    .push(Text::new(format!("{:.1}/{:.1} GB  {:.0}%", self.used.current, self.total, self.usage.current)).size(14))
            )
        )
            .on_press(Message::Navigate(Route::Ram))
            .style(style::Button::ComponentSelect)
            .width(Length::Fill)
            .height(Length::Units(75));

        let large = Column::new().padding(20)
            .push(
                Row::new().align_items(Alignment::Center).height(Length::Units(30))
                    .push(Text::new("RAM").size(28))
            )
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Column::new()
                    .spacing(5)
                    .width(Length::Fill)
                    .height(Length::FillPortion(1))
                    .push(Text::new("Memory Utilization").size(14))
                    .push(
                        Container::new(Text::new("aaa"))
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
                                    .push(Text::new("Utilization").size(16))
                                    .push(Text::new(&format!("{:.0}%", self.usage.current)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Available").size(16))
                                    .push(Text::new(&format!("{:.2} GB", self.available.current)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Used").size(16))
                                    .push(Text::new(&format!("{:.2} GB", self.used.current)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Total").size(16))
                                    .push(Text::new(&format!("{:.0} GB", self.total)).size(24))
                            )
                    )
            );

        (small.into(), large.into())
    }
}
