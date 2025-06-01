use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use crate::infra::error::AppError;

pub async fn establish_connection_pool(database_url: &str) -> Result<PgPool, AppError> {
    PgPoolOptions::new()
        .max_connections(5) // Adjust as needed
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
        .map_err(AppError::DatabaseError)
}