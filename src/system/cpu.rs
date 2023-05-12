use std::collections::VecDeque;
use std::fmt::Display;
use std::rc::Rc;

use iced::{Alignment, Length};
use iced::Element;
use iced::widget::{Button, Column, Container, PickList, Row, Space, Text};
use iced_style::theme;
use regex::Regex;

use crate::system::{Data, Hardware, SensorType};
use crate::ui::{chart::LineGraph, Message, Route};
use crate::ui::style::button::ComponentSelect;
use crate::ui::style::container::GraphBox;
use crate::ui::style::pick_list::PickList as PickListStyle;

// possible states for the cpu graphs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GraphState {
    Temperature,
    Utilization,
    Frequency,
}

impl GraphState {
    pub const ALL: [GraphState; 3] = [
        GraphState::Temperature,
        GraphState::Utilization,
        GraphState::Frequency
    ];
}

impl Display for GraphState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   GraphState::Temperature => "Temperature",
                   GraphState::Utilization => "Utilization",
                   GraphState::Frequency => "Frequency"
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
            _ => unreachable!()
        };

        let graph = match sensor_type {
            SensorType::Temperature => &mut self.temperature_graph,
            SensorType::Frequency => &mut self.frequency_graph,
            SensorType::Load => &mut self.load_graph,
            _ => unreachable!()
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
            total_temperature: 0.0,
            total_frequency: 0.0,
            total_load: 0.0,
            total_power: None,
            maximum_temperature: 0.0,
            maximum_power: None,
            maximum_frequency: 0.0,
            average_temperature: 0.0,
            average_frequency: 0.0,
            average_power: None,
            average_load: 0.0,
            graph_state: GraphState::Utilization,
            power: VecDeque::with_capacity(600),
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

                self.power.push_back(data);
            } else if sensor.name.starts_with("CPU Core") && !sensor.name.ends_with("TjMax") {
                match sensor.sensor_type {
                    SensorType::Load => {
                        let captures = self.regex.captures(&sensor.name).unwrap();

                        let core_index = captures.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;

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

        if self.power.len() > 0 {
            self.total_power = Some(self.power.back().unwrap().current);
        }

        self.total_frequency = self.calculate_total_metric(|d| &d.frequency);
        self.total_load = self.calculate_total_metric(|d| &d.load);

        self.load_graph.push_data(self.total_load); // total load graph
    }

    // average maximum temperature across all cores
    fn calculate_maximums(&mut self) {
        self.maximum_temperature = self.calculate_maximum_metric(|d| &d.temperature);

        if self.power.len() > 0 {
            self.maximum_power = Some(self.power.back().unwrap().maximum);
        }

        self.maximum_frequency = self.calculate_maximum_metric(|d| &d.frequency);
    }

    // average temperature across all cores
    fn calculate_averages(&mut self) {
        self.average_temperature = self.calculate_average_metric(|d| &d.temperature);
        self.average_frequency = self.calculate_average_metric(|d| &d.frequency);
        self.average_power = Some(self.power.iter().map(|d| d.current).sum::<f32>() / self.power.len() as f32);
        self.average_load = self.calculate_average_metric(|d| &d.load);
    }

    fn calculate_total_metric<F>(&self, metric_selector: F) -> f32
        where
            F: Fn(&CpuThread) -> &VecDeque<Data>,
    {
        self.cores
            .iter()
            .map(|core| &core.threads)
            .flatten()
            .map(|thread| {
                let metric = metric_selector(thread);

                if !metric.is_empty() {
                    metric.iter().last().unwrap().current
                } else {
                    0.0
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
            .map(|core| &core.threads)
            .flatten()
            .map(|thread| {
                let metric = metric_selector(thread);

                if !metric.is_empty() {
                    metric.iter().last().unwrap().maximum
                } else {
                    0.0
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
            .map(|core| &core.threads)
            .flatten()
            .map(|thread| {
                let metric = metric_selector(thread);
                let metric_len = metric.len() as f32;

                if metric_len > 0.0 {
                    metric.iter().map(|v| v.current).sum::<f32>() / metric_len
                } else {
                    0.0
                }
            })
            .sum::<f32>()
            / self.logical_processor_count as f32
    }

    // build text stats
    fn make_stats(&self, celsius: bool) -> Element<Message> {
        let mut stats = Row::new() // text stats
            .spacing(20)
            .push(
                Column::new()
                    .spacing(5)
                    .push(
                        Column::new()
                            .push(Text::new("Cores").size(16))
                            .push(Text::new(self.core_count.to_string()).size(24))
                    )
                    .push(
                        Column::new()
                            .push(Text::new("Logical Processors").size(16))
                            .push(Text::new(self.logical_processor_count.to_string()).size(24))
                    )
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(
                        Column::new()
                            .push(Text::new("Utilization").size(16))
                            .push(Text::new(format!("{:.0}%", self.total_load)).size(24))
                    )
                    .push(
                        Column::new()
                            .push(Text::new("Average Utilization").size(16))
                            .push(Text::new(format!("{:.0}%", self.average_load)).size(24))
                    )
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(
                        Column::new()
                            .push(Text::new("Frequency").size(16))
                            .push(Text::new(format!("{:.2} Ghz", self.total_frequency / 1000.0)).size(24))
                    )
                    .push(
                        Column::new()
                            .push(Text::new("Max Frequency").size(16))
                            .push(Text::new(format!("{:.2} Ghz", self.maximum_frequency / 1000.0)).size(24))
                    )
                    .push(
                        Column::new()
                            .push(Text::new("Average Frequency").size(16))
                            .push(Text::new(format!("{:.2} Ghz", self.average_frequency / 1000.0)).size(24))
                    )
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(
                        Column::new()
                            .push(Text::new("Temperature").size(16))
                            .push(Text::new(
                                if celsius {
                                    format!("{:.0}°C", self.total_temperature)
                                } else {
                                    format!("{:.0}°F", self.total_temperature * 1.8 + 32.0)
                                }
                            ).size(24))
                    )
                    .push(
                        Column::new()
                            .push(Text::new("Max Temperature").size(16))
                            .push(Text::new(
                                if celsius {
                                    format!("{:.0}°C", self.maximum_temperature)
                                } else {
                                    format!("{:.0}°F", self.maximum_temperature * 1.8 + 32.0)
                                }
                            ).size(24))
                    )
                    .push(
                        Column::new()
                            .push(Text::new("Average Temperature").size(16))
                            .push(Text::new(
                                if celsius {
                                    format!("{:.0}°C", self.average_temperature)
                                } else {
                                    format!("{:.0}°F", self.average_temperature * 1.8 + 32.0)
                                }
                            ).size(24))
                    )
            );

        // power is an optional stat, only show if it exists
        if self.power.len() > 0 {
            stats = stats.push(
                Column::new()
                    .spacing(5)
                    .push(
                        Column::new()
                            .push(Text::new("Power Consumption").size(16))
                            .push(Text::new(format!("{:.0} Watts", self.total_power.unwrap())).size(24))
                    )
                    .push(
                        Column::new()
                            .push(Text::new("Max Power Consumption").size(16))
                            .push(Text::new(format!("{:.0} Watts", self.maximum_power.unwrap())).size(24))
                    )
                    .push(
                        Column::new()
                            .push(Text::new("Average Power Consumption").size(16))
                            .push(Text::new(format!("{:.0} Watts", self.average_power.unwrap())).size(24))
                    )
            )
        }

        stats.into()
    }

    pub(crate) fn view_small(&self, celsius: bool) -> Element<Message> {
        Button::new(
            Row::new()
                .align_items(Alignment::Center)
                .push(Space::new(Length::Fixed(5.0), Length::Shrink))
                .push(
                    Container::new(
                        self.load_graph.view()
                    )
                        .style(theme::Container::Custom(Box::new(GraphBox::new((0, 255, 255)))))
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                )
                .push(Space::new(Length::Fixed(10.0), Length::Shrink))
                .push(
                    Column::new().spacing(3)
                        .push(Text::new("CPU"))
                        .push(Text::new(&self.name).size(14))
                        .push(Text::new(
                            if celsius {
                                format!("{:.0}%  {:.2} GHz  ({:.0}°C)", self.total_load, self.total_frequency / 1000.0, self.total_temperature)
                            } else {
                                format!("{:.0}%  {:.2} GHz  ({:.0}°F)", self.total_load, self.total_frequency / 1000.0, self.total_temperature * 1.8 + 32.0)
                            }
                        ).size(14))
                ),
        )
            .on_press(Message::Navigate(Route::Cpu))
            .style(theme::Button::Custom(Box::new(ComponentSelect)))
            .width(Length::Fill)
            .height(Length::Fixed(75.0))
            .into()
    }

    pub(crate) fn view_large(&self, celsius: bool) -> Element<Message> {
        let thread_count = self.cores.iter().map(|c| c.thread_count).sum::<usize>();
        let row_count = calculate_rows(thread_count);

        // create the graphs
        let graphs = create_graph_elements(&self.cores, self.graph_state);

        let mut row = Row::new().width(Length::Fill).height(Length::Fill).spacing(10);
        let mut column: Column<Message> = Column::new().spacing(10).width(Length::FillPortion(1)).height(Length::FillPortion(1));
        let mut items_in_column = 0;

        // places the graphs into their columns and the columns into the row
        for graph in graphs {
            column = column.push(graph);
            items_in_column += 1;

            if items_in_column == row_count {
                row = row.push(column);
                column = Column::new().spacing(10).width(Length::FillPortion(1)).height(Length::FillPortion(1));
                items_in_column = 0;
            }
        }

        Column::new().padding(20)
            .push(
                Row::new() // row for the cpu name and graph type picklist
                    .align_items(Alignment::Center)
                    .height(Length::Fixed(30.0))
                    .push(Text::new("CPU").size(28))
                    .push(Space::new(Length::Fixed(20.0), Length::Shrink))
                    .push( // picklist for graph types
                           PickList::new(&GraphState::ALL[..], Some(self.graph_state), Message::CpuPickChanged)
                               .style(
                                   theme::PickList::Custom(
                                       Rc::new(PickListStyle),
                                       Rc::new(PickListStyle),
                                   )
                               )
                               .padding(5)
                    )
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .push(Text::new(&self.name)) // name of cpu display
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(
                Column::new()
                    .height(Length::Fill)
                    .spacing(5)
                    .width(Length::Fill)
                    .push( // graph labels
                           Text::new(
                               match self.graph_state {
                                   GraphState::Utilization => "Utilization (0-100%)",
                                   GraphState::Frequency => "Core Frequency",
                                   GraphState::Temperature => "Temperature"
                               }
                           ).size(14)
                    )
                    .push(row) // the graphs
            )
            .push(Space::new(Length::Shrink, Length::Fixed(20.0)))
            .push(self.make_stats(celsius)) // build the last row with text stats
            .into()
    }
}

// determine the optimal number of rows for the core graph grid
fn calculate_rows(thread_count: usize) -> usize {
    let factors = (1..thread_count + 1).into_iter().filter(|&x| thread_count % x == 0).collect::<Vec<usize>>();
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
        .map(|core| &core.threads)
        .flatten()
        .map(|thread| {
            let (graph, color) = match graph_state {
                GraphState::Utilization => (&thread.load_graph, (0, 255, 255)),
                GraphState::Temperature => (&thread.temperature_graph, (183, 53, 90)),
                GraphState::Frequency => (&thread.frequency_graph, (255, 190, 125)),
            };

            Element::new(
                Container::new(graph.view())
                    .style(theme::Container::Custom(Box::new(GraphBox::new(color))))
                    .width(Length::FillPortion(1))
                    .height(Length::FillPortion(1)),
            )
        })
        .collect()
}
