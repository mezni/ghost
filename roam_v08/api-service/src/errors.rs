// src/infra/errors.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // Renamed from InfraError
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Service initialization error: {0}")]
    ServiceError(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("An unknown application error occurred.")] // Updated message
    Unknown,
}
