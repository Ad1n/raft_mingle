use error::CustomError;
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    port: u16,
    protocol: Protocol,
    domain: Domain,
}

pub type Domain = String;

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum Protocol {
    Http,
    Https,
}

impl Config {
    pub fn new() -> Result<Self, CustomError> {
        let file = File::open("services/server/config.yml")
            .map_err(|err| CustomError::new(&err.to_string()))?;
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
