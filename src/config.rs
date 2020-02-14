use std::net::IpAddr;
use std::io::{Result, Read, Write};
use std::io::{BufReader, BufWriter };

use ron::{de::from_reader, ser::to_string_pretty, ser::PrettyConfig};
use serde::{Serialize, Deserialize};

const CONFIG_NAME: &str = "./config.conf";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ZoriusConfig {
    #[serde()]
    pub web_config: WebServerConfig,
    pub db_config: DbServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebServerConfig {
    pub ip: IpAddr,
    pub port: u16,

}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DbServerConfig {
    pub username: String,
    pub server_domain: String,
    pub password: String,
    pub application_name: String,
    pub db_name: String,
}

impl Default for WebServerConfig {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".parse::<IpAddr>().unwrap(),
            port: 8080,
        }
    }
}

impl ZoriusConfig {
    pub fn new() -> Result<Self> {
        match Self::load_config() {
            Ok(r) => Ok(r),
            Err(_) => {
                let conf = Self::default();
                let _ = std::fs::File::create(CONFIG_NAME)?;
                conf.save_config()?;
                Ok(conf)
            }
        }
    }

    pub fn load_config() -> Result<Self> {
        let file = std::fs::File::open(CONFIG_NAME)?;
        let buf = std::io::BufReader::new(file);
        // TODO: replace expect with a ZoriusError enum 
        let res = ron::de::from_reader(buf).expect("ron: failed to read config file");
        Ok(res)
    }

    pub fn save_config(&self) -> Result<()> {
        let file = std::fs::File::create(CONFIG_NAME)?;
        let mut buf = std::io::BufWriter::new(file);
        // TODO: remove unwrap
        let pretty_config = ron::ser::PrettyConfig::default();
        let config = ron::ser::to_string_pretty(&self, pretty_config).unwrap();
        let _ = buf.write_all(&mut (config.into_bytes()))?;
        buf.flush()?;
        Ok(())
    }
}