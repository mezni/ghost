use dotenvy::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use crate::errors::AppError; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenv().map_err(AppError::DotenvError)?;

        fn get_var(key: &str) -> Result<String, AppError> {
            env::var(&format!("AUTH_{}", key))
                .map_err(|_| AppError::MissingEnvVar(format!("AUTH_{}", key)))
        }

        fn parse_u16(key: &str, val: String) -> Result<u16, AppError> {
            val.parse()
                .map_err(|_| AppError::InvalidEnvVarFormat(format!("AUTH_{}", key)))
        }

        fn parse_i64(key: &str, val: String) -> Result<i64, AppError> {
            val.parse()
                .map_err(|_| AppError::InvalidEnvVarFormat(format!("AUTH_{}", key)))
        }

        let server_host = get_var("SERVER_HOST")?;
        let server_port_str = get_var("SERVER_PORT")?;
        let server_port = parse_u16("SERVER_PORT", server_port_str)?;

        let db_host = get_var("DB_HOST")?;
        let db_port_str = get_var("DB_PORT")?;
        let db_port = parse_u16("DB_PORT", db_port_str)?;
        let db_user = get_var("DB_USER")?;
        let db_password = get_var("DB_PASSWORD")?;
        let db_name = get_var("DB_NAME")?;

        let jwt_secret = get_var("JWT_SECRET")?;
        let jwt_expires_str = get_var("JWT_EXPIRES_IN")?;
        let jwt_expires_in = parse_i64("JWT_EXPIRES_IN", jwt_expires_str)?;

        Ok(Config {
            server: ServerConfig {
                host: server_host,
                port: server_port,
            },
            database: DatabaseConfig {
                host: db_host,
                port: db_port,
                username: db_user,
                password: db_password,
                database: db_name,
            },
            jwt: JwtConfig {
                secret: jwt_secret,
                expires_in: jwt_expires_in,
            },
        })
    }
}
