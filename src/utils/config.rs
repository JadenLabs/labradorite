use poise::serenity_prelude::Color;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use toml;

/// A struct representing the `config.toml`
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub activity: String,
    pub colors: Colors,
    pub emojis: Emojis,
    pub github: String,
}

#[allow(dead_code)]
impl Config {
    /// Loads the `config.toml` into this struct
    /// ```rust
    /// let config = Config::load().unwrap();
    /// println!("Name: {}", config.name);
    /// ```
    pub fn load() -> Result<Config, Box<dyn Error>> {
        let path = "./config.toml";
        let contents = fs::read_to_string(path).expect("Failed to read config");
        let config: Config = toml::from_str(contents.as_str()).expect("Failed to parse config");

        Ok(config)
    }
}

/// A struct representing the colors in the config
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Colors {
    pub primary: String,
    pub secondary: String,
}

pub trait ToColor {
    fn to_color(&self) -> Color;
}

/// Adds the to_color method to Strings meant for `Config::Colors`
/// ```rust
/// let config = config::load_config().unwrap();
/// println!("Color: {}", config.colors.primary.to_color());
/// ```
impl ToColor for String {
    fn to_color(&self) -> Color {
        let hex = self.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Color::from_rgb(r, g, b)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
/// A struct representing the Emojis in the config
pub struct Emojis {
    pub arr_r: String,
    pub chat: String,
    pub dns: String,
    pub network: String,
    pub trend_up: String,
}
