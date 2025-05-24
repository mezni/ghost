use config::Config;
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

pub fn load_config() -> ServerConfig {
    dotenv().ok();

    Config::builder()
        .add_source(config::Environment::default().separator("_"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
}
