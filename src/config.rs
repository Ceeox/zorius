use lazy_static::lazy_static;
use std::{net::IpAddr, result::Result};

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().expect("Failed to load config");
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub web: WebServerConfig,
    pub db: DbServerConfig,
    pub secret_key: String,
    pub domain: String,
    pub token_lifetime: i64,
    pub registration_enabled: bool,
    pub mailer: MailConfig,
    pub log_level: String,
    pub admin_user: AdminUser,
}

#[derive(Debug, Deserialize)]
pub struct AdminUser {
    pub email: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
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
    pub server: String,
    pub username: String,
    pub password: String,
    pub app_name: String,
    pub name: String,
    pub port: u16,
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
        builder = builder.add_source(File::new("config/default", FileFormat::Toml));

        if cfg!(debug_assertions) {
            builder = builder.add_source(File::new("config/dev", FileFormat::Toml));
        } else {
            builder = builder.add_source(File::new("config/prod", FileFormat::Toml));
        }

        builder = builder.add_source(Environment::default().separator("_").ignore_empty(true));

        let config = builder.build()?;

        config.try_deserialize()
    }
}
