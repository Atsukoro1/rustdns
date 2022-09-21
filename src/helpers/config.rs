use serde::{
    Serialize, 
    Deserialize
};
use std::{
    fs::File, 
    io::Read
};

#[derive(Serialize, Deserialize)]
pub enum LogType {
    TERMINAL = 1,
    FILE = 2
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: Host,
    pub cache: Cache,
    pub logging: Logging
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub hostname: String,
    pub port: u16
}

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub hostname: String,
}

#[derive(Serialize, Deserialize)]
pub struct Logging {
    pub on: bool,
    pub log_type: LogType,
    pub file_path: Option<String>
}

#[derive(Debug)]
pub enum ConfigError {
    TOMLERR(toml::de::Error),
    FSERR(std::io::Error)
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::TOMLERR(err)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::FSERR(err)
    }
}

/// Load config from config.conf file located in root directory
/// Returns config struct
pub fn load_config() -> Result<Config, ConfigError> {
    let mut file = File::open("config.toml")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(toml::from_str::<Config>(&contents)?)
} 