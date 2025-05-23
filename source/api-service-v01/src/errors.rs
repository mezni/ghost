use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::{error::Error, fmt};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    DbError(String),
    NotFound(String),
    ValidationError(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DbError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        let error_response = ErrorResponse { message: error_message };

        match self {
            AppError::DbError(_) | AppError::InternalError(_) => {
                HttpResponse::InternalServerError().json(error_response)
            }
            AppError::ValidationError(_) => HttpResponse::BadRequest().json(error_response),
            AppError::NotFound(_) => HttpResponse::NotFound().json(error_response),
        }
    }
}
