use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use argon2::password_hash;
use deadpool::managed::PoolError;
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

    #[error("Database pool error: {0}")]
    DbPoolError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Password hashing error: {0}")]
    HashError(String),

    #[error("JWT error: {0}")]
    JwtError(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidCredentials | AppError::TokenExpired => StatusCode::UNAUTHORIZED,
            AppError::UserNotFound | AppError::SessionNotFound => StatusCode::NOT_FOUND,
            AppError::HashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
            AppError::DatabaseError(_)
            | AppError::DbPoolError(_)
            | AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        if let AppError::InternalServerError(msg) = self {
            eprintln!("Internal error: {}", msg);
        }

        HttpResponse::build(self.status_code()).json(json!({ "error": self.to_string() }))
    }
}

impl From<PoolError<tokio_postgres::Error>> for AppError {
    fn from(err: PoolError<tokio_postgres::Error>) -> Self {
        AppError::DbPoolError(err.to_string())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: password_hash::Error) -> Self {
        AppError::InternalServerError(format!("Password hash error: {}", err))
    }
}
