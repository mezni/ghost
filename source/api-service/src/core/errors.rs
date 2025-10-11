use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use serde::Serialize;
use thiserror::Error;
use tokio_postgres::Error as PgError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    DbError(String),

    #[error("Connection pool error: {0}")]
    PoolError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Other error: {0}")]
    Other(String),
}

// -------------------- From implementations --------------------

// Convert deadpool_postgres::PoolError -> AppError
impl From<PoolError> for AppError {
    fn from(err: PoolError) -> Self {
        AppError::PoolError(err.to_string())
    }
}

// Convert tokio_postgres::Error -> AppError
impl From<PgError> for AppError {
    fn from(err: PgError) -> Self {
        AppError::DbError(err.to_string())
    }
}

// -------------------- Shortcuts --------------------

impl AppError {
    pub fn bad_request(msg: &str) -> Self {
        AppError::BadRequest(msg.to_string())
    }

    pub fn db_error(msg: &str) -> Self {
        AppError::DbError(msg.to_string())
    }

    pub fn pool_error(msg: &str) -> Self {
        AppError::PoolError(msg.to_string())
    }
}

// -------------------- JSON response --------------------

#[derive(Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let err_json = ErrorResponse {
            status: "error".into(),
            message: self.to_string(),
        };

        match self {
            AppError::Io(_) | AppError::DbError(_) | AppError::Other(_) => {
                HttpResponse::InternalServerError().json(err_json)
            }
            AppError::PoolError(_) => HttpResponse::ServiceUnavailable().json(err_json),
            AppError::NotFound(_) => HttpResponse::NotFound().json(err_json),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(err_json),
        }
    }
}
