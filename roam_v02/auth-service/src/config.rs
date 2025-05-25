use serde::Deserialize;
use std::{net::IpAddr, path::PathBuf};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Config {
    #[validate]
    pub server: ServerConfig,
    #[validate]
    pub database: DatabaseConfig,
    #[validate]
    pub auth: AuthConfig,
    #[validate]
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ServerConfig {
    #[validate(ip)]
    pub host: IpAddr,
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    #[validate(range(min = 1, max = 32))]
    pub worker_count: usize,
    #[validate(length(min = 1))]
    pub environment: String,
    #[validate]
    pub cors: CorsConfig,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DatabaseConfig {
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    #[validate(length(min = 1))]
    pub user: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 1, max = 100))]
    pub max_connections: usize,
    pub require_ssl: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthConfig {
    #[validate(length(min = 32))]
    pub jwt_secret: String,
    #[validate(range(min = 1))]
    pub access_token_expiry_minutes: i64,
    #[validate(range(min = 1))]
    pub refresh_token_expiry_days: i64,
    #[validate(length(min = 1))]
    pub password_reset_url: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoggingConfig {
    #[validate(length(min = 1))]
    pub level: String,
    pub json: bool,
    pub file: Option<PathBuf>,
    pub enable_otel: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CorsConfig {
    #[validate(length(min = 1))]
    pub allowed_origins: Vec<String>,
    pub allow_any_origin: bool,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string());

        let config = config::Config::builder()
            // Load default configuration
            .add_source(config::File::with_name("config/default"))
            // Load environment-specific configuration
            .add_source(config::File::with_name(&format!("config/{}", environment)).required(false))
            // Add in settings from environment variables (with a prefix of APP)
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        self.validate().map_err(|errors| {
            errors
                .field_errors()
                .into_iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |e| {
                        format!("{}: {}", field, e.to_string())
                    })
                })
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = Config {
            server: ServerConfig {
                host: "127.0.0.1".parse().unwrap(),
                port: 8080,
                worker_count: 4,
                environment: "development".to_string(),
                cors: CorsConfig {
                    allowed_origins: vec!["http://localhost:3000".to_string()],
                    allow_any_origin: false,
                },
            },
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                user: "postgres".to_string(),
                password: "password".to_string(),
                name: "auth_service".to_string(),
                max_connections: 20,
                require_ssl: false,
            },
            auth: AuthConfig {
                jwt_secret: "super-secret-key-with-at-least-32-characters".to_string(),
                access_token_expiry_minutes: 60,
                refresh_token_expiry_days: 7,
                password_reset_url: "http://localhost:8080/reset-password".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                json: false,
                file: None,
                enable_otel: false,
            },
        };

        assert!(config.validate().is_ok());
    }
}