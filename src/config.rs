use lazy_static::lazy_static;
use std::{net::IpAddr, result::Result};

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().expect("Failed to load config");
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub web: WebServerConfig,
    pub db: DbServerConfig,
    pub secret_key: String,
    pub domain: String,
    pub token_lifetime: i64,
    pub registration_enabled: bool,
    pub mailer: MailConfig,
    pub log_level: String,
}

#[derive(Debug, Deserialize)]
pub struct WebServerConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub enable_ssl: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub log_format: String,
}

#[derive(Debug, Deserialize)]
pub struct DbServerConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct MailConfig {
    pub enable_mailer: bool,
    pub smtp_address: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub merchandise_email_send_to: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();
        builder = builder.set_default("default", "1")?;
        builder = builder.add_source(File::new("config/default", FileFormat::Toml).required(true));

        if cfg!(debug_assertions) {
            builder = builder.add_source(File::new("config/dev", FileFormat::Toml).required(false));
        } else {
            builder =
                builder.add_source(File::new("config/prod", FileFormat::Toml).required(false));
        }

        builder = builder.add_source(Environment::default().separator("_").ignore_empty(true));

        let config = builder.build()?;

        config.try_deserialize()
    }
}
