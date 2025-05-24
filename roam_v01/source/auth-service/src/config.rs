use crate::errors::AppError;
use config::Config;
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

pub fn load_config() -> Result<ServerConfig, AppError> {
    dotenv().ok();

    let db_config: DatabaseConfig = Config::builder()
        .add_source(config::Environment::with_prefix("AUTH_DB").separator("_"))
        .build()
        .map_err(|e| AppError::ConfigError(format!("Config build failed: {}", e)))?
        .try_deserialize()
        .map_err(|e| AppError::ConfigError(format!("Deserialization failed: {}", e)))?;

    Ok(ServerConfig {
        database: db_config,
    })
}
