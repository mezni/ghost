use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Database entity
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub keycloak_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub avatar_url: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserEntity> for shared::models::User {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            username: entity.username,
            email: entity.email,
            roles: vec!["user".to_string()], // This would come from Keycloak
            created_at: entity.created_at,
        }
    }
}

// Service-specific models that extend shared models
#[derive(Debug, Deserialize, Validate)]
pub struct GetUsersQuery {
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<u32>,
}

// Use shared UpdateUserRequest directly
pub type UpdateProfileRequest = shared::models::UpdateUserRequest;

// Response models
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<shared::models::User>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct DeleteUserResponse {
    pub message: String,
    pub user_id: Uuid,
}
