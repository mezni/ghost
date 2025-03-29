use crate::config::ServerConfig;
use crate::errors::AppError;
use deadpool_postgres::{Client, Config, Pool};
use tokio_postgres::NoTls;

const BATCH_STATUS_START: &str = "Started";

pub struct StoreManager {
    pub pool: Pool,
}

impl StoreManager {
    pub fn new(config: ServerConfig) -> Result<Self, AppError> {
        let mut pg_config = Config::new();
        pg_config.dbname = Some(config.dbname.unwrap_or_default());
        pg_config.user = Some(config.user.unwrap_or_default());
        pg_config.password = Some(config.password.unwrap_or_default());
        pg_config.host = Some(config.host.unwrap_or_default());

        let pool = pg_config
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .map_err(|e| AppError::CreatePoolError(e))?;

        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<Client, AppError> {
        let client = self.pool.get().await.map_err(|e| AppError::PoolError(e))?;
        Ok(client)
    }
}

pub async fn insert_batch_exec(store_mgr: &StoreManager, path_name: &str) -> Result<i32, AppError> {
    let client = store_mgr.get_client().await?;
    let query = "INSERT INTO batch_execs (batch_name, start_time, batch_status) VALUES ($1, NOW(), $2) RETURNING id";
    let row = client
        .query_one(query, &[&path_name, &BATCH_STATUS_START])
        .await?;
    let id: i32 = row.try_get("id")?;
    Ok(id)
}

pub async fn update_batch_execs(
    store_mgr: &StoreManager,
    batch_id: i32,
    status: &str,
) -> Result<u64, AppError> {
    let client = store_mgr.get_client().await?;
    let query = "UPDATE batch_execs SET batch_status = $1, end_time = NOW() WHERE id = $2";
    let rows_affected = client.execute(query, &[&status, &batch_id]).await?;
    Ok(rows_affected as u64)
}
