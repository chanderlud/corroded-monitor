use iced::widget::{button, column, container, horizontal_space, row, text, vertical_space};
use iced::{theme, Alignment, Element, Length};

use crate::system::{Data, Hardware};
use crate::ui::chart::LineGraph;
use crate::ui::style::button::ComponentSelect;
use crate::ui::style::container::GraphBox;
use crate::ui::{Message, Route};

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
        button(
            row!(
                horizontal_space(Length::Fixed(5_f32)),
                container(self.throughput_graph.view()) // it contains the gpu load graph
                    .style(theme::Container::Custom(Box::new(GraphBox::new((
                        195, 177, 225
                    )))))
                    .width(Length::Fixed(70_f32))
                    .height(Length::Fixed(60_f32)),
                horizontal_space(Length::Fixed(10_f32)),
                column!(
                    text(format!("Network {}", self.index)),
                    text(&self.name).size(14),
                    text(format!(
                        "{:.0}% {:.0} MB/s",
                        self.utilization.current,
                        (self.upload_speed.current + self.download_speed.current) / 1_000_000_f32
                    ))
                    .size(14),
                )
                .spacing(2)
            )
            .align_items(Alignment::Center),
        )
        .on_press(Message::Navigate(Route::Network(self.index))) // opens the gpu page when pressed
        .style(theme::Button::Custom(Box::new(ComponentSelect)))
        .width(Length::Fill)
        .height(Length::Fixed(75_f32))
        .into()
    }

    // large view of the widget, the network page
    pub(crate) fn view_large(&self) -> Element<Message> {
        column!(
            // the title bar
            row!(
                text(format!("Network {}", self.index)).size(28),
                horizontal_space(Length::Fill),
                text(&self.name)
            )
            .align_items(Alignment::Center)
            .height(Length::Fixed(30_f32)),
            vertical_space(Length::Fixed(20_f32)),
            // the graph
            column!(
                row!(
                    text(format!(
                        "Throughput (0-{:.0} MB/s)",
                        self.throughput_graph.maximum_value / 1_000_000
                    ))
                    .size(14),
                    horizontal_space(Length::Fill),
                )
                .width(Length::Fill),
                container(self.throughput_graph.view())
                    .width(Length::Fill)
                    .height(Length::FillPortion(1))
                    .style(theme::Container::Custom(Box::new(GraphBox::new((
                        195, 177, 225,
                    ))))),
            )
            .spacing(5)
            .width(Length::Fill)
            .height(Length::FillPortion(1)),
            horizontal_space(Length::Fixed(20_f32)),
            // text based data
            row!(
                column!(
                    text("Downloaded").size(16),
                    text(format!("{:.1} GB", self.downloaded.current)).size(24),
                ),
                column!(
                    text("Uploaded").size(16),
                    text(format!("{:.1} GB", self.uploaded.current)).size(24),
                ),
                column!(
                    text("Download Speed").size(16),
                    text(format!(
                        "{:.2} MB/s",
                        self.download_speed.current / 1_000_000_f32
                    ))
                    .size(24),
                ),
                column!(
                    text("Upload Speed").size(16),
                    text(format!(
                        "{:.2} MB/s",
                        self.upload_speed.current / 1_000_000_f32
                    ))
                    .size(24),
                ),
                column!(
                    text("Utilization").size(16),
                    text(format!("{:.1}%", self.utilization.current)).size(24),
                )
            )
            .spacing(20)
        )
        .padding(20)
        .into()
    }
}
