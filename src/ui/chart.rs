use std::collections::VecDeque;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use iced::Element;
use iced::Length;
use plotters::prelude::*;
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget};

use crate::ui::Message;

#[derive(Debug, Clone)]
pub struct StatChart {
    data_points: VecDeque<(DateTime<Utc>, i32)>,
    // data
    color: (u8, u8, u8),
    // color of the line
    pub maximum_value: i32,
    // max value of the graph
    line_width: u32, // width of the line
}

impl StatChart {
    pub fn new(color: (u8, u8, u8)) -> Self {
        Self {
            data_points: vec![(Utc::now(), 0)].into(), // TODO does the x really need to be times? or can it just be a counter
            color,
            maximum_value: 100,
            line_width: 1,
        }
    }

    pub fn push_data(&mut self, value: f32) {
        let local_value = value as i32;

        self.data_points.push_front((Utc::now(), local_value));

        // limit data points to 100 seconds
        if self.data_points.len() > 100 {
            self.data_points.remove(self.data_points.len() - 1);
        }

        // store maximum value for the graph label
        if local_value > self.maximum_value {
            self.maximum_value = local_value
        }
    }

    pub fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<Message> for StatChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::prelude::*;

        let color = RGBColor(self.color.0, self.color.1, self.color.2);

        let newest_time = self
            .data_points
            .front()
            .unwrap_or(&(DateTime::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), Utc), 0)).0;

        let oldest_time = newest_time - Duration::seconds(100); // 100 seconds because 100 datapoints

        let mut chart = chart
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(oldest_time..newest_time, 0..self.maximum_value)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .disable_axes()
            .light_line_style(ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 0 })
            .draw()
            .expect("failed to draw chart mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data_points.iter().map(|x| (x.0, x.1)),
                    0,
                    &color.mix(0.03), // the partially transparent area under the line
                )
                    .border_style(
                        ShapeStyle::from(&color)
                            .stroke_width(self.line_width)
                    ),
            )
            .expect("failed to draw chart data");
    }
}
