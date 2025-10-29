use sqlx::PgPool;
use chrono::Utc;

use crate::core::error::{Result, AppError};
use crate::user_model::{User, CreateUserRequest, UpdateUserRequest};

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user_data: CreateUserRequest) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, first_name, last_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, username, email, first_name, last_name, keycloak_id, is_active, created_at, updated_at
            "#
        )
        .bind(&user_data.username)
        .bind(&user_data.email)
        .bind(&user_data.first_name)
        .bind(&user_data.last_name)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User> {
        // Parse string to Uuid for database query
        let uuid = user_id.parse::<sqlx::types::Uuid>()
            .map_err(|_| AppError::InvalidInput("Invalid user ID format".to_string()))?;
            
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, first_name, last_name, keycloak_id, is_active, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, first_name, last_name, keycloak_id, is_active, created_at, updated_at FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, username, email, first_name, last_name, keycloak_id, is_active, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn update_user(&self, user_id: &str, user_data: UpdateUserRequest) -> Result<User> {
        // Parse string to Uuid for database query
        let uuid = user_id.parse::<sqlx::types::Uuid>()
            .map_err(|_| AppError::InvalidInput("Invalid user ID format".to_string()))?;
            
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users 
            SET 
                email = COALESCE($1, email),
                first_name = COALESCE($2, first_name),
                last_name = COALESCE($3, last_name),
                is_active = COALESCE($4, is_active),
                updated_at = $5
            WHERE id = $6
            RETURNING id, username, email, first_name, last_name, keycloak_id, is_active, created_at, updated_at
            "#
        )
        .bind(user_data.email)
        .bind(user_data.first_name)
        .bind(user_data.last_name)
        .bind(user_data.is_active)
        .bind(Utc::now())
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        // Parse string to Uuid for database query
        let uuid = user_id.parse::<sqlx::types::Uuid>()
            .map_err(|_| AppError::InvalidInput("Invalid user ID format".to_string()))?;
            
        let result = sqlx::query(
            "DELETE FROM users WHERE id = $1"
        )
        .bind(uuid)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::UserNotFound);
        }

        Ok(())
    }

    pub async fn link_keycloak_id(&self, user_id: &str, keycloak_id: &str) -> Result<()> {
        // Parse string to Uuid for database query
        let uuid = user_id.parse::<sqlx::types::Uuid>()
            .map_err(|_| AppError::InvalidInput("Invalid user ID format".to_string()))?;
            
        sqlx::query(
            "UPDATE users SET keycloak_id = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(keycloak_id)
        .bind(Utc::now())
        .bind(uuid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}