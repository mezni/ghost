use crate::errors::AppError;
use postgres_derive::{FromSql, ToSql}; // Required for `derive(ToSql, FromSql)`
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio_postgres::Row;

/// Role enum mapped to PostgreSQL `role_type`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "role_type")]
pub enum RoleType {
    #[postgres(name = "admin")]
    Admin,
    #[postgres(name = "user")]
    User,
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RoleType::Admin => "admin",
            RoleType::User => "user",
        };
        write!(f, "{}", s)
    }
}

/// User model representing a user in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub verified: bool,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: RoleType,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from_row(row: Row) -> Result<Self, AppError> {
        Ok(User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            verified: row.get("verified"),
            password: row.get("password"),
            role: row.get("role"),
            created_at: row.get("created_at"),
        })
    }
}

/// Request model for registering a new user
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// Request model for user login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response model for login success
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i32,
    pub role: RoleType,
}
