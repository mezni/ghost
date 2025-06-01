use super::logger::Logger;
use config::{Config, Environment};
use dotenvy::dotenv;
use serde::{Deserialize, Deserializer};

const SRV_PREFIX: &str = "API_SRV";
const DB_PREFIX: &str = "ROAM_DB";

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub service: ServiceConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: i32,

    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub cors: Vec<String>,
}

fn deserialize_comma_separated<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',')
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .collect())
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

pub fn load_config() -> Result<ServerConfig, config::ConfigError> {
    dotenv().ok();

    let service = Config::builder()
        .add_source(
            Environment::with_prefix(SRV_PREFIX)
                .separator("_")
                .list_separator(","),
        )
        .build()?
        .try_deserialize::<ServiceConfig>()?;
    Logger::debug(&format!("Loaded service config: {:?}", service));
    let database = Config::builder()
        .add_source(Environment::with_prefix(DB_PREFIX).separator("_"))
        .build()?
        .try_deserialize::<DatabaseConfig>()?;
    Logger::debug(&format!("Loaded database config: {:?}", database));

    Logger::info("Configuration loaded successfully");
    Ok(ServerConfig { service, database })
}
