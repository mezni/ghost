// errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Session not found")]
    SessionNotFound,
    #[error("Token expired")]
    TokenExpired,
    #[error("Internal server error")]
    InternalServerError,
}

impl actix_web::error::ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::UserNotFound => actix_web::http::StatusCode::NOT_FOUND,
            AuthError::SessionNotFound => actix_web::http::StatusCode::NOT_FOUND,
            AuthError::TokenExpired => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(serde_json::json!({ "error": self.to_string() }))
    }
}