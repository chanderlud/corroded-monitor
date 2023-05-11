use std::collections::VecDeque;

use iced::Element;
use iced::Length;
use plotters::prelude::*;
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget};

use crate::ui::Message;

#[derive(Debug, Clone)]
pub(crate) struct LineGraph {
    // data
    data_points: VecDeque<(i32, i32)>,
    // color of the line
    color: (u8, u8, u8),
    // max value of the graph
    pub(crate) maximum_value: i32,
    // iterator for the x axis
    iterator: i32,
}

impl LineGraph {
    pub(crate) fn new(color: (u8, u8, u8)) -> Self {
        Self {
            data_points: VecDeque::with_capacity(102),
            color,
            maximum_value: 100,
            iterator: 0,
        }
    }

    // push data point to graph
    pub(crate) fn push_data(&mut self, value: f32) {
        // fill with zeros until we have 100 data points
        while self.data_points.len() < 101 {
            self.data_points.push_front((self.iterator, 0));
            self.iterator += 1;
        }

        let local_value = value as i32;

        self.data_points.push_front((self.iterator, local_value));
        self.iterator += 1;

        // limit data points
        if self.data_points.len() > 101 {
            self.data_points.pop_back();
        }

        // store maximum value for the graph label
        if local_value > self.maximum_value {
            self.maximum_value = local_value
        }
    }

    pub(crate) fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<Message> for LineGraph {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::prelude::*;

        let color = RGBColor(self.color.0, self.color.1, self.color.2);

        let newest = self.data_points.front().unwrap_or(&(0, 0)).0;
        let oldest = self.data_points.back().unwrap_or(&(0, 0)).0;

        let mut chart = chart
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(oldest..newest, 0..self.maximum_value)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .disable_axes()
            .light_line_style(ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 0 })
            .draw().unwrap();

        chart
            .draw_series(
                AreaSeries::new(
                    self.data_points.iter().map(|x| (x.0, x.1)),
                    0,
                    &color.mix(0.03), // the partially transparent area under the line
                )
                    .border_style(
                        ShapeStyle::from(&color)
                            .stroke_width(1)
                    ),
            )
            .expect("failed to draw chart data");
    }
}
