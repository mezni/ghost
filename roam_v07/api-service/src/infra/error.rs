use thiserror::Error;
use actix_web::http::StatusCode;
use actix_web::{ResponseError, HttpResponse};
use sqlx::Error as SqlxError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    Validation(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        log::error!("Application Error: {:?}", self); // Log the error

        match self {
            AppError::DatabaseError(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .json("Database error occurred."),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            AppError::Validation(msg) => HttpResponse::BadRequest().json(msg),
            AppError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            AppError::Forbidden => HttpResponse::Forbidden().json("Forbidden"),
            AppError::Conflict(msg) => HttpResponse::Conflict().json(msg),
            AppError::InternalServerError => HttpResponse::InternalServerError().json("An unexpected error occurred."),
        }
    }
}