use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Db(#[from] tokio_postgres::Error),

    #[error("Connection pool error: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl AppError {
    /// Shortcut for 400 Bad Request
    pub fn bad_request(msg: &str) -> Self {
        AppError::BadRequest(msg.to_string())
    }

    /// Shortcut for database error (without constructing a tokio_postgres::Error)
    pub fn db_error(msg: &str) -> Self {
        AppError::Other(format!("Database error: {}", msg))
    }

    /// Shortcut for pool error (optional)
    pub fn pool_error(msg: &str) -> Self {
        AppError::Other(format!("Pool error: {}", msg))
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Io(err) => {
                HttpResponse::InternalServerError().json(format!("I/O error: {}", err))
            }
            AppError::Db(err) => {
                HttpResponse::InternalServerError().json(format!("Database error: {}", err))
            }
            AppError::Pool(err) => {
                HttpResponse::ServiceUnavailable().json(format!("Pool error: {}", err))
            }
            AppError::NotFound(msg) => HttpResponse::NotFound().json(format!("Not found: {}", msg)),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(format!("Bad request: {}", msg)),
            AppError::Other(msg) => {
                HttpResponse::InternalServerError().json(format!("Error: {}", msg))
            }
        }
    }
}
