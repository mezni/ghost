use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Db(#[from] tokio_postgres::Error),

    #[error("Connection pool error: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),

    #[error("Other error: {0}")]
    Other(String),
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
            AppError::Other(msg) => {
                HttpResponse::InternalServerError().json(format!("Error: {}", msg))
            }
        }
    }
}
