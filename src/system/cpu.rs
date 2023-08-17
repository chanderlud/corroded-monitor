use std::collections::VecDeque;
use std::fmt::Display;
use std::rc::Rc;

use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text, vertical_space,
};
use iced::{theme, Element};
use iced::{Alignment, Length};
use regex::Regex;

use crate::system::{Data, Hardware, SensorType};
use crate::ui::style::button::ComponentSelect;
use crate::ui::style::container::GraphBox;
use crate::ui::style::pick_list::PickList as PickListStyle;
use crate::ui::{chart::LineGraph, Message, Route};

struct ChunkedVec<T> {
    vec: Vec<T>,
    chunk_size: usize,
}

impl<T> ChunkedVec<T> {
    fn new(vec: Vec<T>, chunk_size: usize) -> Self {
        ChunkedVec { vec, chunk_size }
    }
}

impl<T> Iterator for ChunkedVec<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vec.is_empty() {
            None
        } else {
            Some(
                self.vec
                    .drain(..self.chunk_size.min(self.vec.len()))
                    .collect(),
            )
        }
    }
}

// possible states for the cpu graphs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GraphState {
    Temperature,
    Utilization,
    Frequency,
    Power,
}

impl GraphState {
    pub const ALL: [Self; 4] = [
        Self::Temperature,
        Self::Utilization,
        Self::Frequency,
        Self::Power,
    ];
}

impl Display for GraphState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Temperature => "Temperature",
                Self::Utilization => "Utilization",
                Self::Frequency => "Frequency",
                Self::Power => "Power",
            }
        )
    }
}

// data for a single cpu thread
#[derive(Debug, Clone)]
struct CpuThread {
    temperature: VecDeque<Data>,
    temperature_graph: LineGraph,
    frequency: VecDeque<Data>,
    frequency_graph: LineGraph,
    load: VecDeque<Data>,
    load_graph: LineGraph,
}

impl CpuThread {
    fn new() -> Self {
        Self {
            temperature: VecDeque::with_capacity(600),
            temperature_graph: LineGraph::new((183, 53, 90)),
            frequency: VecDeque::with_capacity(600),
            frequency_graph: LineGraph::new((255, 190, 125)),
            load: VecDeque::with_capacity(600),
            load_graph: LineGraph::new((0, 255, 255)),
        }
    }

    // adds data point to thread
    fn push(&mut self, data: Data, sensor_type: SensorType) {
        let array = match sensor_type {
            SensorType::Temperature => &mut self.temperature,
            SensorType::Frequency => &mut self.frequency,
            SensorType::Load => &mut self.load,
            _ => unreachable!(),
        };

        let graph = match sensor_type {
            SensorType::Temperature => &mut self.temperature_graph,
            SensorType::Frequency => &mut self.frequency_graph,
            SensorType::Load => &mut self.load_graph,
            _ => unreachable!(),
        };

        graph.push_data(data.current);

        // limit data points to 10 minutes
        if array.len() == 600 {
            array.pop_front();
        }

        array.push_back(data);
    }
}

// data for a single cpu core, can contain multiple threads
#[derive(Debug, Clone)]
struct CpuCore {
    threads: Vec<CpuThread>,
    thread_count: usize,
}

impl CpuCore {
    fn new() -> Self {
        Self {
            threads: Vec::new(),
            thread_count: 0,
        }
    }

    fn add_thread(&mut self) {
        self.threads.push(CpuThread::new());
        self.thread_count += 1;
    }
}

// the cpu widget
#[derive(Debug, Clone)]
pub(crate) struct Cpu {
    pub(crate) name: String,
    cores: Vec<CpuCore>,
    total_temperature: f32,
    total_frequency: f32,
    total_load: f32,
    total_power: Option<f32>,
    maximum_temperature: f32,
    maximum_power: Option<f32>,
    maximum_frequency: f32,
    average_temperature: f32,
    average_frequency: f32,
    average_power: Option<f32>,
    average_load: f32,
    pub(crate) graph_state: GraphState,
    power: VecDeque<Data>,
    power_graph: LineGraph,
    load_graph: LineGraph,
    regex: Regex,
    core_count: usize,
    logical_processor_count: usize,
}

impl Cpu {
    // new cpu widget with default state
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            cores: Vec::new(),
            total_temperature: 0_f32,
            total_frequency: 0_f32,
            total_load: 0_f32,
            total_power: None,
            maximum_temperature: 0_f32,
            maximum_power: None,
            maximum_frequency: 0_f32,
            average_temperature: 0_f32,
            average_frequency: 0_f32,
            average_power: None,
            average_load: 0_f32,
            graph_state: GraphState::Utilization,
            power: VecDeque::with_capacity(600),
            power_graph: LineGraph::new((119, 221, 119)),
            load_graph: LineGraph::new((0, 255, 255)),
            regex: Regex::new(r"CPU Core #(\d+)(?: Thread #(\d+))?").unwrap(), // regex for parsing cpu core/thread data
            core_count: 0,
            logical_processor_count: 0,
        }
    }

    // update cpu widget with new data
    pub(crate) fn update(&mut self, hardware_data: &Hardware) {
        self.data_parser(hardware_data);

        self.calculate_totals();
        self.calculate_maximums();
        self.calculate_averages();
    }

    // parse data for cpu from the OHM API
    fn data_parser(&mut self, hardware_data: &Hardware) {
        self.name = hardware_data.name.clone();

        for sensor in &hardware_data.sensors {
            let data = Data::from(sensor);

            if sensor.name == "CPU Cores" {
                // limit data points to 10 minutes
                if self.power.len() == 600 {
                    self.power.pop_front();
                }

                self.power_graph.push_data(data.current);
                self.power.push_back(data);
            } else if sensor.name.starts_with("CPU Core") && !sensor.name.ends_with("TjMax") {
                match sensor.sensor_type {
                    SensorType::Load => {
                        let captures = self.regex.captures(&sensor.name).unwrap();

                        let core_index =
                            captures.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;

                        let thread_index = match captures.get(2) {
                            Some(thread) => thread.as_str().parse::<usize>().unwrap() - 1,
                            None => 0,
                        };

                        if self.cores.len() == core_index {
                            self.cores.push(CpuCore::new());
                            self.core_count += 1;
                        }

                        if self.cores[core_index].threads.len() == thread_index {
                            self.cores[core_index].add_thread();
                            self.logical_processor_count += 1;
                        }

                        let thread = &mut self.cores[core_index].threads[thread_index];

                        thread.push(data, SensorType::Load);
                    }
                    SensorType::Clock => {
                        if self.cores.len() == sensor.index - 1 {
                            self.cores.push(CpuCore::new());
                            self.cores[sensor.index - 1].add_thread();
                            self.core_count += 1;
                            self.logical_processor_count += 1;
                        }

                        // clock data is per core so assign data to all threads in the core
                        for thread in &mut self.cores[sensor.index - 1].threads {
                            thread.push(data, SensorType::Frequency);
                        }
                    }
                    SensorType::Temperature => {
                        if self.cores.len() == sensor.index {
                            self.cores.push(CpuCore::new());
                            self.cores[sensor.index].add_thread();
                            self.core_count += 1;
                            self.logical_processor_count += 1;
                        }

                        // temperature data is per core so assign data to all threads in the core
                        for thread in &mut self.cores[sensor.index].threads {
                            thread.push(data, SensorType::Temperature);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // calculate the total stats for all cores
    fn calculate_totals(&mut self) {
        self.total_temperature = self.calculate_total_metric(|d| &d.temperature);

        if !self.power.is_empty() {
            self.total_power = Some(self.power.back().unwrap().current);
        }

        self.total_frequency = self.calculate_total_metric(|d| &d.frequency);
        self.total_load = self.calculate_total_metric(|d| &d.load);

        self.load_graph.push_data(self.total_load); // total load graph
    }

    // average maximum temperature across all cores
    fn calculate_maximums(&mut self) {
        self.maximum_temperature = self.calculate_maximum_metric(|d| &d.temperature);

        if !self.power.is_empty() {
            self.maximum_power = Some(self.power.back().unwrap().maximum);
        }

        self.maximum_frequency = self.calculate_maximum_metric(|d| &d.frequency);
    }

    // average temperature across all cores
    fn calculate_averages(&mut self) {
        self.average_temperature = self.calculate_average_metric(|d| &d.temperature);
        self.average_frequency = self.calculate_average_metric(|d| &d.frequency);
        self.average_power =
            Some(self.power.iter().map(|d| d.current).sum::<f32>() / self.power.len() as f32);
        self.average_load = self.calculate_average_metric(|d| &d.load);
    }

    fn calculate_total_metric<F>(&self, metric_selector: F) -> f32
    where
        F: Fn(&CpuThread) -> &VecDeque<Data>,
    {
        self.cores
            .iter()
            .flat_map(|core| &core.threads)
            .map(|thread| {
                let metric = metric_selector(thread);

                if !metric.is_empty() {
                    metric.iter().last().unwrap().current
                } else {
                    0_f32
                }
            })
            .sum::<f32>()
            / self.logical_processor_count as f32
    }

    fn calculate_maximum_metric<F>(&self, metric_selector: F) -> f32
    where
        F: Fn(&CpuThread) -> &VecDeque<Data>,
    {
        self.cores
            .iter()
            .flat_map(|core| &core.threads)
            .map(|thread| {
                let metric = metric_selector(thread);

                if !metric.is_empty() {
                    metric.iter().last().unwrap().maximum
                } else {
                    0_f32
                }
            })
            .sum::<f32>()
            / self.logical_processor_count as f32
    }

    fn calculate_average_metric<F>(&self, metric_selector: F) -> f32
    where
        F: Fn(&CpuThread) -> &VecDeque<Data>,
    {
        self.cores
            .iter()
            .flat_map(|core| &core.threads)
            .map(|thread| {
                let metric = metric_selector(thread);
                let metric_len = metric.len() as f32;

                if metric_len > 0_f32 {
                    metric.iter().map(|v| v.current).sum::<f32>() / metric_len
                } else {
                    0_f32
                }
            })
            .sum::<f32>()
            / self.logical_processor_count as f32
    }

    // build text stats
    fn make_stats(&self, celsius: bool) -> Element<Message> {
        let mut stat_items = vec![
            column!(
                column!(
                    text("Cores").size(16),
                    text(self.core_count.to_string()).size(24),
                ),
                column!(
                    text("Logical Processors").size(16),
                    text(self.logical_processor_count.to_string()).size(24),
                ),
            )
            .spacing(5)
            .into(),
            column!(
                column!(
                    text("Utilization").size(16),
                    text(format!("{:.0}%", self.total_load)).size(24),
                ),
                column!(
                    text("Average Utilization").size(16),
                    text(format!("{:.0}%", self.average_load)).size(24),
                ),
            )
            .spacing(5)
            .into(),
            column!(
                column!(
                    text("Frequency").size(16),
                    text(format!("{:.2} Ghz", self.total_frequency / 1000_f32)).size(24),
                ),
                column!(
                    text("Max Frequency").size(16),
                    text(format!("{:.2} Ghz", self.maximum_frequency / 1000_f32)).size(24),
                ),
                column!(
                    text("Average Frequency").size(16),
                    text(format!("{:.2} Ghz", self.average_frequency / 1000_f32)).size(24),
                ),
            )
            .spacing(5)
            .into(),
            column!(
                column!(
                    text("Temperature").size(16),
                    text(if celsius {
                        format!("{:.0}°C", self.total_temperature)
                    } else {
                        format!("{:.0}°F", self.total_temperature * 1.8 + 32_f32)
                    })
                    .size(24),
                ),
                column!(
                    text("Max Temperature").size(16),
                    text(if celsius {
                        format!("{:.0}°C", self.maximum_temperature)
                    } else {
                        format!("{:.0}°F", self.maximum_temperature * 1.8 + 32_f32)
                    })
                    .size(24),
                ),
                column!(
                    text("Average Temperature").size(16),
                    text(if celsius {
                        format!("{:.0}°C", self.average_temperature)
                    } else {
                        format!("{:.0}°F", self.average_temperature * 1.8 + 32_f32)
                    })
                    .size(24),
                ),
            )
            .spacing(5)
            .into(),
        ];

        // power is an optional stat, only show if it exists
        if !self.power.is_empty() {
            stat_items.push(
                column!(
                    column!(
                        text("Power Consumption").size(16),
                        text(format!("{:.0} Watts", self.total_power.unwrap())).size(24),
                    ),
                    column!(
                        text("Max Power Consumption").size(16),
                        text(format!("{:.0} Watts", self.maximum_power.unwrap())).size(24),
                    ),
                    column!(
                        text("Average Power Consumption").size(16),
                        text(format!("{:.0} Watts", self.average_power.unwrap())).size(24),
                    ),
                )
                .spacing(5)
                .into(),
            )
        }

        row(stat_items).spacing(20).into()
    }

    // TODO last line is clipped
    pub(crate) fn view_small(&self, celsius: bool) -> Element<Message> {
        button(
            row!(
                horizontal_space(Length::Fixed(5_f32)),
                container(self.load_graph.view())
                    .style(theme::Container::Custom(Box::new(GraphBox::new((
                        0, 255, 255,
                    )))))
                    .width(Length::Fixed(70_f32))
                    .height(Length::Fixed(60_f32)),
                horizontal_space(Length::Fixed(10_f32)),
                column!(
                    text("CPU"),
                    text(&self.name).size(14),
                    text(if celsius {
                        format!(
                            "{:.0}%  {:.2} GHz  ({:.0}°C)",
                            self.total_load,
                            self.total_frequency / 1000_f32,
                            self.total_temperature
                        )
                    } else {
                        format!(
                            "{:.0}%  {:.2} GHz  ({:.0}°F)",
                            self.total_load,
                            self.total_frequency / 1000_f32,
                            self.total_temperature * 1.8 + 32_f32
                        )
                    })
                    .size(14),
                )
                .spacing(3)
            )
            .align_items(Alignment::Center),
        )
        .on_press(Message::Navigate(Route::Cpu))
        .style(theme::Button::Custom(Box::new(ComponentSelect)))
        .width(Length::Fill)
        .height(Length::Fixed(75_f32))
        .into()
    }

    pub(crate) fn view_large(&self, celsius: bool) -> Element<Message> {
        let graph = if self.graph_state == GraphState::Power {
            // single graph for power
            container(self.power_graph.view())
                .style(theme::Container::Custom(Box::new(GraphBox::new((
                    119, 221, 119,
                )))))
                .width(Length::Fill)
                .height(Length::Fill)
        } else {
            let thread_count = self
                .cores
                .iter()
                .map(|core| core.thread_count)
                .sum::<usize>();

            let row_count = calculate_rows(thread_count);

            // create the graphs
            let graphs = create_graph_elements(&self.cores, self.graph_state);

            // create the columns of graphs
            let columns: Vec<Element<Message>> = ChunkedVec::new(graphs, row_count)
                .into_iter()
                .map(|graphs| {
                    column(graphs)
                        .spacing(10)
                        .width(Length::FillPortion(1))
                        .height(Length::FillPortion(1))
                        .into()
                })
                .collect();

            container(
                row(columns)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .spacing(10),
            )
        };

        column!(
            // row for the cpu name and graph type picklist
            row!(
                text("CPU").size(28),
                horizontal_space(Length::Fixed(20_f32)),
                // picklist for graph types
                pick_list(
                    &GraphState::ALL[..],
                    Some(self.graph_state),
                    Message::CpuPickChanged,
                )
                .style(theme::PickList::Custom(
                    Rc::new(PickListStyle),
                    Rc::new(PickListStyle),
                ))
                .padding(5),
                horizontal_space(Length::Fill),
                text(&self.name), // name of cpu display
            )
            .align_items(Alignment::Center)
            .height(Length::Fixed(30_f32)),
            vertical_space(Length::Fixed(20_f32)),
            column!(
                // graph labels
                match self.graph_state {
                    GraphState::Utilization => text("Utilization (0-100%)"),
                    GraphState::Frequency => text("Core Frequency"),
                    GraphState::Temperature => text("Temperature"),
                    GraphState::Power => text(format!(
                        "Power Consumption (0-{} Watts)",
                        self.power_graph.maximum_value
                    )),
                }
                .size(14),
                graph, // the graphs
            )
            .height(Length::Fill)
            .spacing(5)
            .width(Length::Fill),
            vertical_space(Length::Fixed(20_f32)),
            self.make_stats(celsius) // build the last row with text stats
        )
        .padding(20)
        .into()
    }
}

// determine the optimal number of rows for the core graph grid
fn calculate_rows(thread_count: usize) -> usize {
    let factors = (1..thread_count + 1)
        .filter(|&x| thread_count % x == 0)
        .collect::<Vec<usize>>();

    let count = factors.len();

    if count == 0 {
        return 0;
    }

    if let 0 = count % 2 {
        let a = factors[count / 2];
        let b = factors[(count / 2) - 1];

        if a < b {
            a
        } else {
            b
        }
    } else {
        factors[count / 2]
    }
}

// create the graph elements for the cpu core graphs
fn create_graph_elements(cores: &[CpuCore], graph_state: GraphState) -> Vec<Element<Message>> {
    cores
        .iter()
        .flat_map(|core| &core.threads)
        .map(|thread| {
            let (graph, color) = match graph_state {
                GraphState::Utilization => (&thread.load_graph, (0, 255, 255)),
                GraphState::Temperature => (&thread.temperature_graph, (183, 53, 90)),
                GraphState::Frequency => (&thread.frequency_graph, (255, 190, 125)),
                _ => unreachable!(),
            };

            container(graph.view())
                .style(theme::Container::Custom(Box::new(GraphBox::new(color))))
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        })
        .collect()
}
