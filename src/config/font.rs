use serde::{Deserialize, Serialize};

fn get_default_font_size() -> f32 {
    20.0
}

fn get_default_font_color() -> (f32, f32, f32, f32) {
    (1.0, 1.0, 1.0, 1.0)
}

fn get_default_font_family() -> String {
    "Roboto".to_owned()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Defines the font configuration.
/// TODO: implement ser-de for iced::Font
pub struct ScuderiaTermFontConfig {
    #[serde(default = "get_default_font_size")]
    /// The size of the font.
    /// The default value is `20.0`.
    pub size: f32,
    #[serde(default = "get_default_font_color")]
    /// The color of the font.
    /// The default value is `Color::WHITE`.
    /// TODO: implement ser-de for iced::Color
    pub color: (f32, f32, f32, f32),
    #[serde(default = "get_default_font_family")]
    /// The family of the font.
    /// The default value is `"Roboto"`.
    pub family: String,
}

impl Default for ScuderiaTermFontConfig {
    fn default() -> Self {
        Self {
            size: get_default_font_size(),
            color: get_default_font_color(),
            family: get_default_font_family(),
        }
    }
}