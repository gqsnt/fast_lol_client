use std::fs;
use serde::Deserialize;
use crate::AppResult;

#[derive(Deserialize)]
pub struct Config {
    pub riot_path: String,
}

impl Config {
    pub fn new() -> Self {
        let config_data = fs::read_to_string("config.toml").expect("Unable to read config file");
        toml::from_str(&config_data).expect("Unable to parse config file")
    }
}