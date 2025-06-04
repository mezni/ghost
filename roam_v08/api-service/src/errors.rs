use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error), // Only one sqlx::Error variant

    #[error("Service initialization error: {0}")]
    ServiceError(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("An unknown application error occurred.")]
    Unknown,
}
