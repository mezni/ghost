use deadpool_postgres::{Client, Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use crate::errors::AppError;

pub fn create_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.dbname = Some("your_db".to_string());
    cfg.user = Some("your_user".to_string());
    cfg.password = Some("your_pass".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create DB pool")
}

pub async fn get_client(pool: &Pool) -> Result<Client, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DbError(format!("Failed to get DB client: {}", e)))
}
