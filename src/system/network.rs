use iced::{Alignment, Element, Length};
use iced::widget::{Button, Column, Container, Row, Space, Text};
use iced_style::theme;
use crate::system::{Data, Hardware};
use crate::ui::{Message, Route};
use crate::ui::chart::LineGraph;
use crate::ui::style::button::ComponentSelect;
use crate::ui::style::container::GraphBox;

#[derive(Debug, Clone)]
pub(crate) struct NetworkAdapter {
    pub(crate) name: String,
    index: usize,
    utilization: Data,
    download_speed: Data,
    upload_speed: Data,
    downloaded: Data,
    uploaded: Data,
    throughput_graph: LineGraph,
}

impl NetworkAdapter {
    pub(crate) fn new() -> Self {
        NetworkAdapter {
            name: String::new(),
            index: 0,
            utilization: Data::default(),
            download_speed: Data::default(),
            upload_speed: Data::default(),
            downloaded: Data::default(),
            uploaded: Data::default(),
            throughput_graph: LineGraph::new((195, 177, 225)),
        }
    }

    pub(crate) fn update(&mut self, hardware_data: &Hardware, index: usize) {
        self.name = hardware_data.name.clone();
        self.index = index;
        let mut throughput = 0_f32; // combined upload and download speed

        for sensor in &hardware_data.sensors {
            let data = Data::from(sensor);

            match sensor.name.as_str() {
                "Network Utilization" => {
                    self.utilization = data;
                }
                "Data Uploaded" => {
                    self.uploaded = data;
                }
                "Data Downloaded" => {
                    self.downloaded = data;
                }
                "Download Speed" => {
                    throughput += data.current;
                    self.download_speed = data;
                }
                "Upload Speed" => {
                    throughput += data.current;
                    self.upload_speed = data;
                }
                _ => {}
            }
        }

        self.throughput_graph.push_data(throughput);
    }

    // small view of the widget located in the sidebar
    pub fn view_small(&self) -> Element<Message> {
        // the entire widget is a button
        Button::new(
            Row::new()
                .align_items(Alignment::Center)
                .push(Space::new(Length::Fixed(5.0), Length::Shrink))
                .push(
                    Container::new(self.throughput_graph.view()) // it contains the gpu load graph
                        .style(theme::Container::Custom(Box::new(GraphBox::new((195, 177, 225)))))
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                )
                .push(Space::new(Length::Fixed(10.0), Length::Shrink))
                .push(
                    Column::new().spacing(3) // this is the text on the right side of the graph with stats summary
                        .push(Text::new(format!("Network {}", self.index)))
                        .push(Text::new(&self.name).size(14))
                        .push(Text::new(format!("{:.0}% {:.0} MB/s", self.utilization.current, (self.upload_speed.current + self.download_speed.current) / 1_000_000_f32)).size(14))
                )
        )
            .on_press(Message::Navigate(Route::Network(self.index))) // opens the gpu page when pressed
            .style(theme::Button::Custom(Box::new(ComponentSelect)))
            .width(Length::Fill)
            .height(Length::Fixed(75.0))
            .into()
    }

    // large view of the widget, the network page
    pub(crate) fn view_large(&self) -> Element<Message> {
        Column::new().padding(20)
            .push( // the top bar
                   Row::new()
                       .align_items(Alignment::Center)
                       .height(Length::Fixed(30.0))
                       .push(Text::new(format!("Network {}", self.index)).size(28))
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
                        .push(Text::new(format!("Throughput (0-{:.0} MB/s)", self.throughput_graph.maximum_value / 1_000_000)).size(14))
                        .push(Space::new(Length::Fill, Length::Shrink))
                        .width(Length::Fill)
                    )
                    .push(
                        Container::new(self.throughput_graph.view())
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(theme::Container::Custom(Box::new(GraphBox::new((195, 177, 225)))))
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
                                    .push(Text::new("Downloaded").size(16))
                                    .push(Text::new(format!("{:.1} GB", self.downloaded.current)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Uploaded").size(16))
                                    .push(Text::new(format!("{:.1} GB", self.uploaded.current)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Download Speed").size(16))
                                    .push(Text::new(format!("{:.2} MB/s", self.download_speed.current / 1_000_000_f32)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Upload Speed").size(16))
                                    .push(Text::new(format!("{:.2} MB/s", self.upload_speed.current / 1_000_000_f32)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Utilization").size(16))
                                    .push(Text::new(format!("{:.1}%", self.utilization.current)).size(24))
                            )
                    )
            )
            .into()
    }
}
