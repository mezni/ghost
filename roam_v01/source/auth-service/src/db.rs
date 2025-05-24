use crate::config::DatabaseConfig;
use crate::errors::AppError;
use crate::logger::Logger;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub fn create_pg_pool(cfg: &DatabaseConfig) -> Result<Pool, AppError> {
    Logger::info("Creating PostgreSQL connection pool...");

    let pg_config_str = format!(
        "host={} port={} user={} password={} dbname={}",
        cfg.host, cfg.port, cfg.user, cfg.password, cfg.name
    );

    let pg_config = pg_config_str.parse().map_err(|e| {
        let msg = format!("Invalid DB config: {}", e);
        Logger::error(&msg);
        AppError::ConfigError(msg)
    })?;

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let mgr = deadpool_postgres::Manager::from_config(pg_config, NoTls, mgr_config);

    let pool = Pool::builder(mgr)
        .max_size(16)
        .runtime(Runtime::Tokio1)
        .build()
        .map_err(|e| {
            let msg = format!("Failed to build DB pool: {}", e);
            Logger::error(&msg);
            AppError::PoolError(msg)
        })?;

    Logger::info("✅ PostgreSQL pool created successfully.");
    Ok(pool)
}
