use config;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration Error: {0}")]
    Configuration(#[from] config::ConfigError),

    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}
