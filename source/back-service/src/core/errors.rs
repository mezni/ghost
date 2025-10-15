use deadpool_postgres;
use regex::Error as RegexError;
use std::error::Error;
use std::fmt;
use std::io;
use tokio_postgres;

/// Custom application error type
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Db(tokio_postgres::Error),
    Pool(deadpool_postgres::PoolError),
    Csv(csv_async::Error),
    Yaml(serde_yaml::Error),
    Regex(RegexError),
    InvalidFileName(String), // Now includes the file name for context
    InvalidDirectory(String),
    InvalidPattern(String, String),
    Other(String),
}

impl AppError {
    pub fn new<S: Into<String>>(msg: S) -> Self {
        AppError::Other(msg.into())
    }

    pub fn invalid_directory<S: Into<String>>(path: S) -> Self {
        AppError::InvalidDirectory(path.into())
    }

    pub fn invalid_pattern<S1: Into<String>, S2: Into<String>>(pattern: S1, error: S2) -> Self {
        AppError::InvalidPattern(pattern.into(), error.into())
    }

    pub fn invalid_file_name<S: Into<String>>(name: S) -> Self {
        AppError::InvalidFileName(name.into())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Db(e) => write!(f, "Database error: {}", e),
            AppError::Pool(e) => write!(f, "Connection pool error: {}", e),
            AppError::Csv(e) => write!(f, "CSV processing error: {}", e),
            AppError::Yaml(e) => write!(f, "YAML processing error: {}", e),
            AppError::Regex(e) => write!(f, "Regex error: {}", e),
            AppError::InvalidFileName(name) => write!(f, "Invalid file name: {}", name),
            AppError::InvalidDirectory(path) => write!(f, "Invalid directory: {}", path),
            AppError::InvalidPattern(pattern, error) => {
                write!(f, "Invalid regex pattern '{}': {}", pattern, error)
            }
            AppError::Other(s) => write!(f, "Application error: {}", s),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Db(e) => Some(e),
            AppError::Pool(e) => Some(e),
            AppError::Csv(e) => Some(e),
            AppError::Yaml(e) => Some(e),
            AppError::Regex(e) => Some(e),
            AppError::InvalidFileName(_)
            | AppError::InvalidDirectory(_)
            | AppError::InvalidPattern(_, _)
            | AppError::Other(_) => None,
        }
    }
}

/// Conversion implementations
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        AppError::Db(err)
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        AppError::Pool(err)
    }
}

impl From<csv_async::Error> for AppError {
    fn from(err: csv_async::Error) -> Self {
        AppError::Csv(err)
    }
}

impl From<serde_yaml::Error> for AppError {
    fn from(err: serde_yaml::Error) -> Self {
        AppError::Yaml(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Other(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<RegexError> for AppError {
    fn from(err: RegexError) -> Self {
        AppError::Regex(err)
    }
}
