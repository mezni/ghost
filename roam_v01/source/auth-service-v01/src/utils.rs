use argon2::{
    Argon2,
    PasswordHash,
    PasswordVerifier,
    PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng, Error as ArgonError}
};
use anyhow::{Context, Result};
use crate::AppError;

pub fn hash_password(password: &str) -> Result<String, ArgonError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, ArgonError> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_token(user_id: i32) -> String {
    use uuid::Uuid;
    use base64::{Engine as _, engine::general_purpose};
    
    let uuid = Uuid::new_v4();
    let combined = format!("{}:{}", user_id, uuid);
    general_purpose::URL_SAFE_NO_PAD.encode(combined.as_bytes())
}

pub fn validate_token(token: &str) -> Option<i32> {
    use base64::{Engine as _, engine::general_purpose};
    
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(token).ok()?;
    let decoded_str = String::from_utf8(decoded).ok()?;
    let mut parts = decoded_str.split(':');
    let user_id = parts.next()?.parse::<i32>().ok()?;
    Some(user_id)
}