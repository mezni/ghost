use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub sleep_duration_seconds: u64,
    pub sources: Vec<Source>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    pub source_type: String,
    pub source_directory: String,
    pub file_pattern: String,
    pub post_action: String,
    pub archive_directory: Option<String>,
}

/// Reads the configuration from a YAML file.
pub fn read_config(file_path: &str) -> Result<Config, AppError> {
    let file = File::open(Path::new(file_path))?;
    let config = serde_yaml::from_reader(file).map_err(AppError::YamlError)?;
    Ok(config)
}
