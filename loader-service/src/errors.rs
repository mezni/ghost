use tokio_postgres::error::Error as PostgresError;
use thiserror::Error;
use std::env;
use std::io;
use csv::Error as CsvError;
use serde_yaml::Error as YamlError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] PostgresError),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] env::VarError),

    #[error("CSV error: {0}")]
    CsvError(#[from] CsvError),

    #[error("YAML error: {0}")]
    YamlError(#[from] YamlError),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("Pool error: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("Create pool error: {0}")]
    CreatePoolError(#[from] deadpool_postgres::CreatePoolError),
}