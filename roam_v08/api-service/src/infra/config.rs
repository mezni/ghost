// src/infra/config.rs

use super::logger::Logger; // Relative import for logger
use crate::errors::AppError;
use config::{Config, Environment}; // From the 'config' crate
use dotenvy::dotenv; // For loading .env files
use serde::{Deserialize, Deserializer}; // For deserializing configuration

// Constants for environment variable prefixes
const SRV_PREFIX: &str = "API_SRV";
const DB_PREFIX: &str = "ROAM_DB";

/// Represents the overall server configuration, combining service and database settings.
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub service: ServiceConfig,
    pub database: DatabaseConfig,
}

/// Configuration specific to the application service (host, port, CORS).
#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: i32,

    // Custom deserialization for comma-separated CORS origins
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub cors: Vec<String>,
}

/// Helper function to deserialize a comma-separated string into a vector of strings.
/// This handles cases like "http://a.com, http://b.com" -> ["http://a.com", "http://b.com"]
fn deserialize_comma_separated<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',')
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty()) // Filter out empty strings that might result from extra commas
        .collect())
}

/// Configuration specific to the database connection.
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

/// Loads the application configuration from environment variables and .env file.
///
/// Returns `Ok(ServerConfig)` if successful, or `Err(AppError)` if configuration fails.
pub fn load_config() -> Result<ServerConfig, AppError> {
    // Return type now AppError
    // Attempt to load environment variables from a .env file.
    // .ok() converts the Result into an Option, discarding the error if the file is not found.
    dotenv().ok();

    // Load service configuration from environment variables prefixed with SRV_PREFIX (e.g., API_SRV_HOST)
    let service = Config::builder()
        .add_source(
            Environment::with_prefix(SRV_PREFIX)
                .separator("_") // Environment variable key separator (e.g., API_SRV_HOST)
                .list_separator(","), // How to split list values in env vars (e.g., API_SRV_CORS="a,b,c")
        )
        .build()? // Propagate config::ConfigError (auto-converted to AppError::ConfigError by #[from])
        .try_deserialize::<ServiceConfig>()?; // Propagate deserialization errors

    Logger::debug(&format!("Loaded service config: {:?}", service));

    // Load database configuration from environment variables prefixed with DB_PREFIX (e.g., ROAM_DB_HOST)
    let database = Config::builder()
        .add_source(Environment::with_prefix(DB_PREFIX).separator("_"))
        .build()? // Propagate config::ConfigError
        .try_deserialize::<DatabaseConfig>()?; // Propagate deserialization errors

    Logger::debug(&format!("Loaded database config: {:?}", database));

    Logger::info("Configuration loaded successfully");
    Ok(ServerConfig { service, database })
}
