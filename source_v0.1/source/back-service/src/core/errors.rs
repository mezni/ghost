// core/errors.rs
use sqlx::Error as SqlxError;
use std::error::Error;
use std::fmt;
use std::io;

#[non_exhaustive]
#[derive(Debug)]
pub enum AppError {
    Other(String),
    EnvVar(std::env::VarError),
    ParseInt(std::num::ParseIntError),
    Sqlx(SqlxError),
    Io(io::Error),

    // Custom file-related errors
    InvalidDirectory(String),
    InvalidPattern(String, String),
    InvalidFileName(String),
}

impl AppError {
    /// Create a generic error from a string
    pub fn new<T: Into<String>>(msg: T) -> Self {
        AppError::Other(msg.into())
    }

    /// Helper for invalid file name
    pub fn invalid_file_name<T: Into<String>>(name: T) -> Self {
        AppError::InvalidFileName(name.into())
    }

    /// Helper for invalid pattern
    pub fn invalid_pattern<T: Into<String>, U: Into<String>>(pattern: T, err: U) -> Self {
        AppError::InvalidPattern(pattern.into(), err.into())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Other(msg) => write!(f, "{}", msg),
            AppError::EnvVar(e) => write!(f, "Environment variable error: {}", e),
            AppError::ParseInt(e) => write!(f, "Parse int error: {}", e),
            AppError::Sqlx(e) => write!(f, "SQLx error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::InvalidDirectory(path) => write!(f, "Invalid directory: {}", path),
            AppError::InvalidPattern(pat, err) => write!(f, "Invalid pattern `{}`: {}", pat, err),
            AppError::InvalidFileName(name) => write!(f, "Invalid file name: {}", name),
        }
    }
}

impl Error for AppError {}

impl From<std::env::VarError> for AppError {
    fn from(e: std::env::VarError) -> Self {
        AppError::EnvVar(e)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::ParseInt(e)
    }
}

impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        AppError::Sqlx(e)
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> Self {
        AppError::Other(format!("Regex error: {}", err))
    }
}
