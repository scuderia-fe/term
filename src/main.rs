mod command;
mod config;
mod constants;

use iced::theme::palette::Palette;
use iced::widget::column;
use iced::{executor, Application, Color, Sandbox, Settings, Theme};

fn main() -> iced::Result {
    ScuderiaTerm::run(Settings::default())
}

pub struct ScuderiaTerm {
    config: config::ScuderiaTermConfig,
    history: Vec<command::Command>,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
}

impl Application for ScuderiaTerm {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn title(&self) -> String {
        self.config.title.clone()
    }

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let config = config::ScuderiaTermConfig::load();
        let history = Vec::new();

        (Self { config, history }, iced::Command::none())
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![].into()
    }

    fn theme(&self) -> Theme {
        let _palette = Palette {
            background: Color {
                r: self.config.background_color.0,
                g: self.config.background_color.1,
                b: self.config.background_color.2,
                a: self.config.background_color.3,
            },
            primary: iced::Color::BLACK,
            success: iced::Color::BLACK,
            danger: iced::Color::BLACK,
            text: iced::Color::BLACK,
        };

        // TODO: implement custom theme
        Theme::Dark
    }
}
