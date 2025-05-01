use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    InternalServerError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
}

impl ApiError {
    pub fn to_response(&self) -> ErrorResponse {
        match self {
            ApiError::BadRequest(msg) => ErrorResponse {
                error: "BadRequest".to_string(),
                message: msg.to_string(),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
            },
            ApiError::Unauthorized(msg) => ErrorResponse {
                error: "Unauthorized".to_string(),
                message: msg.to_string(),
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
            },
            ApiError::NotFound(msg) => ErrorResponse {
                error: "NotFound".to_string(),
                message: msg.to_string(),
                status_code: StatusCode::NOT_FOUND.as_u16(),
            },
            ApiError::InternalServerError(msg) => ErrorResponse {
                error: "InternalServerError".to_string(),
                message: msg.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            },
        }
    }
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_response())
    }
}