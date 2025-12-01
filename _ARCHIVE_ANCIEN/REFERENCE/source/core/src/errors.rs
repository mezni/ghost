use csv::Error as CsvError;
use regex::Error as RegexError;
use serde_yaml::Error as YamlError;
use std::env;
use std::io;
use std::num::ParseIntError;
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

    #[error("Regex error: {0}")]
    RegexError(#[from] RegexError), // Added variant for RegexError

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

    // File system related errors
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Failed to read file {path}: {source}")]
    FileReadError {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("Invalid config {0}")]
    InvalidConfig(String),

    // Added variant for ParseIntError
    #[error("Integer parse error: {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("Bad request: {0}")]
    BadRequest(String),
}
