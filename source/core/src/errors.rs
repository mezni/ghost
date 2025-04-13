use csv::Error as CsvError;
use serde_yaml::Error as YamlError;
use std::env;
use std::io;
use thiserror::Error;
use tokio_postgres::error::Error as PostgresError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] env::VarError),

    #[error("YAML error: {0}")]
    YamlError(#[from] YamlError),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("CSV error: {0}")]
    CsvError(#[from] CsvError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] PostgresError),

    #[error("Database error: {0}")]
    DatabaseErrorString(String),

    #[error("Pool error: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("Create pool error: {0}")]
    CreatePoolError(#[from] deadpool_postgres::CreatePoolError),

    #[error("Missing config: {0}")]
    MissingConfig(&'static str),
}
