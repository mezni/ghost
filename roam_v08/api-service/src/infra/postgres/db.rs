use crate::errors::AppError;
use crate::infra::config::DatabaseConfig;
use sqlx::{
    ConnectOptions, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::time::Duration;
use tracing::log::LevelFilter;

/// Initializes and returns a PostgreSQL connection pool.
pub async fn init_db_pool(db_config: &DatabaseConfig) -> Result<PgPool, AppError> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.name
    );

    // Parse connection options from the database URL
    let connect_options: PgConnectOptions =
        database_url.parse().map_err(AppError::DatabaseError)?;

    let configured_connect_options = connect_options.log_statements(LevelFilter::Info);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(30))
        .test_before_acquire(true)
        .connect_with(configured_connect_options)
        .await
        .map_err(AppError::DatabaseError)?;

    // Test query to ensure the connection works
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(pool)
}
