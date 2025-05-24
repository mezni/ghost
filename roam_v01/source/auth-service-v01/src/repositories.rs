use crate::{models::*, utils::hash_password, AppError};
use tokio_postgres::{Client, Row};
use argon2::password_hash::Error as ArgonError;

pub struct UserRepository {
    client: Client,
}

impl UserRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_user(&self, user: &RegisterRequest) -> Result<UserResponse, AppError> {
        let hashed_pw = hash_password(&user.password)
            .map_err(|e| AppError::HashError(e))?;

        let row = self.client.query_one(
            "INSERT INTO users (name, email, password, role_id) VALUES ($1, $2, $3, 2) RETURNING id, name, email, verified, role_id, created_at",
            &[&user.name, &user.email, &hashed_pw]
        ).await
        .map_err(|e| AppError::DBError(e.to_string()))?;

        Ok(Self::row_to_user_response(row))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<(i32, String)>, AppError> {
        let row = self.client.query_opt(
            "SELECT id, password FROM users WHERE email = $1",
            &[&email]
        ).await
        .map_err(|e| AppError::DBError(e.to_string()))?;

        row.map(|r| Ok((r.get("id"), r.get("password")))).transpose()
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<UserResponse>, AppError> {
        let row = self.client.query_opt(
            "SELECT id, name, email, verified, role_id, created_at FROM users WHERE id = $1",
            &[&id]
        ).await
        .map_err(|e| AppError::DBError(e.to_string()))?;

        Ok(row.map(Self::row_to_user_response))
    }

    pub async fn update_user(&self, id: i32, update: &UpdateUserRequest) -> Result<UserResponse, AppError> {
        let mut updates = Vec::new();
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_index = 1;

        if let Some(name) = &update.name {
            updates.push(format!("name = ${}", param_index));
            params.push(name);
            param_index += 1;
        }

        if let Some(email) = &update.email {
            updates.push(format!("email = ${}", param_index));
            params.push(email);
            param_index += 1;
        }

        if let Some(password) = &update.password {
            let hashed_pw = hash_password(password)
                .map_err(|e| AppError::HashError(e))?;
            updates.push(format!("password = ${}", param_index));
            params.push(&hashed_pw);
            param_index += 1;
        }

        if updates.is_empty() {
            return Err(AppError::Other("No fields to update".to_string()));
        }

        params.push(&id);

        let query = format!(
            "UPDATE users SET {} WHERE id = ${} RETURNING id, name, email, verified, role_id, created_at",
            updates.join(", "),
            param_index
        );

        let row = self.client.query_one(&query, &params)
            .await
            .map_err(|e| AppError::DBError(e.to_string()))?;

        Ok(Self::row_to_user_response(row))
    }

    fn row_to_user_response(row: Row) -> UserResponse {
        UserResponse {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            verified: row.get("verified"),
            role: RoleType::from(row.get::<_, i32>("role_id")),
            created_at: row.get("created_at"),
        }
    }
}