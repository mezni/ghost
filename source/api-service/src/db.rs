use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use dotenvy::dotenv;
use std::env;
use crate::errors::AppError; 

pub type DbPool = Pool;

/// Create a connection pool from environment variables, returning your custom AppError on failure.
pub async fn get_pool() -> Result<DbPool, AppError> {
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.host = Some(env::var("DB_HOST").map_err(|e| AppError::Other(format!("DB_HOST: {}", e)))?);
    cfg.dbname = Some(env::var("DB_NAME").map_err(|e| AppError::Other(format!("DB_NAME: {}", e)))?);
    cfg.user = Some(env::var("DB_USER").map_err(|e| AppError::Other(format!("DB_USER: {}", e)))?);
    cfg.password = Some(env::var("DB_PASSWORD").map_err(|e| AppError::Other(format!("DB_PASSWORD: {}", e)))?);
    cfg.port = Some(env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap_or(5432));
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Optionally, try to get a connection to verify DB is reachable
    pool.get().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(pool)
}
