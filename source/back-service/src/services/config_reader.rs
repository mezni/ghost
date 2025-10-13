// src/services/config_reader.rs
use crate::core::errors::AppError;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub source_type: String,
    pub source_directory: String,
    pub file_pattern: Option<String>,
    pub post_action: Option<String>,
    pub archive_directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub sources: Vec<Source>,
}

pub fn read(file_name: &str) -> Result<AppConfig, AppError> {
    let yaml = fs::read_to_string(file_name)?;
    let config: Config = serde_yaml::from_str(&yaml)?;
    Ok(config)
}
