use serde::{Deserialize, Serialize};

use crate::constants::APPLICATION_NAME;

fn get_default_size() -> (u32, u32) {
    (800, 600)
}

fn get_default_title() -> String {
    String::from("ScuderiaTerm")
}

fn get_default_background_color() -> (f32, f32, f32, f32) {
    (0.0, 0.0, 0.0, 1.0)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScuderiaTermConfig {
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
}

impl ScuderiaTermConfig {
    pub fn get_app_dir() -> String {
        format!(
            "{}/.config/{}",
            dirs::home_dir().unwrap().to_str().unwrap(),
            APPLICATION_NAME
        )
    }

    pub fn get_config_path() -> String {
        format!("{}/config.toml", Self::get_app_dir())
    }

    pub fn new() -> Self {
        Self {
            size: (800, 600),
            title: String::from("ScuderiaTerm"),
            background_color: (0.0, 0.0, 0.0, 1.0),
            background_image: None,
        }
    }

    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        let app_dir = Self::get_app_dir();

        let config = std::fs::read_to_string(config_path);

        match config {
            Ok(config) => {
                let config: ScuderiaTermConfig = toml::from_str(&config).unwrap();
                config
            }
            Err(_) => {
                let config = Self::new();
                // check if config dir exists, if not create it
                if !std::path::Path::new(&app_dir).exists() {
                    std::fs::create_dir_all(&app_dir).unwrap();
                }
                std::fs::write(Self::get_config_path(), toml::to_string(&config).unwrap()).unwrap();
                config
            }
        }
    }

    // pub fn save(&self) -> () {
    //     let config = toml::to_string(&self).unwrap();
    //     std::fs::write(Self::get_config_path(), config).unwrap();
    // }
}
