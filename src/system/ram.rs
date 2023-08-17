use iced::widget::{button, column, container, horizontal_space, row, text, vertical_space};
use iced::{theme, Alignment, Element, Length};

use crate::system::{Data, Hardware};
use crate::ui::style::button::ComponentSelect;
use crate::ui::style::container::GraphBox;
use crate::ui::{chart::LineGraph, Message, Route};

// ram widget
#[derive(Debug, Clone)]
pub(crate) struct Ram {
    pub(crate) name: String,
    usage: Data,
    used: Data,
    available: Data,
    total: f32,
    load_graph: LineGraph,
}

impl Ram {
    pub(crate) fn new() -> Self {
        // ram widget with default state
        Self {
            name: String::new(),
            usage: Data::default(),
            used: Data::default(),
            available: Data::default(),
            total: 0_f32,
            load_graph: LineGraph::new((183, 53, 90)),
        }
    }

    // parse data for gpu from the OHM API
    pub(crate) fn update(&mut self, hardware_data: &Hardware) {
        for sensor in &hardware_data.sensors {
            let data = Data::from(sensor);

            match sensor.name.as_str() {
                "Memory Used" => self.used = data,
                "Memory Available" => self.available = data,
                "Memory" => {
                    self.usage = data;
                    self.load_graph.push_data(self.usage.current);
                }
                _ => {}
            }
        }

        self.total = self.used.current + self.available.current;
    }

    // small view of the widget located in the sidebar
    pub(crate) fn view_small(&self) -> Element<Message> {
        // the entire widget is a button
        button(
            row!(
                horizontal_space(Length::Fixed(5_f32)),
                container(self.load_graph.view()) // it contains the ram load graph
                    .style(theme::Container::Custom(Box::new(GraphBox::new((
                        183, 53, 90
                    )))))
                    .width(Length::Fixed(70_f32))
                    .height(Length::Fixed(60_f32)),
                horizontal_space(Length::Fixed(10_f32)),
                column!(
                    text("RAM"),
                    text(format!(
                        "{:.1}/{:.0} GB  {:.0}%",
                        self.used.current, self.total, self.usage.current
                    ))
                    .size(14),
                )
                .spacing(3)
            )
            .align_items(Alignment::Center),
        )
        .on_press(Message::Navigate(Route::Ram))
        .style(theme::Button::Custom(Box::new(ComponentSelect)))
        .width(Length::Fill)
        .height(Length::Fixed(75_f32))
        .into()
    }

    // large view of the widget, the ram page
    pub(crate) fn view_large(&self) -> Element<Message> {
        column!(
            // title bar
            row!(text("RAM").size(28))
                .align_items(Alignment::Center)
                .height(Length::Fixed(30_f32)),
            vertical_space(Length::Fixed(20_f32)),
            // ram load graph
            column!(
                text("Memory Utilization").size(14),
                container(self.load_graph.view())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(theme::Container::Custom(Box::new(GraphBox::new((
                        183, 53, 90,
                    ))))),
            )
            .spacing(5)
            .width(Length::Fill)
            .height(Length::FillPortion(1)),
            vertical_space(Length::Fixed(20_f32)),
            // text based stats
            row!(
                column!(
                    text("Utilization").size(16),
                    text(format!("{:.0}%", self.usage.current)).size(24),
                ),
                column!(
                    text("Available").size(16),
                    text(format!("{:.2} GB", self.available.current)).size(24),
                ),
                column!(
                    text("Used").size(16),
                    text(format!("{:.2} GB", self.used.current)).size(24),
                ),
                column!(
                    text("Total").size(16),
                    text(format!("{:.0} GB", self.total)).size(24),
                ),
            )
            .spacing(20)
        )
        .padding(20)
        .into()
    }
}
