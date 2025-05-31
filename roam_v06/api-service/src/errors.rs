use config;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration Error: {0}")]
    Configuration(#[from] config::ConfigError),
}
