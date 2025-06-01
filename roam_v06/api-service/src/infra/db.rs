use crate::errors::AppError;
use deadpool_postgres::{Client, Config, Pool, Runtime};
use tokio_postgres::NoTls;

use crate::infra::config::DatabaseConfig;

pub struct DBManager {
    pub pool: Pool,
}

impl DBManager {
    pub fn new(config: DatabaseConfig) -> Result<Self, AppError> {
        let mut pg_config = Config::new();

        // Use the strings directly — no ok_or needed
        pg_config.dbname = Some(config.name);
        pg_config.user = Some(config.user);
        pg_config.password = Some(config.password);
        pg_config.host = Some(config.host);

        let pool = pg_config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(AppError::CreatePoolError)?;

        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<Client, AppError> {
        self.pool.get().await.map_err(AppError::PoolError)
    }
}
