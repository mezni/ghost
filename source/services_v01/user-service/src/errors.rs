use thiserror::Error;
use warp::{Rejection, Reply};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    // Database errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    // Business logic errors
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserExists,
    #[error("Username already taken")]
    UsernameTaken,
    #[error("Email already registered")]
    EmailTaken,

    // Keycloak errors
    #[error("Keycloak error: {0}")]
    Keycloak(String),
    #[error("Failed to create user in Keycloak")]
    KeycloakUserCreation,
    #[error("Failed to update user in Keycloak")]
    KeycloakUserUpdate,
    #[error("Failed to delete user in Keycloak")]
    KeycloakUserDeletion,

    // Authentication/Authorization errors
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,

    // External service errors
    #[error("External service error: {0}")]
    ExternalService(String),

    // Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),
}

impl AppError {
    pub fn status_code(&self) -> warp::http::StatusCode {
        match self {
            AppError::Validation(_) => warp::http::StatusCode::BAD_REQUEST,
            AppError::UserNotFound => warp::http::StatusCode::NOT_FOUND,
            AppError::UserExists | AppError::UsernameTaken | AppError::EmailTaken => {
                warp::http::StatusCode::CONFLICT
            }
            AppError::Unauthorized => warp::http::StatusCode::UNAUTHORIZED,
            AppError::Forbidden => warp::http::StatusCode::FORBIDDEN,
            _ => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn to_error_response(&self) -> shared::models::ErrorResponse {
        shared::models::ErrorResponse {
            code: self.status_code().as_str().to_string(),
            message: self.to_string(),
            details: None,
            timestamp: chrono::Utc::now(),
        }
    }
}

impl warp::reject::Reject for AppError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    shared::errors::handle_rejection(err).await
}
