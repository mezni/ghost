use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("PostgreSQL pool error: {0}")]
    PoolError(String),

    #[error("Unexpected error: {0}")]
    Other(String),
}
