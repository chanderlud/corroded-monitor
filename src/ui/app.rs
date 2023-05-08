use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use iced::{Alignment, Command, executor, Length, Padding, Subscription};
use iced::{Application, Element};
use iced::time::every;
use iced::widget::{Button, Column, Container, PickList, Row, Scrollable, Space, Text, Toggler};
use iced::widget::scrollable::Properties;
use iced_style::{theme, Theme as IcedTheme};
use tokio::sync::Mutex;

use crate::config::Config;
use crate::gpu::GraphState;
use crate::system::{HardwareMonitor, SystemStats};
use crate::ui::{Message, Route, Theme};
use crate::ui::style::button::SettingsButton;
use crate::ui::style::container::{MainBox, SecondaryBox};
use crate::ui::style::pick_list::PickList as PickListStyle;
use crate::ui::style::scrollable::Scrollable as ScrollableStyle;
use crate::ui::style::toggler::Toggler as TogglerStyle;

pub(crate) struct App {
    route: Route,
    stats: SystemStats,
    monitor: Option<Arc<Mutex<HardwareMonitor>>>,
    config: Config,
}

impl From<Config> for App {
    fn from(config: Config) -> Self {
        Self {
            route: Route::Cpu,
            stats: SystemStats::new(),
            monitor: None, // monitor is initialized asynchronously later
            config,
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = IcedTheme;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // creating the hardware monitor takes a second so its done asynchronously
        (App::from(flags), Command::perform(HardwareMonitor::new(), Message::MonitorCreated))
    }

    fn title(&self) -> String {
        String::from("Corroded Monitor")
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
            Message::UpdateCompleted(mut updated_stats) => {
                // in case the graph states have changed since the update began
                updated_stats.cpu.graph_state = self.stats.cpu.graph_state;

                for (index, gpu) in updated_stats.gpus.iter_mut().enumerate() {
                    if self.stats.gpus.len() > 0 {
                        gpu.graph_state_1 = self.stats.gpus[index].graph_state_1;
                        gpu.graph_state_2 = self.stats.gpus[index].graph_state_2;
                        gpu.graph_state_3 = self.stats.gpus[index].graph_state_3;
                    }
                }

                for (index, disk) in updated_stats.disks.iter_mut().enumerate() {
                    if self.stats.disks.len() > 0 {
                        disk.graph_state = self.stats.disks[index].graph_state;
                    }
                }

                self.stats = updated_stats;
                Command::none()
            }
            Message::MonitorCreated(monitor) => {
                self.monitor = Some(monitor);
                Command::none()
            }
            Message::Navigate(route) => {
                self.route = route;
                Command::none()
            }
            Message::CpuPickChanged(state) => {
                self.stats.cpu.graph_state = state;
                Command::none()
            }
            Message::GpuPickChanged(state) => {
                if GraphState::REGION_ONE.contains(&state) {
                    for gpu in &mut self.stats.gpus {
                        gpu.graph_state_1 = state;
                    }
                } else if GraphState::REGION_TWO.contains(&state) {
                    for gpu in &mut self.stats.gpus {
                        gpu.graph_state_2 = state;
                    }
                } else {
                    for gpu in &mut self.stats.gpus {
                        gpu.graph_state_3 = state;
                    }
                }

                Command::none()
            }
            Message::StoragePickChanged(state) => {
                for disk in &mut self.stats.disks {
                    disk.graph_state = state;
                }

                Command::none()
            }
            Message::ThemeChanged(theme) => {
                self.config.theme = theme;
                self.config.save().expect("Failed to save config");
                Command::none()
            }
            Message::TemperatureUnitChanged => {
                self.config.celsius = !self.config.celsius;
                self.config.save().expect("Failed to save config");
                Command::none()
            }
        }
    }

    // the base of the GUI
    fn view(&self) -> Element<'_, Self::Message> {
        let mut gpu_items = Column::new();

        for gpu in &self.stats.gpus {
            gpu_items = gpu_items.push(gpu.view_small(self.config.celsius));
        }

        let mut disk_items = Column::new();

        for disk in &self.stats.disks {
            disk_items = disk_items.push(disk.view_small(self.config.celsius));
        }

        let mut network_items = Column::new();

        for adapter in &self.stats.network_adapters {
            network_items = network_items.push(adapter.view_small());
        }


        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Container::new(
                    Scrollable::new(
                        Column::new()
                            .push(self.stats.cpu.view_small(self.config.celsius))
                            .push(gpu_items)
                            .push(self.stats.ram.view_small())
                            .push(disk_items)
                            .push(network_items)
                            .push(Space::new(Length::Fill, Length::Fixed(10.0)))
                            .push(
                                Row::new()
                                    .push(Space::new(Length::FillPortion(1), Length::Shrink))
                                    .push(
                                        Button::new(
                                            Column::new()
                                                .push(Space::new(Length::Fill, Length::Fill))
                                                .push(
                                                    Row::new()
                                                        .push(Space::new(Length::Fill, Length::Shrink))
                                                        .push(Text::new("Settings"))
                                                        .push(Space::new(Length::Fill, Length::Shrink))
                                                )
                                                .push(Space::new(Length::Fill, Length::Fill))
                                        )
                                            .on_press(Message::Navigate(Route::Settings))
                                            .style(theme::Button::Custom(Box::new(SettingsButton)))
                                            .width(Length::FillPortion(3))
                                            .height(Length::Fixed(50.0))
                                            .padding(Padding::new(10.0))
                                    )
                                    .push(Space::new(Length::FillPortion(1), Length::Shrink))
                            )
                            .push(Space::new(Length::Fill, Length::Fixed(20.0)))
                    )
                        .style(theme::Scrollable::Custom(Box::new(ScrollableStyle)))
                        .vertical_scroll(
                            Properties::new()
                                .scroller_width(6)
                                .margin(0.5)
                        )
                ).style(theme::Container::Custom(Box::new(SecondaryBox))).height(Length::Fill).width(Length::Fixed(300.0))
            )
            .push(
                match self.route {
                    Route::Cpu => Container::new(self.stats.cpu.view_large(self.config.celsius))
                        .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill),
                    Route::Gpu(index) => Container::new(self.stats.gpus[index].view_large(self.config.celsius))
                        .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill),
                    Route::Ram => Container::new(self.stats.ram.view_large())
                        .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill),
                    Route::Storage(index) => Container::new(self.stats.disks[index].view_large(self.config.celsius))
                        .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill),
                    Route::Network(index) => Container::new(self.stats.network_adapters[index].view_large())
                        .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill),
                    Route::Settings => {
                        Container::new(
                            Column::new()
                                .padding(20)
                                .spacing(10)
                                .push(
                                    Row::new()
                                        .height(Length::Fixed(30.0))
                                        .push(Text::new("Settings").size(28))
                                )
                                .push(Space::new(Length::Fill, Length::Fixed(10.0)))
                                .push(
                                    Row::new()
                                        .spacing(10)
                                        .align_items(Alignment::Center)
                                        .push(Text::new("Theme").size(20))
                                        .push(
                                            PickList::new(&Theme::ALL[..], Some(self.config.theme), Message::ThemeChanged)
                                                .style(
                                                    theme::PickList::Custom(
                                                        Rc::new(PickListStyle),
                                                        Rc::new(PickListStyle),
                                                    )
                                                )
                                                .padding(5)
                                        )
                                )
                                .push(
                                    Row::new()
                                        .spacing(10)
                                        .align_items(Alignment::Center)
                                        .push(Text::new("Fahrenheit").size(20))
                                        .push(
                                            Toggler::new(None, self.config.celsius, |_| { Message::TemperatureUnitChanged })
                                                .width(Length::Shrink)
                                                .style(theme::Toggler::Custom(Box::new(TogglerStyle)))
                                        )
                                        .push(Text::new("Celsius").size(20))
                                )
                        )
                            .style(theme::Container::Custom(Box::new(MainBox))).height(Length::Fill).width(Length::Fill)
                    } // TODO overlay settings
                }
            )
            .into()
    }

    fn theme(&self) -> Self::Theme {
        match self.config.theme {
            Theme::System => match dark_light::detect() {
                dark_light::Mode::Light => IcedTheme::Light,
                _ => IcedTheme::Dark // Default and Dark map to Dark
            }
            Theme::Light => IcedTheme::Light,
            Theme::Dark => IcedTheme::Dark,
        }
    }

    // update the stats every second
    fn subscription(&self) -> Subscription<Message> {
        every(Duration::from_millis(1000)).map(|_| Message::Update)
    }
}
