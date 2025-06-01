// src/infra/postgres/db.rs

use crate::errors::AppError; // Import your central AppError
use crate::infra::config::DatabaseConfig;
use sqlx::{
    ConnectOptions, // This trait provides methods like `log_statements`
    Error as SqlxError,
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::time::Duration;
use tracing::log::LevelFilter; // For sqlx logging level // Import DatabaseConfig

/// Initializes and returns a PostgreSQL connection pool.
///
/// This function constructs a database URL from the provided `DatabaseConfig`,
/// configures the connection pool options (min/max connections, timeouts),
/// and attempts to establish a connection to the PostgreSQL database.
/// It also performs a simple test query to ensure the connection is live.
///
/// # Arguments
/// * `db_config` - A reference to the `DatabaseConfig` containing database credentials.
///
/// # Returns
/// A `Result` containing `PgPool` on success, or an `AppError` on failure.
pub async fn init_db_pool(db_config: &DatabaseConfig) -> Result<PgPool, AppError> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.name
    );

    let connect_options: PgConnectOptions = database_url
        .parse()
        .map_err(|e: SqlxError| AppError::DatabaseError(format!("Invalid database URL: {}", e)))?;

    let configured_connect_options = connect_options.log_statements(LevelFilter::Info); // OK, keep this for logging

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10)) // This is the only valid timeout here
        .idle_timeout(Duration::from_secs(30))
        .test_before_acquire(true)
        .connect_with(configured_connect_options)
        .await
        .map_err(|e: SqlxError| {
            AppError::DatabaseConnectionError(format!("Failed to connect to database: {}", e))
        })?;

    sqlx::query("SELECT 1") // use runtime-checked query
        .execute(&pool)
        .await
        .map_err(|e: SqlxError| {
            AppError::DatabaseError(format!("Database test query failed: {}", e))
        })?;

    Ok(pool)
}
