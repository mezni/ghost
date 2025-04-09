use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub dbname: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig {
            dbname: None,
            user: None,
            password: None,
            host: None,
        }
    }
}


pub fn read_srv_config() -> Result<ServerConfig, AppError> {
    // Load environment variables from the .env file.
    if let Err(err) = dotenv() {
        return Err(AppError::Unexpected(format!(
            "Failed to load .env file: {}",
            err
        )));
    }

    let mut cfg = ServerConfig::new();

    cfg.dbname = Some(env::var("DB_NAME").map_err(|_| AppError::MissingEnvVar("DB_NAME".into()))?);
    cfg.user = Some(env::var("DB_USER").map_err(|_| AppError::MissingEnvVar("DB_USER".into()))?);
    cfg.password =
        Some(env::var("DB_PASSWORD").map_err(|_| AppError::MissingEnvVar("DB_PASSWORD".into()))?);
    cfg.host = Some(env::var("DB_HOST").map_err(|_| AppError::MissingEnvVar("DB_HOST".into()))?);

    Ok(cfg)
}