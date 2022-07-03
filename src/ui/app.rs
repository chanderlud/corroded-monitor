use iced::{Application, Clipboard, Column, Command, Container, Element, executor, Length, Row, Subscription};

use crate::SystemStats;
use crate::ui::{Message, Route, style};

pub struct App {
    route: Route,
    stats: SystemStats
}

impl Default for App {
    fn default() -> Self {
        Self {
            route: Route::Cpu,
            stats: SystemStats::new()
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        "Corroded Monitor".to_string()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::Update => Command::perform(self.stats.clone().update(), Message::Result),
            Message::Result(s) => { self.stats = s; Command::none() },
            Message::Navigate(r) => { self.route = r; Command::none() },
            Message::PickChanged(v) => { self.stats.cpu.graph_state = Some(v); Command::none() }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1000 as u64)).map(|_| Message::Update)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let (cpu_small, cpu_large) = self.stats.cpu.view();
        let (gpu_small, gpu_large) = self.stats.gpu.view();
        let (ram_small, ram_large) = self.stats.ram.view();

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Container::new(
                    Column::new()
                        .push(cpu_small)
                        .push(gpu_small)
                        .push(ram_small)
                ).style(style::Container::Secondary).height(Length::Fill).width(Length::Units(300))
            )
            .push(
                match self.route {
                    Route::Cpu => Container::new(
                        cpu_large
                    ).style(style::Container::Main).height(Length::Fill).width(Length::Fill),
                    Route::Gpu => Container::new(
                        gpu_large
                    ).style(style::Container::Main).height(Length::Fill).width(Length::Fill),
                    Route::Ram => Container::new(
                        ram_large
                    ).style(style::Container::Main).height(Length::Fill).width(Length::Fill),
                }
            )
            .into()
    }
}
