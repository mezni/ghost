use crate::config::ServerConfig;
use crate::errors::AppError;
use deadpool_postgres::{Client, Config, Pool};
use tokio_postgres::NoTls;
use tokio_postgres::Row;
use tokio_postgres::types::FromSql;

const BATCH_STATUS_START: &str = "Started";
const INSERT_BATCH_EXEC_QUERY: &str = "INSERT INTO batch_execs (batch_name, source_name, start_time, batch_status) \
                                       VALUES ($1, $2, NOW(), $3) RETURNING id";

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

    pub async fn insert_batch_exec(
        &self,
        batch_name: &str,
        source_name: &str,
    ) -> Result<i32, AppError> {
        let client = self.get_client().await?;
        let row = client
            .query_one(
                INSERT_BATCH_EXEC_QUERY,
                &[&batch_name, &source_name, &BATCH_STATUS_START],
            )
            .await?;
        let id: i32 = row.try_get("id")?;
        Ok(id)
    }
}
