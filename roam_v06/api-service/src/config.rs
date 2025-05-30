use config::{Config, Environment};
use dotenvy::dotenv;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub service: ServiceConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

pub fn load_config() -> Result<ServerConfig, config::ConfigError> {
    dotenv().ok();

    println!(
        "DEBUG: Raw env var API_SRV_CORS: {:?}",
        std::env::var("API_SRV_CORS")
    );

    let service = Config::builder()
        .add_source(
            Environment::with_prefix("API_SRV")
                .separator("_")
                .list_separator(","),
        )
        .build()?
        .try_deserialize::<ServiceConfig>()?;

    let database = Config::builder()
        .add_source(Environment::with_prefix("ROAM_DB").separator("_"))
        .build()?
        .try_deserialize::<DatabaseConfig>()?;

    Ok(ServerConfig { service, database })
}
