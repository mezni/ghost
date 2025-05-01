use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn from_row(row: &postgres::Row) -> Self {
        User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_active: row.get("is_active"),
            is_verified: row.get("is_verified"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    #[validate(length(max = 100))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100))]
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(max = 100))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100))]
    pub last_name: Option<String>,
}