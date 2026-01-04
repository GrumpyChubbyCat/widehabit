use config::ConfigError;
use diesel_async::pooled_connection::PoolError;
use tracing::metadata::ParseLevelError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WideAppError {
    #[error("Failed to bind TCP listener: {0}")]
    Server(#[from] std::io::Error),

    #[error("Database pool error: {0}")]
    Pool(#[from] PoolError),

    #[error("Config error: {0}")]
    Config(#[from] ConfigError),

    #[error("Log level parse error: {0}")]
    Level(#[from] ParseLevelError)
}