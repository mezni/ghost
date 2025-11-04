use crate::core::errors::AppError;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tokio::sync::OnceCell;

pub struct Db;

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

impl Db {
    /// Initialize and return a global PostgreSQL connection pool.
    pub async fn pool() -> Result<&'static PgPool, AppError> {
        DB_POOL
            .get_or_try_init(|| async {
                dotenv().ok();

                let host = env::var("ROAM_DB_HOST").map_err(AppError::EnvVar)?;
                let dbname = env::var("ROAM_DB_NAME").map_err(AppError::EnvVar)?;
                let user = env::var("ROAM_DB_USER").map_err(AppError::EnvVar)?;
                let password = env::var("ROAM_DB_PASSWORD").map_err(AppError::EnvVar)?;
                let port = env::var("ROAM_DB_PORT").unwrap_or_else(|_| "5432".to_string());

                let database_url = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    user, password, host, port, dbname
                );

                // Only needed for SQLx macros at compile time; optional at runtime
                // unsafe { env::set_var("DATABASE_URL", &database_url); }

                PgPool::connect(&database_url).await.map_err(AppError::Sqlx)
            })
            .await
    }

    /// Get the database URL for SQLx macros
    pub fn database_url() -> Result<String, AppError> {
        dotenv().ok();

        let host = env::var("ROAM_DB_HOST").map_err(AppError::EnvVar)?;
        let dbname = env::var("ROAM_DB_NAME").map_err(AppError::EnvVar)?;
        let user = env::var("ROAM_DB_USER").map_err(AppError::EnvVar)?;
        let password = env::var("ROAM_DB_PASSWORD").map_err(AppError::EnvVar)?;
        let port = env::var("ROAM_DB_PORT").unwrap_or_else(|_| "5432".to_string());

        Ok(format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, dbname
        ))
    }
}
