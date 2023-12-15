use std::error::Error;

use iced::{executor, Element, Settings};

use iced::{Application, Command, Theme};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        "Hello, world!".into()
    }
}
