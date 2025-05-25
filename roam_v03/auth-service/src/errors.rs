// src/errors.rs
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use dotenvy::Error as DotenvError;
use serde_json::json;
use std::io;
use thiserror::Error;
use tokio_postgres::Error as PgError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid environment variable format for {0}")]
    InvalidEnvVarFormat(String),

    #[error("Dotenv error: {0}")]
    DotenvError(#[from] DotenvError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] PgError),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Session not found")]
    SessionNotFound,

    #[error("Token expired")]
    TokenExpired,

    #[error("Database pool creation error: {0}")]
    DbPoolCreateError(String),  // or use appropriate error type

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidCredentials | AppError::TokenExpired => StatusCode::UNAUTHORIZED,
            AppError::UserNotFound | AppError::SessionNotFound => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_)
            | AppError::DbPoolCreateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({ "error": self.to_string() }))
    }
}
