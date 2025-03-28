use crate::errors::AppError;
use deadpool_postgres::{Client, Config, Pool};
use dotenvy::dotenv;
use std::env;
use tokio_postgres::NoTls;
use tokio_postgres::Row;

const BATCH_STATUS_START: &str = "Started";

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
        let pool = cfg
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .map_err(|e| AppError::CreatePoolError(e))?;

        Ok(DBPool { pool })
    }

    pub async fn get_client(&self) -> Result<Client, AppError> {
        let client = self.pool.get().await.map_err(|e| AppError::PoolError(e))?;
        Ok(client)
    }
}

/// Inserts a new batch exec and returns the generated ID
pub async fn insert_batch_exec(db: &DBPool, path_name: &str) -> Result<i32, AppError> {
    let client = db.get_client().await?;
    let query = "INSERT INTO batch_execs (batch_name, start_time, batch_status) VALUES ($1, NOW(), $2) RETURNING id";
    let row = client
        .query_one(query, &[&path_name, &BATCH_STATUS_START])
        .await?;
    let id: i32 = row.try_get("id")?;
    Ok(id)
}

pub async fn update_batch_execs(db: &DBPool, batch_id: i32, status: &str) -> Result<u64, AppError> {
    let client = db.get_client().await?;
    let query = "UPDATE batch_execs SET batch_status = $1, end_time = NOW() WHERE id = $2";
    let rows_affected = client.execute(query, &[&status, &batch_id]).await?;
    Ok(rows_affected as u64)
}
