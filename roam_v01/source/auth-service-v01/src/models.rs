use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum RoleType {
    Admin,
    User,
}

impl From<&str> for RoleType {
    fn from(role: &str) -> Self {
        match role.to_lowercase().as_str() {
            "admin" => RoleType::Admin,
            _ => RoleType::User,
        }
    }
}

impl From<i32> for RoleType {
    fn from(role_id: i32) -> Self {
        match role_id {
            1 => RoleType::Admin,
            _ => RoleType::User,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub verified: bool,
    pub role: RoleType,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}