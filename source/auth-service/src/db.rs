use crate::errors::AppError;
use deadpool_postgres::{Client, Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use dotenvy::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub type DbPool = Pool;

/// Establish a PostgreSQL connection pool and register custom types
pub async fn get_pool() -> Result<DbPool, AppError> {
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.host = Some(env_var("AUTH_DB_HOST")?);
    cfg.dbname = Some(env_var("AUTH_DB_NAME")?);
    cfg.user = Some(env_var("AUTH_DB_USER")?);
    cfg.password = Some(env_var("AUTH_DB_PASSWORD")?);
    cfg.port = Some(
        env::var("AUTH_DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .unwrap_or(5432),
    );
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| AppError::Database(e.to_string()))?;

    // ✅ Ensure we register the enum type with PostgreSQL
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    register_enum_type(&client).await?;

    Ok(pool)
}

fn env_var(name: &str) -> Result<String, AppError> {
    env::var(name).map_err(|e| AppError::Other(format!("{}: {}", name, e)))
}

/// 🔁 Ensure PostgreSQL registers the `role_type` enum for Rust type mapping
async fn register_enum_type(client: &Client) -> Result<(), AppError> {
    client.execute("SELECT 'user'::role_type", &[]).await?;
    Ok(())
}
