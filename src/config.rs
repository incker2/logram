use failure::Error;
use serde_derive::Deserialize;
use std::fs::File;

use crate::source::LogSourcesConfig;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub sources: LogSourcesConfig,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelegramConfig {
    pub token: String,
    pub chat_id: String,
}

impl Config {
    pub fn read(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let config: Self = serde_yaml::from_reader(file)?;

        Ok(config)
    }
}
