// src/errors.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Database connection error: {0}")] // New variant for connection issues
    DatabaseConnectionError(String),

    #[error("Database error: {0}")] // General database operation errors
    DatabaseError(String), // You could also use #[from] sqlx::Error here for more detail

    #[error("Service initialization error: {0}")]
    ServiceError(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("An unknown application error occurred.")]
    Unknown,
}
