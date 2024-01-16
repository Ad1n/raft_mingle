use serde::Deserialize;
use std::{fs::File, io, io::Read};
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    port: u16,
    scheme: Scheme,
    host: Host,
}

pub type Host = String;

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum Scheme {
    Http,
    Https,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let file = File::open("services/server/config.yml")?;
        let mut content = String::new();
        let _ = std::io::BufReader::new(file).read_to_string(&mut content);
        let config: Config =
            serde_yaml::from_str(&content).expect("Failed to deserialize YAML");

        Ok(config)
    }

    pub fn port(&self) -> u16 {
        *&self.port
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Parse config error")]
    IO(#[from] io::Error),
}
