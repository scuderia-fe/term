use serde::{Deserialize, Serialize};

fn get_default_size() -> (u32, u32) {
    (800, 600)
}

fn get_default_title() -> String {
    "ScuderiaTerm".to_owned()
}

fn get_default_background_color() -> (f32, f32, f32, f32) {
    (0.0, 0.0, 0.0, 1.0)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Defines the window configuration.
pub struct ScuderiaTermWindowConfig {
    #[serde(default = "get_default_size")]
    /// The size of the window, in pixels.
    /// The default value is `(800, 600)`.
    pub size: (u32, u32),
    #[serde(default = "get_default_title")]
    /// The title of the window.
    /// The default value is `"ScuderiaTerm"`.
    pub title: String,
    #[serde(default = "get_default_background_color")]
    /// The background color of the window.
    /// The default value is `Color::BLACK`.
    /// TODO: implement ser-de for iced::Color
    pub background_color: (f32, f32, f32, f32),
    #[serde(default)]
    /// The background image of the window.
    /// The default value is `None`.
    /// TODO: implement ser-de for iced::Image
    pub background_image: Option<String>,
    #[serde(default)]
    /// The icon of the window.
    /// The default value is `None`.
    /// TODO: implement ser-de for iced::Image
    pub icon: Option<String>,
}

impl Default for ScuderiaTermWindowConfig {
    fn default() -> Self {
        Self {
            size: get_default_size(),
            title: get_default_title(),
            background_color: get_default_background_color(),
            background_image: None,
            icon: None,
        }
    }
}
