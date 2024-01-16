use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::{fs, process::exit};
use toml;

lazy_static! {
    /// # `[CONFIG]`
    /// Global static reference to [Bog] configuration data.
    pub static ref CONFIG: Config = Config::read_from_file("config.toml");
}


/// Struct for the `Config` object.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// `Secrets` struct.
    pub secrets: Secrets,
    pub bot: BotConfig,
}

/// Struct for the `Secrets` object. `Secrets` is a child of `Config`.
#[derive(Deserialize, Debug)]
pub struct Secrets {
    /// Discord `client secret`.
    pub discord_token: String,
    /// OpenRouter `API key`.
    pub openrouter_token: String,
}

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    /// Discord bot name.
    pub name: String,
}

impl Config {
    /// Reads the configuration from a file and returns a Config object.
    pub fn read_from_file(filename: &str) -> Config {
        let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Error reading from file: {}", filename);
            exit(1);
        });

        toml::from_str(&contents).unwrap_or_else(|_| {
            eprintln!("Error loading secrets from file: {}", filename);
            exit(1);
        })
    }
}
