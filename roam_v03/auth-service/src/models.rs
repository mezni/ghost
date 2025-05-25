// models.rs
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: NaiveDateTime,
}

// DTOs (Data Transfer Objects)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}