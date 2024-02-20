use clap::{arg, Command};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{io, io::Read};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub scheme: Scheme,
    pub host: Host,
    pub id: u8,
    pub rpc_clients: HashMap<u8, hyper::Uri>,
}

pub type Host = String;

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum Scheme {
    Http,
    Https,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let rpc_uris: Vec<_> = Self::try_from_env_with_delimiter("RPC_CLIENTS_PORTS")
            .into_iter()
            .map(|p| {
                let uri_str = format!("http://localhost:{}", p);
                hyper::Uri::try_from(&uri_str)
            })
            .map(|r| r.unwrap())
            .collect();

        let peer_ids = Self::try_from_env_with_delimiter("PEER_IDS")
            .iter()
            .map(|s| u8::from_str(s).unwrap())
            .collect::<Vec<u8>>();

        let rpc_clients = HashMap::from(
            peer_ids
                .into_iter()
                .zip(rpc_uris.into_iter())
                .collect::<HashMap<u8, hyper::Uri>>(),
        );

        Ok(Self {
            port: Self::try_from_env("PORT")?,
            scheme: Scheme::Http,
            host: "localhost".to_string(),
            id: Self::try_from_env("ID")?,
            rpc_clients,
        })
    }

    pub fn try_from_env<T: AsRef<OsStr> + Display + Clone, R: FromStr>(
        var: T,
    ) -> Result<R, ConfigError> {
        Ok(std::env::var(var.clone())
            .unwrap()
            .parse::<R>()
            .map_err(|_| ParseEnvVariableError {
                message: format!("ENV variable missing {}", var),
            })?)
    }

    pub fn try_from_env_with_delimiter<T: AsRef<OsStr> + Display + Clone>(
        var: T,
    ) -> Vec<String> {
        std::env::var(var)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn peer_ids(&self) -> Vec<u8> {
        self.rpc_clients.keys().cloned().collect()
    }

    pub fn clients_uris(&self) -> Vec<hyper::Uri> {
        self.rpc_clients.values().cloned().collect()
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
    #[error("Parse env error: {0}")]
    ParseEnvVar(#[from] ParseEnvVariableError),
}

#[derive(Debug)]
pub struct ParseEnvVariableError {
    pub message: String,
}

impl Display for ParseEnvVariableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseEnvVariableError {}
