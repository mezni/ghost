use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalError,

    #[error("Not Found")]
    NotFound,

    #[error("Bad Request: {0}")]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalError => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
            AppError::NotFound => HttpResponse::NotFound().body("Not Found"),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().body(msg),
        }
    }
}
