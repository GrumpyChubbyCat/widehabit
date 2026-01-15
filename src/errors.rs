use config::ConfigError;
use diesel_async::pooled_connection::{PoolError, bb8::RunError};
use tracing::metadata::ParseLevelError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("Failed to bind TCP listener: {0}")]
    Server(#[from] std::io::Error),

    #[error("Database pool error: {0}")]
    Pool(#[from] PoolError),

    #[error("Config error: {0}")]
    Config(#[from] ConfigError),

    #[error("Log level parse error: {0}")]
    Level(#[from] ParseLevelError)
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),

    #[error("Connection pool error: {0}")]
    RunError(#[from] RunError),

    #[error("Tokio thread pool error: {0}")]
    TokioThreadPoolError(String),

    #[error("Password hash error")]
    HashError(String),

    #[error("JWT encoding error")]
    JWTError(String),

    #[error("Internal cast error")] 
    Cast(String),

    #[error("Entity already exists")]
    AlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Validation failed")]
    Validation(String),

    #[error("Not found")]
    NotFound,

    #[error("User blocked")]
    Blocked,
}