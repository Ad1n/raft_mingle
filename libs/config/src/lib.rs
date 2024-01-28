use clap::{arg, Command};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::path::Path;
use std::{fs::File, io, io::Read};
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    port: u16,
    scheme: Scheme,
    host: Host,
    rpc_clients: Vec<Host>,
}

pub type Host = String;

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum Scheme {
    Http,
    Https,
}

impl Config {
    pub fn new(path: Box<Path>) -> Result<Self, ConfigError> {
        let file = File::open(path)?;
        let mut content = String::new();
        let _ = io::BufReader::new(file).read_to_string(&mut content);
        let config: Config =
            serde_yaml::from_str(&content).expect("Failed to deserialize YAML");

        Ok(config)
    }

    pub fn port(&self) -> u16 {
        *&self.port
    }

    pub fn try_port_from_env(&self) -> Result<u16, ConfigError> {
        let r = std::env::var("PORT")
            .unwrap_or_else(|_| self.port.to_string())
            .parse()?;

        Ok(r)
    }
}

pub fn get_config_object<T>(config_name: &str) -> Result<T, ConfigError>
where
    T: DeserializeOwned,
{
    let contents = std::fs::read_to_string(config_name)?;
    Ok(serde_yaml::from_str::<T>(&contents)?)
}

pub fn initialize_config<T>(name: &'static str, about: &'static str) -> T
where
    T: DeserializeOwned,
{
    let app = Command::new(name)
        .version("1.0")
        .about(about)
        .arg(arg!(-c --config<FILE> "Sets config file").required(true))
        .get_matches();

    let config_name = app
        .get_one::<String>("config")
        .expect("config parameter")
        .to_owned();
    let config = get_config_object::<T>(&config_name);

    config.expect("initialize_app: deserialize config")
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Parse config error")]
    IO(#[from] io::Error),
    #[error("Parse yaml error")]
    YamlParse(#[from] serde_yaml::Error),
    #[error("Parse port error")]
    ParsePort(#[from] std::num::ParseIntError),
}
