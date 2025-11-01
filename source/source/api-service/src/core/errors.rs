// src/core/errors.rs
use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as SqlxError;
use std::env::VarError;
use std::io;
use thiserror::Error;

/// Centralized application error type
#[derive(Debug, Error)]
pub enum AppError {
    // ───── Database & System Errors ─────
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

    // ───── Internal Errors ─────
    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Maps AppError to proper HTTP responses
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // ───── Request & Auth Errors ─────
            AppError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(error_body("Bad Request", msg))
            }
            AppError::NotFound => HttpResponse::NotFound().json(error_body(
                "Not Found",
                "The requested resource was not found",
            )),
            AppError::Unauthorized => HttpResponse::Unauthorized()
                .json(error_body("Unauthorized", "Authentication required")),
            AppError::Forbidden => HttpResponse::Forbidden().json(error_body(
                "Forbidden",
                "You do not have permission to access this resource",
            )),

            // ───── System & Database Errors ─────
            AppError::Sqlx(err) => {
                let (status, message) = map_sqlx_error(err);
                HttpResponse::build(status).json(error_body("Database Error", &message))
            }
            AppError::EnvVar(err) => HttpResponse::InternalServerError()
                .json(error_body("Environment Error", &err.to_string())),
            AppError::Io(err) => {
                HttpResponse::InternalServerError().json(error_body("IO Error", &err.to_string()))
            }

            // ───── Internal Errors ─────
            AppError::Internal(msg) => {
                HttpResponse::InternalServerError().json(error_body("Internal Server Error", msg))
            }
        }
    }
}

/// Convert sqlx errors into user-friendly messages
fn map_sqlx_error(err: &SqlxError) -> (actix_web::http::StatusCode, String) {
    use actix_web::http::StatusCode;
    match err {
        SqlxError::RowNotFound => (StatusCode::NOT_FOUND, "Record not found".to_string()),
        SqlxError::Database(db_err) => (
            StatusCode::BAD_REQUEST,
            format!("Database constraint error: {}", db_err.message()),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unexpected database error".to_string(),
        ),
    }
}

/// Standard JSON error response format
fn error_body(error: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "error": error,
        "message": message
    })
}

/// Convenient result alias
pub type AppResult<T> = Result<T, AppError>;
