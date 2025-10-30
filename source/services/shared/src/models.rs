use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
    pub required_roles: Option<Vec<String>>,
    pub required_permissions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenValidationResponse {
    pub valid: bool,
    pub user: Option<User>,
    pub missing_roles: Vec<String>,
    pub missing_permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: Option<String>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub success: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            success: true,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            data: None,
            error: Some(message),
            success: false,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorResponse {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: None,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

#[derive(Debug, serde::Deserialize, Validate)]
pub struct PaginationQuery {
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<u32>,
}
