use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error;
use deadpool_postgres::{PoolError, CreatePoolError};
use tokio_postgres::Error as PgError;

#[derive(Debug)]
pub enum AppError {
    MissingConfig(&'static str),
    CreatePoolError(CreatePoolError),
    PoolError(PoolError),
    DatabaseError(PgError),
    Configuration(Box<dyn Error + Send + Sync>),
    Other(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AppError::MissingConfig(key) => write!(f, "Missing configuration for: {}", key),
            AppError::CreatePoolError(e) => write!(f, "Failed to create connection pool: {}", e),
            AppError::PoolError(e) => write!(f, "Connection pool error: {}", e),
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::Configuration(e) => write!(f, "Configuration error: {}", e),
            AppError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::CreatePoolError(e) => Some(e),
            AppError::PoolError(e) => Some(e),
            AppError::DatabaseError(e) => Some(e),
            AppError::Configuration(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
