use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub riot_path: String,
}

impl Config {
    pub fn new() -> Self {
        let config_file = PathBuf::from("config.toml");
        if config_file.exists() {
            let config_data = fs::read_to_string(config_file).expect("Unable to read config file");
            toml::from_str(&config_data).expect("Unable to parse config file")
        } else {
            panic!("Config file not found");
        }

    }
}

