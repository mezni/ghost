use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub async fn create_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.host = std::env::var("DB_HOST").unwrap_or("localhost".to_string());
    cfg.port = std::env::var("DB_PORT").unwrap_or("5432".to_string()).parse().unwrap();
    cfg.user = std::env::var("DB_USER").unwrap_or("postgres".to_string());
    cfg.password = std::env::var("DB_PASS").unwrap_or("postgres".to_string());
    cfg.dbname = std::env::var("DB_NAME").unwrap_or("user_management".to_string());
    
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create database pool")
}