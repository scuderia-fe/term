mod window;
mod font;

use serde::{Deserialize, Serialize};

use crate::constants::APPLICATION_NAME;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScuderiaTermConfig {
    #[serde(default)]
    pub window: window::ScuderiaTermWindowConfig,
    #[serde(default)]
    pub font: font::ScuderiaTermFontConfig,
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
            window: window::ScuderiaTermWindowConfig::default(),
            font: font::ScuderiaTermFontConfig::default(),
        }
    }

    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        let app_dir = Self::get_app_dir();

        let config = std::fs::read_to_string(config_path);

        match config {
            Ok(config) => {
                let config: ScuderiaTermConfig = toml::from_str(&config).unwrap();
                if cfg!(debug_assertions) {
                    println!("Loaded config: {:?}", config);
                }
                std::fs::write(Self::get_config_path(), toml::to_string(&config).unwrap()).unwrap();
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
}