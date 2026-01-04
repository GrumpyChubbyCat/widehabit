use std::net::Ipv4Addr;

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WideConfig {
    #[serde(default = "default_listen_address")]
    pub listen_address: Ipv4Addr,
    #[serde(default = "default_port")]
    pub listen_port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_db")]
    pub database_url: String,
    #[serde(default = "default_db_pool")]
    pub database_pool: u32,
}

impl WideConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("wide"))
            .build()?;

        config.try_deserialize()
    }
}

fn default_listen_address() -> Ipv4Addr {
    Ipv4Addr::new(127, 0, 0, 1)
}

fn default_port() -> u16 {
    9091
}

fn default_db() -> String {
    String::from("postgresql://lamantin:chokny1975@localhost/widehabit")
}

fn default_db_pool() -> u32 {
    30
}

fn default_log_level() -> String {
    String::from("DEBUG")
}
