use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, Result},
    models::UserEntity,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, username: &str, email: &str, keycloak_id: &str) -> Result<UserEntity>;
    async fn find_by_id(&self, id: Uuid) -> Result<UserEntity>;
    async fn find_by_email(&self, email: &str) -> Result<UserEntity>;
    async fn find_by_username(&self, username: &str) -> Result<UserEntity>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<UserEntity>;
    async fn list(&self, page: u32, limit: u32) -> Result<Vec<UserEntity>>;
    async fn update(
        &self,
        id: Uuid,
        update_data: &shared::models::UpdateUserRequest,
    ) -> Result<UserEntity>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn count(&self) -> Result<u64>;
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, username: &str, email: &str, keycloak_id: &str) -> Result<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            INSERT INTO users (username, email, keycloak_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            username,
            email,
            keycloak_id,
            chrono::Utc::now(),
            chrono::Utc::now()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::UserNotFound,
            _ => AppError::Database(e),
        })?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::UserNotFound,
            _ => AppError::Database(e),
        })?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT * FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::UserNotFound,
            _ => AppError::Database(e),
        })?;

        Ok(user)
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT * FROM users WHERE keycloak_id = $1
            "#,
            keycloak_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::UserNotFound,
            _ => AppError::Database(e),
        })?;

        Ok(user)
    }

    async fn list(&self, page: u32, limit: u32) -> Result<Vec<UserEntity>> {
        let offset = (page - 1) * limit;
        let users = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT * FROM users 
            ORDER BY created_at DESC 
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(users)
    }

    async fn update(
        &self,
        id: Uuid,
        update_data: &shared::models::UpdateUserRequest,
    ) -> Result<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            UPDATE users 
            SET 
                email = COALESCE($1, email),
                username = COALESCE($2, username),
                first_name = COALESCE($3, first_name),
                last_name = COALESCE($4, last_name),
                phone = COALESCE($5, phone),
                updated_at = $6
            WHERE id = $7
            RETURNING *
            "#,
            update_data.email,
            update_data.username,
            update_data.first_name,
            update_data.last_name,
            update_data.phone,
            chrono::Utc::now(),
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn count(&self) -> Result<u64> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM users
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(count.count.unwrap_or(0) as u64)
    }
}
