use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.to_rfc3339()
}

pub fn parse_timestamp(timestamp_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(timestamp_str).map(|dt| dt.with_timezone(&Utc))
}

pub fn extract_bearer_token(auth_header: &str) -> Option<String> {
    if auth_header.starts_with(crate::constants::BEARER_PREFIX) {
        Some(auth_header[crate::constants::BEARER_PREFIX.len()..].to_string())
    } else {
        None
    }
}

pub fn validate_email(email: &str) -> bool {
    // Simple email validation - you can use a more comprehensive crate if needed
    email.contains('@') && email.contains('.') && email.len() > 5
}

pub fn validate_password_strength(password: &str) -> bool {
    password.len() >= crate::constants::MIN_PASSWORD_LENGTH
        && password.len() <= crate::constants::MAX_PASSWORD_LENGTH
}

#[cfg(feature = "hashing")]
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

#[cfg(feature = "hashing")]
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

#[cfg(not(feature = "hashing"))]
pub fn hash_password(_password: &str) -> Result<String, Box<dyn std::error::Error>> {
    Err("bcrypt feature not enabled".into())
}

#[cfg(not(feature = "hashing"))]
pub fn verify_password(_password: &str, _hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
    Err("bcrypt feature not enabled".into())
}
