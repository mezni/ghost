use crate::auth::models::{Claims, User};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::env;

pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

impl JwtConfig {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        let expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string()) // 24 hours
            .parse()
            .unwrap_or(86400);

        Self { secret, expiration }
    }
}

pub fn create_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let config = JwtConfig::new();
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(config.expiration))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let config = JwtConfig::new();
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}
