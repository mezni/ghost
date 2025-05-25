use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio_postgres::error::Error as PgError;
use validator::ValidationErrors;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum AppError {
    // 400 - Bad Request
    Validation(ValidationErrors),
    InvalidRequest(String),
    MalformedData(String),

    // 401 - Unauthorized
    Unauthorized,
    InvalidCredentials,
    ExpiredToken,
    InvalidToken,

    // 403 - Forbidden
    Forbidden,
    InsufficientPermissions,

    // 404 - Not Found
    NotFound,
    UserNotFound,
    ResourceNotFound(String),

    // 409 - Conflict
    Conflict,
    UserExists,
    EmailInUse,

    // 422 - Unprocessable Entity
    UnprocessableEntity(Vec<String>),

    // 500 - Internal Server Error
    InternalServerError,
    DatabaseError(PgError),
    HashingError(argon2::password_hash::Error),
    JwtError(jsonwebtoken::errors::Error),
    IoError(std::io::Error),
    ConfigurationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Validation(err) => write!(f, "Validation error: {}", err),
            AppError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            AppError::MalformedData(msg) => write!(f, "Malformed data: {}", msg),
            AppError::Unauthorized => write!(f, "Unauthorized access"),
            AppError::InvalidCredentials => write!(f, "Invalid credentials"),
            AppError::ExpiredToken => write!(f, "Token expired"),
            AppError::InvalidToken => write!(f, "Invalid token"),
            AppError::Forbidden => write!(f, "Forbidden"),
            AppError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            AppError::NotFound => write!(f, "Not found"),
            AppError::UserNotFound => write!(f, "User not found"),
            AppError::ResourceNotFound(res) => write!(f, "{} not found", res),
            AppError::Conflict => write!(f, "Conflict occurred"),
            AppError::UserExists => write!(f, "User already exists"),
            AppError::EmailInUse => write!(f, "Email address already in use"),
            AppError::UnprocessableEntity(details) => write!(f, "Unprocessable entity: {:?}", details),
            AppError::InternalServerError => write!(f, "Internal server error"),
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::HashingError(err) => write!(f, "Hashing error: {}", err),
            AppError::JwtError(err) => write!(f, "JWT error: {}", err),
            AppError::IoError(err) => write!(f, "IO error: {}", err),
            AppError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (code, error, message) = match self {
            // 400 Errors
            AppError::Validation(err) => (
                StatusCode::BAD_REQUEST,
                "validation_error",
                format!("Validation failed: {}", err),
            ),
            AppError::InvalidRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                msg.to_string(),
            ),
            AppError::MalformedData(msg) => (
                StatusCode::BAD_REQUEST,
                "malformed_data",
                msg.to_string(),
            ),

            // 401 Errors
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Authentication required".to_string(),
            ),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "invalid_credentials",
                "Invalid username or password".to_string(),
            ),
            AppError::ExpiredToken => (
                StatusCode::UNAUTHORIZED,
                "expired_token",
                "The provided token has expired".to_string(),
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "invalid_token",
                "Invalid authentication token".to_string(),
            ),

            // 403 Errors
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "forbidden",
                "Access to this resource is denied".to_string(),
            ),
            AppError::InsufficientPermissions => (
                StatusCode::FORBIDDEN,
                "insufficient_permissions",
                "You don't have permission to perform this action".to_string(),
            ),

            // 404 Errors
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "not_found",
                "The requested resource was not found".to_string(),
            ),
            AppError::UserNotFound => (
                StatusCode::NOT_FOUND,
                "user_not_found",
                "No user exists with the provided credentials".to_string(),
            ),
            AppError::ResourceNotFound(res) => (
                StatusCode::NOT_FOUND,
                "resource_not_found",
                format!("{} not found", res),
            ),

            // 409 Errors
            AppError::Conflict => (
                StatusCode::CONFLICT,
                "conflict",
                "A conflict occurred while processing your request".to_string(),
            ),
            AppError::UserExists => (
                StatusCode::CONFLICT,
                "user_exists",
                "A user with this email already exists".to_string(),
            ),
            AppError::EmailInUse => (
                StatusCode::CONFLICT,
                "email_in_use",
                "This email address is already registered".to_string(),
            ),

            // 422 Errors
            AppError::UnprocessableEntity(details) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "unprocessable_entity",
                "The request was well-formed but contains semantic errors".to_string(),
            ),

            // 500 Errors
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error",
                "An unexpected error occurred".to_string(),
            ),
        };

        let mut response = ErrorResponse {
            code: code.as_u16(),
            error: error.to_string(),
            message,
            details: None,
        };

        // Add details for validation errors
        if let AppError::Validation(err) = self {
            response.details = Some(
                err.field_errors()
                    .values()
                    .flat_map(|errors| errors.iter().map(|e| e.to_string()))
                    .collect(),
            );
        }

        // Add details for unprocessable entities
        if let AppError::UnprocessableEntity(details) = self {
            response.details = Some(details.clone());
        }

        HttpResponse::build(code).json(response)
    }
}

// Error conversions
impl From<PgError> for AppError {
    fn from(err: PgError) -> Self {
        match err.code() {
            Some(code) if code == &tokio_postgres::error::SqlState::UNIQUE_VIOLATION => {
                AppError::UserExists
            }
            Some(code) if code == &tokio_postgres::error::SqlState::NO_DATA_FOUND => {
                AppError::UserNotFound
            }
            _ => AppError::DatabaseError(err),
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::Validation(err)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::HashingError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::ExpiredToken,
            _ => AppError::JwtError(err),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl FieldError {
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}