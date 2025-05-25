use crate::{
    errors::AppError,
    models::*,
    utils::{hash_password, verify_password},
};
use deadpool_postgres::Client;
use tokio_postgres::{Row, Transaction};
use uuid::Uuid;

/// User repository handles all database operations for users
pub struct UserRepository;

impl UserRepository {
    /// Create a new user
    pub async fn create(
        client: &Client,
        request: &CreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        let password_hash = hash_password(&request.password)?;
        let role = request.role.as_deref().map(Role::from).unwrap_or(Role::User);

        let row = client
            .query_one(
                r#"
                INSERT INTO users (email, password_hash, role)
                VALUES ($1, $2, $3)
                RETURNING id, email, role, email_verified, created_at
                "#,
                &[&request.email, &password_hash, &role.to_string()],
            )
            .await?;

        Ok(UserResponse::from_row(&row)?)
    }

    /// Find user by email
    pub async fn find_by_email(client: &Client, email: &str) -> Result<Option<User>, AppError> {
        let row = client
            .query_opt(
                "SELECT * FROM users WHERE email = $1",
                &[&email],
            )
            .await?;

        row.map(|r| User::from_row(&r)).transpose()
            .map_err(|e| AppError::MalformedData(e))
    }

    /// Get user by ID
    pub async fn find_by_id(client: &Client, user_id: Uuid) -> Result<Option<User>, AppError> {
        let row = client
            .query_opt(
                "SELECT * FROM users WHERE id = $1",
                &[&user_id],
            )
            .await?;

        row.map(|r| User::from_row(&r)).transpose()
            .map_err(|e| AppError::MalformedData(e))
    }

    /// Verify user credentials
    pub async fn verify_credentials(
        client: &Client,
        email: &str,
        password: &str,
    ) -> Result<User, AppError> {
        let user = Self::find_by_email(client, email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        if !verify_password(password, &user.password_hash)? {
            return Err(AppError::InvalidCredentials);
        }

        Ok(user)
    }

    /// Update user password
    pub async fn update_password(
        client: &Client,
        user_id: Uuid,
        new_password: &str,
    ) -> Result<(), AppError> {
        let password_hash = hash_password(new_password)?;

        client
            .execute(
                "UPDATE users SET password_hash = $1 WHERE id = $2",
                &[&password_hash, &user_id],
            )
            .await?;

        Ok(())
    }

    /// Mark email as verified
    pub async fn verify_email(client: &Client, user_id: Uuid) -> Result<(), AppError> {
        client
            .execute(
                "UPDATE users SET email_verified = true WHERE id = $1",
                &[&user_id],
            )
            .await?;

        Ok(())
    }

    /// Create refresh token
    pub async fn create_refresh_token(
        tx: &Transaction<'_>,
        user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), AppError> {
        tx.execute(
            r#"
            INSERT INTO refresh_tokens (token, user_id, expires_at)
            VALUES ($1, $2, $3)
            "#,
            &[&token, &user_id, &expires_at],
        )
        .await?;

        Ok(())
    }

    /// Find valid refresh token
    pub async fn find_refresh_token(
        client: &Client,
        token: &str,
    ) -> Result<Option<RefreshToken>, AppError> {
        let row = client
            .query_opt(
                r#"
                SELECT * FROM refresh_tokens
                WHERE token = $1 AND expires_at > NOW()
                "#,
                &[&token],
            )
            .await?;

        row.map(|r| RefreshToken::from_row(&r)).transpose()
            .map_err(|e| AppError::MalformedData(e))
    }

    /// Delete refresh token
    pub async fn delete_refresh_token(client: &Client, token: &str) -> Result<(), AppError> {
        client
            .execute(
                "DELETE FROM refresh_tokens WHERE token = $1",
                &[&token],
            )
            .await?;

        Ok(())
    }

    /// Delete all refresh tokens for a user
    pub async fn delete_user_refresh_tokens(
        client: &Client,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        client
            .execute(
                "DELETE FROM refresh_tokens WHERE user_id = $1",
                &[&user_id],
            )
            .await?;

        Ok(())
    }
}

// Row conversion implementations
impl UserResponse {
    pub fn from_row(row: &Row) -> Result<Self, String> {
        Ok(Self {
            id: row.get("id"),
            email: row.get("email"),
            role: Role::from(row.get::<_, String>("role").as_str()),
            email_verified: row.get("email_verified"),
            created_at: row.get("created_at"),
        })
    }
}