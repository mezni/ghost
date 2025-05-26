use crate::errors::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut thread_rng())
}

pub fn hash_password(password: &str, salt: &SaltString) -> Result<String, AppError> {
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), salt)
        .map_err(|e| AppError::HashError(format!("Password hashing error: {}", e)))?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::HashError(format!("Invalid password hash: {}", e)))?;

    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(
    secret: &str,
    user_id: i32,
    expires_in_seconds: i64,
) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(expires_in_seconds))
        .ok_or_else(|| AppError::JwtError("Failed to calculate expiration time".into()))?
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::JwtError(format!("JWT generation error: {}", e)))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|e| AppError::JwtError(format!("JWT verification error: {}", e)))?;

    Ok(token_data.claims)
}
