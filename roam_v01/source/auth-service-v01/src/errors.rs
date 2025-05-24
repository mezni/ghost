use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("PostgreSQL pool error: {0}")]
    PoolError(String),

    #[error("DB error: {0}")]
    DBError(String),

    #[error("Authentication error")]
    AuthError,
    
    #[error("Password hashing error")]
    HashError(#[from] anyhow::Error),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unexpected error: {0}")]
    Other(String),
}

impl warp::reject::Reject for AppError {}