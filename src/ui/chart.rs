use std::collections::VecDeque;

use chrono::{DateTime, Utc};
use iced::Length;
use iced::pure::Element;
use plotters::prelude::*;
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget};

use crate::ui::Message;

pub enum Size {
    Small,
    Large
}

#[derive(Debug, Clone)]
pub struct StatChart {
    data_points: VecDeque<(DateTime<Utc>, f32)>,
    color: (u8, u8, u8),
    pub maximum_value: i32,
    line_width: u32
}

impl StatChart {
    pub fn new(color: (u8, u8, u8), size: Size) -> Self {
        Self {
            data_points: vec![(Utc::now(), 0.0)].into_iter().collect(),
            color,
            maximum_value: 100,
            line_width: match size { Size::Small => 1, Size::Large => 2 }
        }
    }

    pub fn push_data(&mut self, value: f32) {
        self.data_points.push_front((Utc::now(), value));

        if self.data_points.len() > 100 {
            self.data_points.remove(self.data_points.len() - 1);
        }

        if value > self.maximum_value as f32 {
            self.maximum_value = value as i32
        }

    }

    pub fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self.clone())
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<Message> for StatChart {
    fn build_chart<DB: DrawingBackend>(&self, mut chart: ChartBuilder<DB>) {
        use plotters::prelude::*;

        let color = RGBColor(self.color.0, self.color.1, self.color.2);

        let newest_time = self
            .data_points
            .front()
            .unwrap_or(&(DateTime::from_utc(chrono::NaiveDateTime::from_timestamp(0, 0), Utc), 0.0)).0;

        let oldest_time = newest_time - chrono::Duration::seconds(60 as i64);

        let mut chart = chart
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(oldest_time..newest_time, 0..self.maximum_value.to_owned())
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
                    self.data_points.iter().map(|x| (x.0, x.1 as i32)),
                    0,
                    &color.mix(0.15),
                )
                    .border_style(ShapeStyle::from(&color).stroke_width(self.line_width)),
            )
            .expect("failed to draw chart data");
    }
}