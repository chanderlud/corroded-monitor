use std::sync::Arc;
use std::time::Duration;

use iced::{Command, executor, Length, Subscription};
use iced::{Application, Element};
use iced::time::every;
use iced::widget::{Column, Container, Row};
use iced_style::{Theme, theme};
use tokio::sync::Mutex;

use crate::{HardwareMonitor, SystemStats};
use crate::gpu::GraphState;
use crate::ui::{Message, Route};
use crate::ui::style::containers::{MainBox, SecondaryBox};

pub struct App {
    route: Route,
    stats: SystemStats,
    monitor: Option<Arc<Mutex<HardwareMonitor>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            route: Route::Cpu,
            stats: SystemStats::new(),
            monitor: None, // monitor is initialized asynchronously later
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // creating the hardware monitor takes a second so its done asynchronously
        (App::default(), Command::perform(HardwareMonitor::new(), Message::MonitorCreated))
    }

    fn title(&self) -> String {
        "Corroded Monitor".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Update => match self.monitor.clone() {
                Some(monitor) => {
                    Command::perform(
                        self.stats.clone().update(monitor),
                        Message::UpdateCompleted,
                    )
                }
                None => Command::none() // hardware monitor is not created yet
            },
            Message::UpdateCompleted(updated_stats) => {
                self.stats = updated_stats;
                Command::none()
            }
            Message::MonitorCreated(monitor) => {
                self.monitor = Some(monitor);
                Command::none()
            }
            Message::Navigate(r) => {
                self.route = r;
                Command::none()
            }
            Message::CpuPickChanged(state) => {
                self.stats.cpu.graph_state = state;
                Command::none()
            }
            Message::GpuPickChanged(state) => {
                if GraphState::REGION_ONE.contains(&state) {
                    self.stats.gpu.graph_state_1 = state;
                } else if GraphState::REGION_TWO.contains(&state) {
                    self.stats.gpu.graph_state_2 = state;
                } else {
                    self.stats.gpu.graph_state_3 = state;
                }

                Command::none()
            }
        }
    }

    // the base of the GUI
    fn view(&self) -> Element<'_, Self::Message> {
        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Container::new(
                    Column::new()
                        .push(self.stats.cpu.view_small())
                        .push(self.stats.gpu.view_small())
                        .push(self.stats.ram.view_small())
                ).style(theme::Container::Custom(Box::new(SecondaryBox))).height(Length::Fill).width(Length::Fixed(300.0))
            )
            .push(
                match self.route {
                    Route::Cpu => Container::new(self.stats.cpu.view_large())
                        .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill),
                    Route::Gpu => Container::new(self.stats.gpu.view_large())
                        .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill),
                    Route::Ram => Container::new(self.stats.ram.view_large())
                        .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill),
                }
            )
            .into()
    }

    fn theme(&self) -> Self::Theme {
        match dark_light::detect() {
            dark_light::Mode::Light => Theme::Light,
            _ => Theme::Dark // default is Dark mode
        }
    }

    // update the stats every second
    fn subscription(&self) -> Subscription<Message> {
        every(Duration::from_millis(1000 as u64)).map(|_| Message::Update)
    }
}
