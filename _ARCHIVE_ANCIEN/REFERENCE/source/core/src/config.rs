use crate::errors::AppError;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl ServerConfig {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.dbname.is_none() {
            return Err(AppError::MissingEnvVar("DB_NAME".into()));
        }
        if self.user.is_none() {
            return Err(AppError::MissingEnvVar("DB_USER".into()));
        }
        if self.password.is_none() {
            return Err(AppError::MissingEnvVar("DB_PASSWORD".into()));
        }
        if self.host.is_none() {
            return Err(AppError::MissingEnvVar("DB_HOST".into()));
        }
        Ok(())
    }
}

pub fn read_srv_config() -> Result<ServerConfig, AppError> {
    let env_path = Path::new(".env");

    if !env_path.exists() {
        return Err(AppError::FileNotFound(".env".to_string()));
    }

    if let Err(err) = dotenv() {
        return Err(AppError::Unexpected(format!("{}", err)));
    }

    let mut cfg = ServerConfig::new();

    cfg.dbname = Some(env::var("DB_NAME").map_err(|_| AppError::MissingEnvVar("DB_NAME".into()))?);
    cfg.user = Some(env::var("DB_USER").map_err(|_| AppError::MissingEnvVar("DB_USER".into()))?);
    cfg.password =
        Some(env::var("DB_PASSWORD").map_err(|_| AppError::MissingEnvVar("DB_PASSWORD".into()))?);
    cfg.host = Some(env::var("DB_HOST").map_err(|_| AppError::MissingEnvVar("DB_HOST".into()))?);

    Ok(cfg)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub sources: Vec<Source>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Source {
    pub source_type: String,
    pub source_directory: String,
    pub file_pattern: Option<String>,
    pub post_action: Option<String>,
    pub archive_directory: Option<String>,
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.sources.is_empty() {
            return Err(AppError::InvalidConfig(
                "No sources defined in config file".into(),
            ));
        }

        for (i, source) in self.sources.iter().enumerate() {
            if source.source_type.is_empty() {
                return Err(AppError::InvalidConfig(format!(
                    "Source at index {} has empty `source_type`",
                    i
                )));
            }

            if source.source_directory.is_empty() {
                return Err(AppError::InvalidConfig(format!(
                    "Source at index {} has empty `source_directory`",
                    i
                )));
            }
        }

        Ok(())
    }
}

pub fn read_app_config(file_path: &str) -> Result<AppConfig, AppError> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(AppError::FileNotFound(file_path.into()));
    }

    let file = File::open(path).map_err(|e| AppError::FileReadError {
        path: file_path.into(),
        source: e,
    })?;

    let config = serde_yaml::from_reader(file).map_err(AppError::YamlError)?;
    Ok(config)
}
