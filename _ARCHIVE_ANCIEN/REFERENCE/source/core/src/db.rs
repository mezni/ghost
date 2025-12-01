use crate::config::ServerConfig;
use crate::errors::AppError;
use deadpool_postgres::{Client, Config, Pool};
use tokio_postgres::NoTls;

pub struct DBManager {
    pub pool: Pool,
}

impl DBManager {
    pub fn new(config: ServerConfig) -> Result<Self, AppError> {
        let mut pg_config = Config::new();

        pg_config.dbname = Some(config.dbname.ok_or(AppError::MissingConfig("DB_NAME"))?);
        pg_config.user = Some(config.user.ok_or(AppError::MissingConfig("DB_USER"))?);
        pg_config.password = Some(
            config
                .password
                .ok_or(AppError::MissingConfig("DB_PASSWORD"))?,
        );
        pg_config.host = Some(config.host.ok_or(AppError::MissingConfig("DB_HOST"))?);

        let pool = pg_config
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .map_err(AppError::CreatePoolError)?;

        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<Client, AppError> {
        self.pool.get().await.map_err(AppError::PoolError)
    }
}
