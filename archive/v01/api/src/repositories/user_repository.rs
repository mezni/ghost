use crate::errors::ApiError;
use crate::models::user::{NewUser, UpdateUser, User};
use bcrypt::{hash, verify, DEFAULT_COST};
use deadpool_postgres::Client;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: deadpool_postgres::Pool,
}

impl UserRepository {
    pub fn new(pool: deadpool_postgres::Pool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, ApiError> {
        let client = self.pool.get().await?;
        let password_hash = hash(new_user.password, DEFAULT_COST)
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
        
        let row = client.query_one(
            r#"
            INSERT INTO users (email, username, password_hash, first_name, last_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            &[
                &new_user.email,
                &new_user.username,
                &password_hash,
                &new_user.first_name,
                &new_user.last_name,
            ]
        ).await.map_err(|e| {
            if let Some(code) = e.code() {
                if code == &tokio_postgres::error::SqlState::UNIQUE_VIOLATION {
                    return ApiError::BadRequest("Email or username already exists".to_string());
                }
            }
            ApiError::InternalServerError(e.to_string())
        })?;
        
        Ok(User::from_row(&row))
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, ApiError> {
        let client = self.pool.get().await?;
        let row = client.query_one(
            "SELECT * FROM users WHERE id = $1",
            &[&user_id]
        ).await.map_err(|e| {
            if let Some(code) = e.code() {
                if code == &tokio_postgres::error::SqlState::NO_DATA_FOUND {
                    return ApiError::NotFound(format!("User {} not found", user_id));
                }
            }
            ApiError::InternalServerError(e.to_string())
        })?;
        
        Ok(User::from_row(&row))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, ApiError> {
        let client = self.pool.get().await?;
        let row = client.query_one(
            "SELECT * FROM users WHERE email = $1",
            &[&email]
        ).await.map_err(|e| {
            if let Some(code) = e.code() {
                if code == &tokio_postgres::error::SqlState::NO_DATA_FOUND {
                    return ApiError::NotFound(format!("User with email {} not found", email));
                }
            }
            ApiError::InternalServerError(e.to_string())
        })?;
        
        Ok(User::from_row(&row))
    }

    pub async fn update_user(&self, user_id: Uuid, update_data: UpdateUser) -> Result<User, ApiError> {
        let client = self.pool.get().await?;
        let row = client.query_one(
            r#"
            UPDATE users 
            SET 
                first_name = COALESCE($1, first_name),
                last_name = COALESCE($2, last_name),
                updated_at = NOW()
            WHERE id = $3
            RETURNING *
            "#,
            &[
                &update_data.first_name,
                &update_data.last_name,
                &user_id,
            ]
        ).await.map_err(|e| {
            ApiError::InternalServerError(e.to_string())
        })?;
        
        Ok(User::from_row(&row))
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), ApiError> {
        let client = self.pool.get().await?;
        let affected = client.execute(
            "DELETE FROM users WHERE id = $1",
            &[&user_id]
        ).await.map_err(|e| {
            ApiError::InternalServerError(e.to_string())
        })?;
        
        if affected == 0 {
            Err(ApiError::NotFound(format!("User {} not found", user_id)))
        } else {
            Ok(())
        }
    }

    pub async fn verify_password(&self, email: &str, password: &str) -> Result<User, ApiError> {
        let user = self.get_user_by_email(email).await?;
        
        match verify(password, &user.password_hash) {
            Ok(true) => Ok(user),
            Ok(false) => Err(ApiError::Unauthorized("Invalid password".to_string())),
            Err(e) => Err(ApiError::InternalServerError(e.to_string())),
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, ApiError> {
        let client = self.pool.get().await?;
        let rows = client.query(
            "SELECT * FROM users ORDER BY created_at DESC",
            &[]
        ).await.map_err(|e| {
            ApiError::InternalServerError(e.to_string())
        })?;
        
        Ok(rows.iter().map(User::from_row).collect())
    }
}

// Helper to convert Postgres rows to User
impl User {
    pub fn from_row(row: &Row) -> Self {
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