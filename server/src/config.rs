use std::net::Ipv4Addr;

use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "default_jwt_secret")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub jwt_secret: String,
    #[serde(default = "default_access_lt")] 
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub access_lt: i64, // Access token lifetime
    #[serde(default = "default_refresh_lt")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub refresh_lt: i64, // Refresh token lifetime
}

#[derive(Clone, Debug, Deserialize)]
pub struct WideConfig {
    #[serde(default = "default_listen_address")]
    pub listen_address: Ipv4Addr,
    #[serde(default = "default_port")]
    pub listen_port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_json_log")]
    pub json_log: bool,
    #[serde(default = "default_db")]
    pub database_url: String,
    #[serde(default = "default_db_pool")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub database_pool: u32,
    #[serde(flatten)]
    pub auth_config: AuthConfig,
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

fn default_json_log() -> bool {
    false
}

fn default_jwt_secret() -> String {
    String::from("95fe6a63-dda8-4613-a0c2-8f99dd7c628f-00321011-0b63-405a-afff-c448f1de71dc")
}

fn default_access_lt() -> i64 {
    15
}

fn default_refresh_lt() -> i64 {
    6
}
