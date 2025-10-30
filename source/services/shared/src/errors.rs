use serde_json::json;
use thiserror::Error;
use warp::{Rejection, Reply};

use crate::models::ErrorResponse; // Import from models

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication required")]
    Unauthorized,

    #[error("Insufficient permissions")]
    Forbidden,

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Validation error: {details}")]
    Validation { details: String },

    #[error("User already exists")]
    UserExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },

    #[error("Internal server error")]
    Internal,
}

impl AppError {
    pub fn status_code(&self) -> warp::http::StatusCode {
        match self {
            AppError::Unauthorized => warp::http::StatusCode::UNAUTHORIZED,
            AppError::Forbidden => warp::http::StatusCode::FORBIDDEN,
            AppError::NotFound { .. } => warp::http::StatusCode::NOT_FOUND,
            AppError::Validation { .. } => warp::http::StatusCode::BAD_REQUEST,
            AppError::UserExists => warp::http::StatusCode::CONFLICT,
            AppError::InvalidCredentials => warp::http::StatusCode::UNAUTHORIZED,
            AppError::ServiceUnavailable { .. } => warp::http::StatusCode::SERVICE_UNAVAILABLE,
            AppError::Internal => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn to_response(&self) -> ErrorResponse {
        match self {
            AppError::Validation { details } => ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: self.to_string(),
                details: Some(json!({ "fields": details })),
                timestamp: chrono::Utc::now(),
            },
            AppError::NotFound { resource } => ErrorResponse {
                code: "NOT_FOUND".to_string(),
                message: self.to_string(),
                details: Some(json!({ "resource": resource })),
                timestamp: chrono::Utc::now(),
            },
            AppError::ServiceUnavailable { service } => ErrorResponse {
                code: "SERVICE_UNAVAILABLE".to_string(),
                message: self.to_string(),
                details: Some(json!({ "service": service })),
                timestamp: chrono::Utc::now(),
            },
            _ => ErrorResponse {
                code: self.status_code().as_str().to_string(),
                message: self.to_string(),
                details: None,
                timestamp: chrono::Utc::now(),
            },
        }
    }
}

impl warp::reject::Reject for AppError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let (code, error_response) = if err.is_not_found() {
        (
            warp::http::StatusCode::NOT_FOUND,
            ErrorResponse::new("NOT_FOUND".to_string(), "Endpoint not found".to_string()),
        )
    } else if let Some(app_error) = err.find::<AppError>() {
        (app_error.status_code(), app_error.to_response())
    } else {
        (
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(
                "INTERNAL_ERROR".to_string(),
                "Internal server error".to_string(),
            ),
        )
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&error_response),
        code,
    ))
}
