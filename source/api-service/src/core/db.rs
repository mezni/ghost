use deadpool_postgres::{Config as DeadpoolConfig, Pool};
use dotenvy::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub struct Db;

impl Db {
    pub fn create_pool() -> Pool {
        // Load environment variables from .env
        dotenv().ok();

        let host = env::var("ROAM_DB_HOST").expect("ROAM_DB_HOST must be set");
        let dbname = env::var("ROAM_DB_NAME").expect("ROAM_DB_NAME must be set");
        let user = env::var("ROAM_DB_USER").expect("ROAM_DB_USER must be set");
        let password = env::var("ROAM_DB_PASSWORD").expect("ROAM_DB_PASSWORD must be set");
        let port = env::var("ROAM_DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let max_connections = env::var("ROAM_DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .expect("ROAM_DB_MAX_CONNECTIONS must be a valid integer");

        let mut cfg = DeadpoolConfig::new();
        cfg.host = Some(host);
        cfg.dbname = Some(dbname);
        cfg.user = Some(user);
        cfg.password = Some(password);
        cfg.port = Some(port.parse().unwrap_or(5432));

        cfg.pool = Some(deadpool_postgres::PoolConfig {
            max_size: max_connections,
            ..Default::default()
        });

        cfg.create_pool(None, NoTls)
            .expect("Failed to create database pool")
    }
}
