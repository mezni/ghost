use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct Auth;

impl Auth {
    pub fn create_jwt(email: &str) -> String {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret".into());
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .unwrap()
            .timestamp() as usize;

        let claims = Claims {
            sub: email.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }

    pub fn validate_jwt(token: &str) -> Option<Claims> {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret".into());
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .ok()
        .map(|data| data.claims)
    }
}
