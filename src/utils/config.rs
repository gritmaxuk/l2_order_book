use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub exchange: ExchangeConfig,
}

#[derive(Deserialize)]
pub struct ExchangeConfig {
    pub url: String,
    pub depth_limit: usize,
    pub instrument: Option<String>, 
}

impl Config {
    pub fn from_file(file: &str) -> Self {
        let content = fs::read_to_string(file).expect("Unable to read config file");
        toml::from_str(&content).expect("Unable to parse config file")
    }
}