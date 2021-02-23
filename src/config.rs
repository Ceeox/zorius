use lazy_static::lazy_static;
use std::{env, net::IpAddr, result::Result, usize};

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().expect("Failed to load config");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub web: WebServerConfig,
    pub db: DbServerConfig,
    pub secret_key: String,
    pub domain: String,
    pub token_lifetime: i64,
    pub registration_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebServerConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub enable_ssl: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub log_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbServerConfig {
    pub server: String,
    pub username: String,
    pub password: String,
    pub app_name: String,
    pub name: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        // load the default config
        s.merge(File::with_name("config/default"))?;
        // check if we're running in debug mode
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        // load the dev or prod config file
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        // override config if values are present in env
        s.merge(Environment::new().separator("_").ignore_empty(true))?;

        s.try_into()
    }
}
