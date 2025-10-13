use deadpool_postgres;
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
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Db(e) => write!(f, "DB error: {}", e),
            AppError::Pool(e) => write!(f, "Pool error: {}", e),
            AppError::Csv(e) => write!(f, "CSV error: {}", e),
            AppError::Other(s) => write!(f, "Other error: {}", s),
        }
    }
}

impl Error for AppError {}

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
