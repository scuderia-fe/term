mod command;
mod config;
mod constants;

use iced::executor;
use iced::theme::palette::Palette;
use iced::widget::column;
use iced::{Application, Color, Settings, Theme};

fn main() -> iced::Result {
    let config: config::ScuderiaTermConfig = config::ScuderiaTermConfig::load();

    ScuderiaTerm::run(Settings {
        antialiasing: true,
        // default_font: Some(include_bytes!("../assets/Roboto-Regular.ttf")),
        default_text_size: 20.0,
        exit_on_close_request: true,
        window: iced::window::Settings {
            size: config.window.size,
            // icon: config.icon,
            transparent: true,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

pub struct ScuderiaTerm {
    history: Vec<command::Command>,
    config: config::ScuderiaTermConfig 
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
        self.config.window.title.clone()
    }

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let config = config::ScuderiaTermConfig::load();
        let history = Vec::new();

        (Self { config, history }, iced::Command::none())
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        print!("{:?}", self.history);
        column![].into()
    }

    fn theme(&self) -> Theme {
        let _palette = Palette {
            background: Color {
                r: self.config.window.background_color.0,
                g: self.config.window.background_color.1,
                b: self.config.window.background_color.2,
                a: self.config.window.background_color.3,
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
