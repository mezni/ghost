use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{LoginRequest, RegisterRequest, RoleType, User};
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use deadpool_postgres::tokio_postgres::Row;
use tokio_pg_mapper::FromTokioPostgresRow;

use rand::rngs::OsRng;

pub async fn create_user(pool: &DbPool, form: RegisterRequest) -> Result<User, AppError> {
    let client = pool.get().await?;

    // Generate a cryptographically secure salt using OsRng
    let salt = SaltString::generate(&mut OsRng);

    // Hash the password using Argon2 and the generated salt
    let password_hash = Argon2::default()
        .hash_password(form.password.as_bytes(), &salt)
        .map_err(|e| AppError::Hashing(format!("Password hash error: {}", e)))? // Use Hashing error variant
        .to_string();

    // Default role, adjust if RegisterRequest has role

    let default_role = RoleType::User;

    let stmt = client
        .prepare(
            "
        INSERT INTO users (name, email, password, verified, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
    ",
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let row = client
        .query_one(
            &stmt,
            &[
                &form.name,
                &form.email,
                &password_hash,
                &false,
                &default_role,
            ],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(User::from_row(row)?)
}

pub async fn authenticate_user(pool: &DbPool, form: &LoginRequest) -> Result<User, AppError> {
    let client = pool.get().await?;

    let stmt = client
        .prepare("SELECT * FROM users WHERE email = $1")
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let row = client
        .query_opt(&stmt, &[&form.email])
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let user = match row {
        Some(r) => User::from_row(r)?,
        None => return Err(AppError::Unauthorized("Invalid email or password".into())),
    };

    // Parse stored password hash
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|e| AppError::Hashing(format!("Hash parse error: {}", e)))?;

    // Verify password
    let is_valid = Argon2::default()
        .verify_password(form.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid email or password".into()));
    }

    Ok(user)
}
