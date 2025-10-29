use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use sqlx::Error as SqlxError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    
    #[error("Keycloak error: {0}")]
    KeycloakError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Internal server error")]
    InternalError,
    
    #[error("Request error: {0}")]
    RequestError(#[from] ReqwestError),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeJsonError),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(e) => {
                eprintln!("Database error: {:?}", e); // Add logging
                HttpResponse::InternalServerError().json(format!("Database error: {}", e))
            }
            AppError::KeycloakError(msg) => {
                HttpResponse::BadRequest().json(msg)
            }
            AppError::AuthError(msg) => {
                HttpResponse::Unauthorized().json(msg)
            }
            AppError::UserNotFound => {
                HttpResponse::NotFound().json("User not found")
            }
            AppError::InvalidInput(msg) => {
                HttpResponse::BadRequest().json(msg)
            }
            AppError::InternalError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            AppError::RequestError(e) => {
                HttpResponse::BadGateway().json(format!("External service error: {}", e))
            }
            AppError::SerializationError(e) => {
                HttpResponse::InternalServerError().json(format!("Serialization error: {}", e))
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;