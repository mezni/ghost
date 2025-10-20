// core/health.rs
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use actix_web::{HttpResponse, get, web};
use deadpool_postgres::Pool;

#[get("/health")]
pub async fn health(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    Logger::debug("call /health");
    let client = pool.get().await?;
    client.query_one("SELECT 1", &[]).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "database": "connected"
    })))
}
