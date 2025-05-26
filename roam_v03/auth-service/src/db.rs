use crate::config::DatabaseConfig;
use crate::errors::AppError;
use crate::logger::Logger;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub fn create_pg_pool(cfg: &DatabaseConfig) -> Result<Pool, AppError> {
    Logger::info("Creating PostgreSQL connection pool...");

    // Build connection string
    let pg_config_str = format!(
        "host={} port={} user={} password={} dbname={}",
        cfg.host, cfg.port, cfg.username, cfg.password, cfg.database
    );

    // Parse connection string into tokio_postgres::Config
    let pg_config = pg_config_str
        .parse::<tokio_postgres::Config>()
        .map_err(|e| {
            let msg = format!("Invalid DB config: {}", e);
            Logger::error(&msg);
            AppError::InternalServerError(msg) // or a custom ConfigError variant if you want
        })?;

    // Manager config for recycling connections
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    // Create manager
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);

    // Build the pool with max size and runtime
    let pool = Pool::builder(mgr)
        .max_size(16)
        .runtime(Runtime::Tokio1)
        .build()
        .map_err(|e| {
            let msg = format!("Failed to build DB pool: {}", e);
            Logger::error(&msg);
            AppError::InternalServerError(msg)
        })?;

    Logger::info("PostgreSQL pool created successfully.");
    Ok(pool)
}
