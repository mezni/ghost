use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);
        let keycloak_url = env::var("KEYCLOAK_URL")
            .expect("KEYCLOAK_URL must be set");
        let keycloak_realm = env::var("KEYCLOAK_REALM")
            .expect("KEYCLOAK_REALM must be set");
        let keycloak_client_id = env::var("KEYCLOAK_CLIENT_ID")
            .expect("KEYCLOAK_CLIENT_ID must be set");
        let keycloak_client_secret = env::var("KEYCLOAK_CLIENT_SECRET")
            .expect("KEYCLOAK_CLIENT_SECRET must be set");
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        Ok(Config {
            database_url,
            host,
            port,
            keycloak_url,
            keycloak_realm,
            keycloak_client_id,
            keycloak_client_secret,
            jwt_secret,
        })
    }
}