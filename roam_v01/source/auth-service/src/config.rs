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

pub fn load_config() -> ServerConfig {
    dotenv().ok(); // Load from .env into env vars

    // Load just the database config from AUTH_DB_* env vars
    let db_config: DatabaseConfig = Config::builder()
        .add_source(config::Environment::with_prefix("AUTH_DB").separator("_"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();

    ServerConfig {
        database: db_config,
    }
}
