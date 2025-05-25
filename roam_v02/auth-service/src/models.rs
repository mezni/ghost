use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;
use validator::Validate;

/// User role with permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Admin,
    User,
    Custom(String),
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match role.to_lowercase().as_str() {
            "admin" => Role::Admin,
            "user" => Role::User,
            custom => Role::Custom(custom.to_string()),
        }
    }
}

/// Core user model representing database table
#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Role,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User creation request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    #[validate(length(min = 1))]
    pub role: Option<String>,
}

/// User response (safe for API responses)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
}

/// Login request
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

/// Successful login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
    pub expires_in: i64,
}

/// Password reset request
#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(email)]
    pub email: String,
}

/// New password submission
#[derive(Debug, Deserialize, Validate)]
pub struct NewPasswordRequest {
    #[validate(length(min = 8))]
    pub new_password: String,
    pub token: String,
}

/// Email verification request
#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

// Database row conversions
impl User {
    /// Converts a PostgreSQL row to a User
    pub fn from_row(row: &Row) -> Result<Self, String> {
        Ok(Self {
            id: row.get("id"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            role: Role::from(row.get::<_, String>("role").as_str()),
            email_verified: row.get("email_verified"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            email_verified: user.email_verified,
            created_at: user.created_at,
        }
    }
}

/// Database model for refresh tokens
#[derive(Debug)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn from_row(row: &Row) -> Result<Self, String> {
        Ok(Self {
            token: row.get("token"),
            user_id: row.get("user_id"),
            expires_at: row.get("expires_at"),
            created_at: row.get("created_at"),
        })
    }
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,         // user ID
    pub exp: usize,        // expiry timestamp
    pub role: Role,        // user role
    pub refresh: bool,     // if this is a refresh token
}

/// Pagination parameters
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

/// User update request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,
    
    #[validate(length(min = 8))]
    pub password: Option<String>,
    
    pub role: Option<String>,
}