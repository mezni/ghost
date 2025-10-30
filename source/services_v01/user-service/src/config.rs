use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
    pub keycloak_admin_user: String,
    pub keycloak_admin_password: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),
    #[error("Invalid port: {0}")]
    InvalidPort(String),
}

impl Config {
    pub fn from_env() -> Result<Self, crate::errors::AppError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/users".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .map_err(|_| crate::errors::AppError::Config("Invalid PORT".to_string()))?,
            keycloak_url: std::env::var("KEYCLOAK_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            keycloak_realm: std::env::var("KEYCLOAK_REALM")
                .unwrap_or_else(|_| "master".to_string()),
            keycloak_client_id: std::env::var("KEYCLOAK_CLIENT_ID")
                .unwrap_or_else(|_| "user-service".to_string()),
            keycloak_client_secret: std::env::var("KEYCLOAK_CLIENT_SECRET")
                .unwrap_or_else(|_| "".to_string()),
            keycloak_admin_user: std::env::var("KEYCLOAK_ADMIN_USER")
                .unwrap_or_else(|_| "admin".to_string()),
            keycloak_admin_password: std::env::var("KEYCLOAK_ADMIN_PASSWORD")
                .unwrap_or_else(|_| "admin".to_string()),
        })
    }
}
