use crate::core::db::Db;
use crate::core::errors::AppError;
use crate::users::models::{CreateUser, User};
use bcrypt::{DEFAULT_COST, hash};
use chrono::Utc;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    /// Create a new user
    pub async fn create(pool: &Pool, user: CreateUser) -> Result<User, AppError> {
        let client = pool.get().await?;

        // Hash the password
        let password_hash = hash(&user.password, DEFAULT_COST)
            .map_err(|e| AppError::Other(format!("Password hashing error: {}", e)))?;

        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();

        let stmt = "
            INSERT INTO users (
                id, username, email, password_hash, first_name, last_name, is_valid, is_admin, created_at, updated_at
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            RETURNING id, username, email, password_hash, first_name, last_name, is_valid, is_admin, created_at, updated_at
        ";

        let row = client
            .query_one(
                stmt,
                &[
                    &id,
                    &user.username,
                    &user.email,
                    &password_hash,
                    &user.first_name,
                    &user.last_name,
                    &true,  // is_valid default
                    &false, // is_admin default
                    &now,
                    &now,
                ],
            )
            .await?;

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_valid: row.get("is_valid"),
            is_admin: row.get("is_admin"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// Get user by ID
    pub async fn get_by_id(pool: &Pool, user_id: Uuid) -> Result<Option<User>, AppError> {
        let client = pool.get().await?;
        let stmt = "SELECT * FROM users WHERE id = $1";
        let row_opt = client.query_opt(stmt, &[&user_id]).await?;

        if let Some(row) = row_opt {
            Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                is_valid: row.get("is_valid"),
                is_admin: row.get("is_admin"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get user by email (useful for login)
    pub async fn get_by_email(pool: &Pool, email: &str) -> Result<Option<User>, AppError> {
        let client = pool.get().await?;
        let stmt = "SELECT * FROM users WHERE email = $1";
        let row_opt = client.query_opt(stmt, &[&email]).await?;

        if let Some(row) = row_opt {
            Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                is_valid: row.get("is_valid"),
                is_admin: row.get("is_admin"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update a user
    pub async fn update(pool: &Pool, user: &User) -> Result<User, AppError> {
        let client = pool.get().await?;
        let now = Utc::now().naive_utc();

        let stmt = "
            UPDATE users SET
                username = $1,
                email = $2,
                password_hash = $3,
                first_name = $4,
                last_name = $5,
                is_valid = $6,
                is_admin = $7,
                updated_at = $8
            WHERE id = $9
            RETURNING *
        ";

        let row = client
            .query_one(
                stmt,
                &[
                    &user.username,
                    &user.email,
                    &user.password_hash,
                    &user.first_name,
                    &user.last_name,
                    &user.is_valid,
                    &user.is_admin,
                    &now,
                    &user.id,
                ],
            )
            .await?;

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_valid: row.get("is_valid"),
            is_admin: row.get("is_admin"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// Delete a user
    pub async fn delete(pool: &Pool, user_id: Uuid) -> Result<(), AppError> {
        let client = pool.get().await?;
        client
            .execute("DELETE FROM users WHERE id = $1", &[&user_id])
            .await?;
        Ok(())
    }
}
