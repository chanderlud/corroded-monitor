use serde_json::Value;
use iced::{Element, Row, Column, Text, button, Button, Length, Space, Container, Align, pick_list, PickList};
use crate::Data;
use crate::ui::{Message, Route, style, chart::{StatChart, Size}};

enum DataType {
    Temperature,
    Load,
    Frequency,
    Power,
    None,
}

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

#[derive(Debug, Clone)]
pub struct CpuCore {
    temperature: Vec<Data>,
    frequency: Vec<Data>,
    load: Vec<Data>,
    power: Vec<Data>,
    load_graph: StatChart,
    temperature_graph: StatChart,
    frequency_graph: StatChart,
}

impl CpuCore {
    fn default() -> Self {
        Self {
            temperature: vec![],
            frequency: vec![],
            load: vec![],
            power: vec![],
            load_graph: StatChart::new((0, 255, 255), Size::Large),
            temperature_graph: StatChart::new((183, 53, 90), Size::Large),
            frequency_graph: StatChart::new((255, 190, 125), Size::Large),
        }
    }
}

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
    pub nav_state: button::State,
    pick_state: pick_list::State<GraphState>,
    pub graph_state: Option<GraphState>,
    load_graph: StatChart
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            cores: vec![],
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
            nav_state: button::State::default(),
            pick_state: Default::default(),
            graph_state: Some(GraphState::Utilization),
            load_graph: StatChart::new((0, 255, 255), Size::Small)
        }
    }

    pub fn update(mut self, data: &Value) -> Self {
        self.data_parser(data);

        self.calculate_totals();
        self.calculate_maximums();
        self.calculate_averages();

        self
    }

    fn data_parser(&mut self, data: &Value) {
        for child in data["Children"].as_array().unwrap()[0]["Children"].as_array().unwrap() {
            match child["ImageURL"].as_str().unwrap() {
                "images_icon/cpu.png" => {
                    for grand_child in child["Children"].as_array().unwrap() {
                        let data_type = match grand_child["Text"].as_str().unwrap() {
                            "Clocks" => DataType::Frequency,
                            "Temperatures" => DataType::Temperature,
                            "Load" => DataType::Load,
                            "Powers" => DataType::Power,
                            _ => DataType::None,
                        };

                        for core in grand_child["Children"].as_array().unwrap() {
                            let label = core["Text"].as_str().unwrap();

                            if label.contains("CPU Core") && !label.contains("CPU Cores") {
                                let index = label.split("#").collect::<Vec<&str>>()[1].parse::<usize>().unwrap() - 1;

                                match self.cores.get(index) {
                                    Some(_) => {}
                                    None => { self.cores.push(CpuCore::default()) }
                                }

                                let d = Data::from_value(core);

                                match data_type {
                                    DataType::Temperature => { self.cores[index].temperature_graph.push_data(d.current);  self.cores[index].temperature.push(d); },
                                    DataType::Frequency => { self.cores[index].frequency_graph.push_data(d.current); self.cores[index].frequency.push(d) },
                                    DataType::Load => { self.cores[index].load_graph.push_data(d.current); self.cores[index].load.push(d) },
                                    DataType::Power => self.cores[index].power.push(d),
                                    _ => {}
                                }
                            } else if label == "CPU Cores" {
                                let d = Data::from_value(core);

                                for core in self.cores.iter_mut() {
                                    core.power.push(d.clone())
                                }
                            }
                        }
                    }

                    self.name = child["Text"].as_str().unwrap().to_owned();
                    break;
                }
                _ => {}
            }
        }
    }

    fn calculate_totals(&mut self) {
        let core_count = self.cores.len() as f32;

        self.total_temperature = self.cores.iter().map(|d| if d.temperature.len() > 0 { d.temperature.last().unwrap().current } else { 0.0 }).sum::<f32>() / core_count;
        self.total_power = self.cores.iter().map(|d| if d.power.len() > 0 { d.power.last().unwrap().current } else { 0.0 }).sum::<f32>() / core_count;
        self.total_frequency = self.cores.iter().map(|d| if d.frequency.len() > 0 { d.frequency.last().unwrap().current } else { 0.0 }).sum::<f32>() / core_count;
        self.total_load = self.cores.iter().map(|d| if d.load.len() > 0 { d.load.last().unwrap().current } else { 0.0 }).sum::<f32>() / core_count;

        self.load_graph.push_data(self.total_load);
    }

    fn calculate_maximums(&mut self) {
        let core_count = self.cores.len() as f32;

        self.maximum_temperature = self.cores.iter().map(|d| if d.temperature.len() > 0 { d.temperature.last().unwrap().maximum } else { 0.0 }).sum::<f32>() / core_count;
        self.maximum_power = self.cores.iter().map(|d| if d.power.len() > 0 { d.power.last().unwrap().maximum } else { 0.0 }).sum::<f32>() / core_count;
        self.maximum_frequency = self.cores.iter().map(|d| if d.frequency.len() > 0 { d.frequency.last().unwrap().maximum } else { 0.0 }).sum::<f32>() / core_count;
    }

    fn calculate_averages(&mut self) {
        let core_count = self.cores.len() as f32;

        self.average_temperature = self.cores.iter().map(|d| d.temperature.iter().map(|v| v.current).sum::<f32>() / d.temperature.len() as f32).sum::<f32>() / core_count;
        self.average_frequency = self.cores.iter().map(|d| d.frequency.iter().map(|v| v.current).sum::<f32>() / d.frequency.len() as f32).sum::<f32>() / core_count;
        self.average_power = self.cores.iter().map(|d| d.power.iter().map(|v| v.current).sum::<f32>() / d.power.len() as f32).sum::<f32>() / core_count;
        self.average_load = self.cores.iter().map(|d| d.load.iter().map(|v| v.current).sum::<f32>() / d.load.len() as f32).sum::<f32>() / core_count;
    }

    pub fn view(&mut self) -> (Element<Message>, Element<Message>) {
        let core_count = self.cores.len();
        let c = calculate_rows(core_count);

        let graphs = match self.graph_state.unwrap() {
            GraphState::Utilization => self.cores.iter_mut().map(|c| Element::new(Container::new(c.load_graph.view()).style(style::Container::Chart((0, 255, 255))).width(Length::FillPortion(1)).height(Length::FillPortion(1)))).collect::<Vec<Element<Message>>>(),
            GraphState::Temperature => self.cores.iter_mut().map(|c| Element::new(Container::new(c.temperature_graph.view()).style(style::Container::Chart((183, 53, 90))).width(Length::FillPortion(1)).height(Length::FillPortion(1)))).collect::<Vec<Element<Message>>>(),
            GraphState::Frequency => self.cores.iter_mut().map(|c| Element::new(Container::new(c.frequency_graph.view()).style(style::Container::Chart((255, 190, 125))).width(Length::FillPortion(1)).height(Length::FillPortion(1)))).collect::<Vec<Element<Message>>>()
        };

        let mut rows = Row::new().width(Length::Fill).height(Length::Fill).spacing(10);

        let mut column: Column<Message> = Column::new().spacing(10).width(Length::FillPortion(1)).height(Length::FillPortion(1));

        let mut x = 0;
        for graph in graphs {
            x += 1;
            column = column.push(graph);

            if x == c {
                rows = rows.push(column);
                column = Column::new().spacing(10).width(Length::FillPortion(1)).height(Length::FillPortion(1));
                x = 0;
            }
        }

        let small = Button::new(&mut self.nav_state, Row::new().align_items(Align::Center)
            .push(Space::new(Length::Units(5), Length::Shrink))
            .push(
                Container::new(
                    self.load_graph.view()
                )
                    .style(style::Container::Chart((0, 255, 255)))
                    .width(Length::Units(70))
                    .height(Length::Units(60))
            )
            .push(Space::new(Length::Units(10), Length::Shrink))
            .push(
                Column::new().spacing(3)
                    .push(Text::new("CPU"))
                    .push(Text::new(&self.name).size(14))
                    .push(Text::new(format!("{:.0}%  {:.2} GHz  ({:.0}°C)", self.total_load, self.total_frequency / 1000.0, self.total_temperature)).size(14))
            ),
        )
            .on_press(Message::Navigate(Route::Cpu))
            .style(style::Button::ComponentSelect)
            .width(Length::Fill)
            .height(Length::Units(75));

        let large = Column::new().padding(20)
            .push(
                Row::new().align_items(Align::Center).height(Length::Units(30))
                    .push(Text::new("CPU").size(28))
                    .push(Space::new(Length::Units(20), Length::Shrink))
                    .push(PickList::new(&mut self.pick_state, &GraphState::ALL[..], self.graph_state, Message::PickChanged).style(style::PickList::Main))
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .push(Text::new(&self.name))
            )
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Column::new().height(Length::Fill)
                    .spacing(5)
                    .width(Length::Fill)
                    .push(Text::new(
                        match self.graph_state.unwrap() {
                            GraphState::Utilization => "Utilization (0-100%)".to_string(),
                            GraphState::Frequency => "Core Frequency".to_string(),
                            GraphState::Temperature => "Temperature".to_string(),
                        }
                    ).size(14))
                    .push(rows)
            )
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Row::new().spacing(20)
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Utilization").size(16))
                                    .push(Text::new(&format!("{:.0}%", self.total_load)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Average Utilization").size(16))
                                    .push(Text::new(&format!("{:.0}%", self.average_load)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Frequency").size(16))
                                    .push(Text::new(&format!("{:.2} Ghz", self.total_frequency / 1000.0)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Frequency").size(16))
                                    .push(Text::new(&format!("{:.2} Ghz", self.maximum_frequency / 1000.0)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Average Frequency").size(16))
                                    .push(Text::new(&format!("{:.2} Ghz", self.average_frequency / 1000.0)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Temperature").size(16))
                                    .push(Text::new(&format!("{:.0}°C", self.total_temperature)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Temperature").size(16))
                                    .push(Text::new(&format!("{:.0}°C", self.maximum_temperature)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Average Temperature").size(16))
                                    .push(Text::new(&format!("{:.0}°C", self.average_temperature)).size(24))
                            )
                    )
                    .push(
                        Column::new().spacing(5)
                            .push(
                                Column::new()
                                    .push(Text::new("Power Consumption").size(16))
                                    .push(Text::new(&format!("{:.0} Watts", self.total_power)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Max Power Consumption").size(16))
                                    .push(Text::new(&format!("{:.0} Watts", self.maximum_power)).size(24))
                            )
                            .push(
                                Column::new()
                                    .push(Text::new("Average Power Consumption").size(16))
                                    .push(Text::new(&format!("{:.0} Watts", self.average_power)).size(24))
                            )
                    )
            );

        (small.into(), large.into())
    }
}

fn calculate_rows(core_count: usize) -> usize {
    let factors = (1..core_count + 1).into_iter().filter(|&x| core_count % x == 0).collect::<Vec<usize>>();
    let count = factors.len();

    if count == 0 {
        return 0 ;
    }

    if let 0 = count % 2 {
        let a = factors[count / 2];
        let b = factors[(count / 2) -1];

        if a < b {
            a
        } else {
            b
        }

    } else {
        factors[count / 2]
    }
}