use serde::Deserialize;
use std::error::Error;
use std::fs;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub activity: String
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let path = "./config.toml";
    let contents = fs::read_to_string(path).expect("Failed to read config");
    let config: Config = toml::from_str(contents.as_str()).expect("Failed to parse config");

    Ok(config)
}
