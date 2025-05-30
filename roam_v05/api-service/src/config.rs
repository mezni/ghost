use config::{Config, Environment};
use dotenvy::dotenv;
use serde::Deserialize;

/// Main application configuration struct, holding all service and database settings.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub service: ServiceConfig,
    pub database: DatabaseConfig,
}

/// Configuration for the application's service (e.g., HTTP server).
#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    pub host: String,     // e.g., "0.0.0.0"
    pub port: u16,        // e.g., 3200
    // CHANGED: 'cors' is now a nested struct (CorsConfig)
    pub cors: CorsConfig,
}

/// Nested struct for CORS-related settings.
#[derive(Debug, Deserialize)]
pub struct CorsConfig {
    // This will now map to API_SRV_CORS_ALLOWED_ORIGIN
    pub allowed_origin: Vec<String>,
}

/// Configuration for the database connection.
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,     // Database host, e.g., "localhost"
    pub port: u16,        // Database port, e.g., 5434 for PostgreSQL
    pub user: String,     // Database username
    pub password: String, // Database password
    pub name: String,     // Database name
}

/// Loads the application configuration from environment variables.
///
/// It uses `dotenvy` to load variables from a `.env` file in development,
/// and `config` to deserialize environment variables into structured configs.
///
/// Environment variables for service config should be prefixed with `API_SRV_`
/// (e.g., `API_SRV_HOST`, `API_SRV_PORT`).
///
/// Environment variables for database config should be prefixed with `ROAM_DB_`
/// (e.g., `ROAM_DB_HOST`, `ROAM_DB_USER`).
pub fn load_config() -> Result<ServerConfig, config::ConfigError> {
    // Load environment variables from a .env file if it exists.
    dotenv().ok();

    // Build and deserialize ServiceConfig from environment variables
    let service_config: ServiceConfig = Config::builder()
        // Changed prefix to "API_SRV"
        .add_source(Environment::with_prefix("API_SRV").separator("_"))
        .build()?
        .try_deserialize()?;

    // Build and deserialize DatabaseConfig from environment variables
    let db_config: DatabaseConfig = Config::builder()
        // Changed prefix to "ROAM_DB"
        .add_source(Environment::with_prefix("ROAM_DB").separator("_"))
        .build()?
        .try_deserialize()?;

    // Combine the individual configs into the main ServerConfig struct
    Ok(ServerConfig {
        service: service_config,
        database: db_config,
    })
}
