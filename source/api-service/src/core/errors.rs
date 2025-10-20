// src/core/errors.rs
use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as SqlxError;
use std::env::VarError;
use std::io;
use thiserror::Error;

/// Central application error type
#[derive(Debug, Error)]
pub enum AppError {
    // ───── Database & Env Errors ─────
    #[error("Database error: {0}")]
    Sqlx(#[from] SqlxError),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] VarError),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    // ───── Request/Validation Errors ─────
    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Resource not found")]
    NotFound,

    // ───── Authentication/Authorization Errors ─────
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    // ───── Internal Server Error ─────
    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Map AppError to proper HTTP responses
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // ───── Common Errors ─────
            AppError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(error_body("Bad Request", msg))
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(error_body("Not Found", "Resource not found"))
            }
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json(error_body("Unauthorized", "Access denied"))
            }
            AppError::Forbidden => {
                HttpResponse::Forbidden().json(error_body("Forbidden", "Not allowed"))
            }

            // ───── System Errors ─────
            AppError::Sqlx(err) => HttpResponse::InternalServerError()
                .json(error_body("Database Error", &err.to_string())),
            AppError::EnvVar(err) => HttpResponse::InternalServerError()
                .json(error_body("Environment Error", &err.to_string())),
            AppError::Io(err) => {
                HttpResponse::InternalServerError().json(error_body("IO Error", &err.to_string()))
            }
            AppError::Internal(msg) => {
                HttpResponse::InternalServerError().json(error_body("Internal Server Error", msg))
            }
        }
    }
}

/// JSON error response format
fn error_body(error: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "error": error,
        "message": message
    })
}

/// Convenient alias
pub type AppResult<T> = Result<T, AppError>;
