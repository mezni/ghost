use crate::models::{User, CreateUser, UpdateUser};
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;
use crate::errors::AppError;

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateUser) -> Result<User, AppError> {
        let prefs = input.preferences.unwrap_or_else(|| json!({}));
        let rec = sqlx::query_as::<_, User>(r#"
            INSERT INTO users (username, email, keycloak_id, first_name, last_name, phone, date_of_birth, avatar_url, preferences)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
            RETURNING *
        "#)
        .bind(input.username)
        .bind(input.email)
        .bind(input.keycloak_id)
        .bind(input.first_name)
        .bind(input.last_name)
        .bind(input.phone)
        .bind(input.date_of_birth)
        .bind(input.avatar_url)
        .bind(prefs)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get(&self, id: Uuid) -> Result<User, AppError> {
        let rec = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        rec.ok_or(AppError::NotFound)
    }

    pub async fn get_by_keycloak_id(&self, keycloak_id: &str) -> Result<User, AppError> {
        let rec = sqlx::query_as::<_, User>("SELECT * FROM users WHERE keycloak_id = $1")
            .bind(keycloak_id)
            .fetch_optional(&self.pool)
            .await?;
        rec.ok_or(AppError::NotFound)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        let recs = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(recs)
    }

    pub async fn update(&self, id: Uuid, input: UpdateUser) -> Result<User, AppError> {
        // Partial update: get existing, then apply changes
        let mut user = self.get(id).await?;

        if let Some(v) = input.username { user.username = v; }
        if let Some(v) = input.email { user.email = v; }
        if let Some(v) = input.first_name { user.first_name = Some(v); }
        if let Some(v) = input.last_name { user.last_name = Some(v); }
        if let Some(v) = input.phone { user.phone = Some(v); }
        if let Some(v) = input.date_of_birth { user.date_of_birth = Some(v); }
        if let Some(v) = input.avatar_url { user.avatar_url = Some(v); }
        if let Some(v) = input.preferences { user.preferences = v; }

        let rec = sqlx::query_as::<_, User>(r#"
            UPDATE users SET
                username = $1,
                email = $2,
                first_name = $3,
                last_name = $4,
                phone = $5,
                date_of_birth = $6,
                avatar_url = $7,
                preferences = $8
            WHERE id = $9
            RETURNING *
        "#)
        .bind(user.username)
        .bind(user.email)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.phone)
        .bind(user.date_of_birth)
        .bind(user.avatar_url)
        .bind(user.preferences)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let res = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if res.rows_affected() == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(())
        }
    }
}
