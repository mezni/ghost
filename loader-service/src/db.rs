use deadpool_postgres::{Client, Config, Pool};
use tokio_postgres::NoTls;
use std::env;
use dotenvy::dotenv;
use crate::errors::AppError;

pub struct DBPool {
    pool: Pool,
}

impl DBPool {
    pub fn new() -> Result<Self, AppError> {
        dotenv().ok(); // Load environment variables

        let mut cfg = Config::new();

        // Retrieve DB connection details
        cfg.dbname = Some(env::var("DB_NAME")?.to_string());
        cfg.user = Some(env::var("DB_USER")?.to_string());
        cfg.password = Some(env::var("DB_PASSWORD")?.to_string());
        cfg.host = Some(env::var("DB_HOST")?.to_string());

        // Create a new pool
        let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .map_err(|e| AppError::CreatePoolError(e))?;

        Ok(DBPool { pool })
    }

    pub async fn get_client(&self) -> Result<Client, AppError> {
        let client = self.pool.get().await.map_err(|e| AppError::PoolError(e))?;
        Ok(client)
    }
}