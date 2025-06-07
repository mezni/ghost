// src/errors.rs

use actix_web::http::StatusCode; // Add StatusCode
use actix_web::{HttpResponse, ResponseError}; // Add ResponseError and HttpResponse
use sqlx::Error as SqlxError;
use thiserror::Error; // Alias sqlx::Error for clarity

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError), // This is correctly set to accept sqlx::Error

    #[error("Service initialization error: {0}")]
    ServiceError(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("An unknown application error occurred.")]
    Unknown,
    // Add other application-specific errors here as needed
    // #[error("Invalid input: {0}")]
    // InvalidInput(String),
}

// Implement Actix Web's ResponseError trait for AppError
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            // Internal Server Errors (5xx)
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::MissingEnvVar(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            // Example of a Client Error (4xx) - uncomment and add this variant to enum if needed
            // AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string() // Use the #[error] message as the response body
        }))
    }
}
