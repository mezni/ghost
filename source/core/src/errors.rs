use serde_yaml::Error as YamlError;
use std::env;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] env::VarError),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("YAML error: {0}")]
    YamlError(#[from] YamlError),
}
