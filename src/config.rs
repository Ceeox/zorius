use lazy_static::lazy_static;
use std::{env, net::IpAddr, result::Result};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

pub static ENV_NAME: &str = "RUN_MODE";
pub static DEFAULT_CONFIG_FOLDER: &str = "config";
pub static DEFAULT_CONFIG_FILE: &str = "default";
pub static DEVELOPMENT_CONFIG_FILE: &str = "dev";
pub static PRODUCTION_CONFIG_FILE: &str = "prod";

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
    pub firstname: String,
    pub lastname: String,
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
        let mut s = Config::new();
        // load the default config
        s.merge(File::with_name(&format!(
            "{}/{}",
            DEFAULT_CONFIG_FOLDER, DEFAULT_CONFIG_FILE
        )))?;
        // check if we're running in debug or prod mode
        // if run mode is not present always use prod
        let config_path = match env::var(ENV_NAME) {
            Ok(mode) if mode.eq(PRODUCTION_CONFIG_FILE) || mode.eq(DEVELOPMENT_CONFIG_FILE) => {
                format!("{}/{}", DEFAULT_CONFIG_FOLDER, mode)
            }
            _ => format!("{}/{}", DEFAULT_CONFIG_FOLDER, PRODUCTION_CONFIG_FILE),
        };
        s.merge(File::with_name(&config_path).required(true))?;
        // override config if values are present in env
        s.merge(Environment::new().separator("_").ignore_empty(true))?;

        s.try_into()
    }
}
