use thiserror::Error;
use actix_web::{HttpResponse, http::StatusCode, error::ResponseError};
use serde_json::json;
use std::io;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid environment variable format for {0}")]
    InvalidEnvVarFormat(String),

    #[error(transparent)]
    DotenvError(#[from] dotenvy::Error),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Session not found")]
    SessionNotFound,

    #[error("Token expired")]
    TokenExpired,

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidCredentials | AppError::TokenExpired => StatusCode::UNAUTHORIZED,
            AppError::UserNotFound | AppError::SessionNotFound => StatusCode::NOT_FOUND,

            // Treat these environment/config related errors as 500 internal errors
            AppError::IoError(_) 
            | AppError::MissingEnvVar(_) 
            | AppError::InvalidEnvVarFormat(_) 
            | AppError::DotenvError(_) 
            | AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(json!({ "error": self.to_string() }))
    }
}

