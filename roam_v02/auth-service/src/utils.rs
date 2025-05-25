use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;
use crate::{
    errors::AppError,
    models::{Claims, Role},
};

// Password utilities
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::HashingError(e))?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| AppError::HashingError(e))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// JWT utilities
pub fn generate_jwt(
    user_id: Uuid,
    role: Role,
    jwt_secret: &[u8],
    expires_in: Duration,
) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(expires_in)
        .ok_or(AppError::InternalServerError)?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        role,
        refresh: false,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret),
    )
    .map_err(|e| AppError::JwtError(e))
}

pub fn generate_refresh_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

pub fn verify_jwt(token: &str, jwt_secret: &[u8]) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::JwtError(e))
}

// Random string generation
pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

// Email validation
pub fn is_valid_email(email: &str) -> bool {
    use regex::Regex;
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
        ).unwrap();
    }
    RE.is_match(email)
}

// Token expiration calculation
pub fn get_token_expiration(hours: i64) -> DateTime<Utc> {
    Utc::now() + Duration::hours(hours)
}

// Password strength validation
pub fn is_strong_password(password: &str) -> bool {
    password.len() >= 8 &&
    password.chars().any(|c| c.is_ascii_uppercase()) &&
    password.chars().any(|c| c.is_ascii_lowercase()) &&
    password.chars().any(|c| c.is_ascii_digit())
}

// URL-safe base64 encoding
pub fn base64_url_encode(input: &[u8]) -> String {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    URL_SAFE_NO_PAD.encode(input)
}

// URL-safe base64 decoding
pub fn base64_url_decode(input: &str) -> Result<Vec<u8>, AppError> {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    URL_SAFE_NO_PAD.decode(input).map_err(|_| AppError::InvalidRequest("Invalid base64 input".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "securePassword123!";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrongpassword", &hash).unwrap());
    }

    #[test]
    fn test_jwt_generation() {
        let secret = b"secret";
        let user_id = Uuid::new_v4();
        let token = generate_jwt(user_id, Role::User, secret, Duration::hours(1)).unwrap();
        let claims = verify_jwt(&token, secret).unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid-email"));
    }

    #[test]
    fn test_password_strength() {
        assert!(is_strong_password("StrongPass1"));
        assert!(!is_strong_password("weak"));
        assert!(!is_strong_password("nouppercase1"));
        assert!(!is_strong_password("NOLOWERCASE1"));
        assert!(!is_strong_password("NoNumbers"));
    }
}