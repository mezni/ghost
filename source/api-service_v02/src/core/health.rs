// core/health.rs
use crate::core::errors::AppError;
use actix_web::{HttpResponse, get, web};
use deadpool_postgres::Pool;

#[get("/health")]
pub async fn health(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    // Test database connection
    let client = pool.get().await?; // PoolError -> AppError via From
    client.query_one("SELECT 1", &[]).await?; // PgError -> AppError via From

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "database": "connected"
    })))
}
