use crate::errors::AppError;
use crate::models::{Session, User};
use chrono::NaiveDateTime;
use deadpool_postgres::Client;
use tokio_postgres::Row;

pub struct UserRepository<'a> {
    pub client: &'a Client,
}

pub struct SessionRepository<'a> {
    pub client: &'a Client,
}

impl<'a> UserRepository<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn get_by_id(&self, user_id: i32) -> Result<User, AppError> {
        let row = self
            .client
            .query_one("SELECT * FROM users WHERE id = $1", &[&user_id])
            .await?;
        Ok(Self::row_to_user(&row))
    }

    pub async fn get_by_username(&self, username: &str) -> Result<User, AppError> {
        let row = self
            .client
            .query_one("SELECT * FROM users WHERE username = $1", &[&username])
            .await?;
        Ok(Self::row_to_user(&row))
    }

    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let row = self.client
            .query_one(
                "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
                &[&username, &email, &password_hash],
            )
            .await?;
        Ok(Self::row_to_user(&row))
    }

    pub async fn update(
        &self,
        user_id: i32,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let row = self.client
            .query_one(
                "UPDATE users SET email = $1, password_hash = $2, updated_at = NOW() WHERE id = $3 RETURNING *",
                &[&email, &password_hash, &user_id],
            )
            .await?;
        Ok(Self::row_to_user(&row))
    }

    fn row_to_user(row: &Row) -> User {
        User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl<'a> SessionRepository<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        user_id: i32,
        token: &str,
        expires_at: NaiveDateTime,
    ) -> Result<Session, AppError> {
        let row = self
            .client
            .query_one(
                "INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING *",
                &[&user_id, &token, &expires_at],
            )
            .await?;
        Ok(Self::row_to_session(&row))
    }

    pub async fn delete_by_token(&self, token: &str) -> Result<(), AppError> {
        self.client
            .execute("DELETE FROM sessions WHERE token = $1", &[&token])
            .await?;
        Ok(())
    }

    pub async fn get_by_token(&self, token: &str) -> Result<Session, AppError> {
        let row = self
            .client
            .query_one("SELECT * FROM sessions WHERE token = $1", &[&token])
            .await?;
        Ok(Self::row_to_session(&row))
    }

    fn row_to_session(row: &Row) -> Session {
        Session {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token: row.get("token"),
            expires_at: row.get("expires_at"),
            created_at: row.get("created_at"),
        }
    }
}
