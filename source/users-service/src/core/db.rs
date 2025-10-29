use sqlx::postgres::PgPool;
use crate::core::error::Result;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}