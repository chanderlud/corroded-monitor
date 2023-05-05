use std::rc::Rc;

use iced::{Alignment, Length};
use iced::Element;
use iced::widget::{Button, Column, Container, PickList, Row, Space, Text};
use iced_style::theme;
use regex::Regex;

use crate::system::{Data, Hardware, HardwareType, SensorType};
use crate::ui::{chart::StatChart, Message, Route};
use crate::ui::style::buttons::ComponentSelect;
use crate::ui::style::containers::GraphBox;
use crate::ui::style::pick_list::PickList as PickListStyle;

// possible states for the cpu graphs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphState {
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

impl std::fmt::Display for GraphState {
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
pub struct CpuThread {
    temperature: Vec<Data>,
    frequency: Vec<Data>,
    load: Vec<Data>,
    load_graph: StatChart,
    temperature_graph: StatChart,
    frequency_graph: StatChart,
}

impl CpuThread {
    fn new() -> Self {
        Self {
            temperature: Vec::new(),
            frequency: Vec::new(),
            load: Vec::new(),
            load_graph: StatChart::new((0, 255, 255)),
            temperature_graph: StatChart::new((183, 53, 90)),
            frequency_graph: StatChart::new((255, 190, 125)),
        }
    }
}

// data for a single cpu core, can contain multiple threads
#[derive(Debug, Clone)]
pub struct CpuCore {
    pub(crate) threads: Vec<CpuThread>,
    thread_count: usize,
}

impl CpuCore {
    fn new() -> Self {
        Self {
            threads: Vec::new(),
            thread_count: 0,
        }
    }
}

impl CpuCore {
    fn add_thread(&mut self) {
        self.threads.push(CpuThread::new());
        self.thread_count += 1;
    }
}

// TODO this may leak memory
// the cpu widget
#[derive(Debug, Clone)]
pub struct Cpu {
    pub name: String,
    pub cores: Vec<CpuCore>,
    pub total_temperature: f32,
    pub total_frequency: f32,
    pub total_load: f32,
    pub total_power: f32,
    pub maximum_temperature: f32,
    pub maximum_power: f32,
    pub maximum_frequency: f32,
    pub average_temperature: f32,
    pub average_frequency: f32,
    pub average_power: f32,
    pub average_load: f32,
    pub graph_state: GraphState,
    power: Vec<Data>,
    load_graph: StatChart,
    regex: Regex,
}

impl Cpu {
    // new cpu widget with default state
    pub fn new() -> Self {
        Self {
            name: String::new(),
            cores: Vec::new(),
            total_temperature: 0.0,
            total_frequency: 0.0,
            total_load: 0.0,
            total_power: 0.0,
            maximum_temperature: 0.0,
            maximum_power: 0.0,
            maximum_frequency: 0.0,
            average_temperature: 0.0,
            average_frequency: 0.0,
            average_power: 0.0,
            average_load: 0.0,
            graph_state: GraphState::Utilization,
            power: Vec::new(),
            load_graph: StatChart::new((0, 255, 255)),
            regex: Regex::new(r"CPU Core #(\d+)(?: Thread #(\d+))?").unwrap(), // regex for parsing cpu core/thread data
        }
    }

    // update cpu widget with new data
    pub fn update(&mut self, hardware_data: &Vec<Hardware>) {
        self.data_parser(hardware_data);

        self.calculate_totals();
        self.calculate_maximums();
        self.calculate_averages();
    }

    // parse data for gpu from the OHM API
    fn data_parser(&mut self, hardware_data: &Vec<Hardware>) {
        for hardware in hardware_data {
            match hardware.hardware_type {
                HardwareType::Cpu => {
                    self.name = hardware.name.clone();

                    for sensor in &hardware.sensors {
                        let data = Data::from(sensor);

                        if sensor.name == "CPU Cores" {
                            self.power.push(data);
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
                                    }

                                    if self.cores[core_index].threads.len() == thread_index {
                                        self.cores[core_index].add_thread();
                                    }

                                    let thread = &mut self.cores[core_index].threads[thread_index];

                                    thread.load_graph.push_data(data.current);
                                    thread.load.push(data);
                                }
                                SensorType::Clock => {
                                    if self.cores.len() == sensor.index - 1 {
                                        self.cores.push(CpuCore::new());
                                        self.cores[sensor.index - 1].add_thread();
                                    }

                                    // clock data is per core so assign data to all threads in the core
                                    for thread in &mut self.cores[sensor.index - 1].threads {
                                        thread.frequency_graph.push_data(data.current.clone());
                                        thread.frequency.push(data.clone());
                                    }
                                }
                                SensorType::Temperature => {
                                    if self.cores.len() == sensor.index {
                                        self.cores.push(CpuCore::new());
                                        self.cores[sensor.index].add_thread();
                                    }

                                    // temperature data is per core so assign data to all threads in the core
                                    for thread in &mut self.cores[sensor.index].threads {
                                        thread.temperature_graph.push_data(data.current.clone());
                                        thread.temperature.push(data.clone());
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    break; // only parse the first cpu
                }
                _ => continue
            }
        }
    }

    // calculate the total stats for all cores
    fn calculate_totals(&mut self) {
        self.total_temperature = self.calculate_total_metric(|d| &d.temperature);
        self.total_power = self.power.last().unwrap().current;
        self.total_frequency = self.calculate_total_metric(|d| &d.frequency);
        self.total_load = self.calculate_total_metric(|d| &d.load);

        self.load_graph.push_data(self.total_load); // total load graph
    }

    // average maximum temperature across all cores
    fn calculate_maximums(&mut self) {
        self.maximum_temperature = self.calculate_maximum_metric(|d| &d.temperature);
        self.maximum_power = self.power.last().unwrap().maximum;
        self.maximum_frequency = self.calculate_maximum_metric(|d| &d.frequency);
    }

    // average temperature across all cores
    fn calculate_averages(&mut self) {
        self.average_temperature = self.calculate_average_metric(|d| &d.temperature);
        self.average_frequency = self.calculate_average_metric(|d| &d.frequency);
        self.average_power = self.power.iter().map(|d| d.current).sum::<f32>() / self.power.len() as f32;
        self.average_load = self.calculate_average_metric(|d| &d.load);
    }

    fn calculate_total_metric<F>(&self, metric_selector: F) -> f32
        where
            F: Fn(&CpuThread) -> &Vec<Data>,
    {
        let core_count = self.cores.len() as f32;

        self.cores
            .iter()
            .map(|core| &core.threads)
            .flatten()
            .map(|thread| {
                let metric = metric_selector(thread);

                if !metric.is_empty() {
                    metric.last().unwrap().current
                } else {
                    0.0
                }
            })
            .sum::<f32>()
            / core_count
    }

    fn calculate_maximum_metric<F>(&self, metric_selector: F) -> f32
        where
            F: Fn(&CpuThread) -> &Vec<Data>,
    {
        let core_count = self.cores.len() as f32;

        self.cores
            .iter()
            .map(|core| &core.threads)
            .flatten()
            .map(|thread| {
                let metric = metric_selector(thread);

                if !metric.is_empty() {
                    metric.last().unwrap().maximum
                } else {
                    0.0
                }
            })
            .sum::<f32>()
            / core_count
    }

    fn calculate_average_metric<F>(&self, metric_selector: F) -> f32
        where
            F: Fn(&CpuThread) -> &Vec<Data>,
    {
        let core_count = self.cores.len() as f32;

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
            / core_count
    }

    pub fn view_small(&self) -> Element<Message> {
        Button::new(
            Row::new()
                .align_items(Alignment::Center)
                .push(Space::new(Length::Fixed(5.0), Length::Shrink))
                .push(
                    Container::new(
                        self.load_graph.view()
                    )
                        .style(theme::Container::Custom(Box::new(GraphBox { color: (0, 255, 255) })))
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                )
                .push(Space::new(Length::Fixed(10.0), Length::Shrink))
                .push(
                    Column::new().spacing(3)
                        .push(Text::new("CPU"))
                        .push(Text::new(&self.name).size(14))
                        .push(Text::new(format!("{:.0}%  {:.2} GHz  ({:.0}°C)", self.total_load, self.total_frequency / 1000.0, self.total_temperature)).size(14))
                ),
        )
            .on_press(Message::Navigate(Route::Cpu))
            .style(theme::Button::Custom(Box::new(ComponentSelect)))
            .width(Length::Fill)
            .height(Length::Fixed(75.0))
            .into()
    }

    pub fn view_large(&self) -> Element<Message> {
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
            .push(
                Row::new() // text stats
                    .spacing(20)
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
                                    .push(Text::new(format!("{:.0}°C", self.total_temperature)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Temperature").size(16))
                                    .push(Text::new(format!("{:.0}°C", self.maximum_temperature)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Average Temperature").size(16))
                                    .push(Text::new(format!("{:.0}°C", self.average_temperature)).size(24))
                            )
                    )
                    .push(
                        Column::new()
                            .spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Power Consumption").size(16))
                                    .push(Text::new(format!("{:.0} Watts", self.total_power)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Power Consumption").size(16))
                                    .push(Text::new(format!("{:.0} Watts", self.maximum_power)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Average Power Consumption").size(16))
                                    .push(Text::new(format!("{:.0} Watts", self.average_power)).size(24))
                            )
                    )
            )
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
                    .style(theme::Container::Custom(Box::new(GraphBox { color })))
                    .width(Length::FillPortion(1))
                    .height(Length::FillPortion(1)),
            )
        })
        .collect()
}
