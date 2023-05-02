use std::time::Duration;
use iced::{Command, executor, Length, Subscription};
use iced::{Application, Element};
use iced::time::every;
use iced::widget::{Column, Container, Row};
use iced_style::{Theme, theme};

use crate::gpu::GraphState;
use crate::SystemStats;
use crate::ui::{Message, Route};
use crate::ui::style::containers::{MainBox, SecondaryBox};

pub struct App {
    route: Route,
    stats: SystemStats,
}

impl Default for App {
    fn default() -> Self {
        Self {
            route: Route::Cpu,
            stats: SystemStats::new(),
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        "Corroded Monitor".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Update => Command::perform(self.stats.clone().update(), Message::Result),
            Message::Result(s) => {
                self.stats = s;
                Command::none()
            }
            Message::Navigate(r) => {
                self.route = r;
                Command::none()
            }
            Message::CpuPickChanged(v) => {
                self.stats.cpu.graph_state = v;
                Command::none()
            }
            Message::GpuPickChanged(v) => {
                if GraphState::REGION_ONE.contains(&v) {
                    self.stats.gpu.graph_state_1 = v;
                } else if GraphState::REGION_TWO.contains(&v) {
                    self.stats.gpu.graph_state_2 = v;
                } else {
                    self.stats.gpu.graph_state_3 = v;
                }

                Command::none()
            }
        }
    }

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

    fn subscription(&self) -> Subscription<Message> { // update the GUI every second
        every(Duration::from_millis(1000 as u64)).map(|_| Message::Update)
    }
}
