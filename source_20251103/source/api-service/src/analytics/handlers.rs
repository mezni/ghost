use crate::analytics::models::MetricsRequest;
use crate::analytics::services::MetricsService;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use actix_web::{HttpResponse, Scope, get, post, web};
use sqlx::PgPool;

/// Configure the `/analytics` scope
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_analytics).service(test_analytics);
}

#[post("/analytics")]
async fn handle_analytics(
    pool: web::Data<PgPool>,
    req: web::Json<MetricsRequest>,
) -> Result<HttpResponse, AppError> {
    let validated_req = req.into_inner().validate()?;
    let result = MetricsService::handle_metric_request(&pool, validated_req).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/test")]
async fn test_analytics(_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    Logger::info("Testing analytics endpoint");

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Analytics test endpoint is working!",
        "status": "success",
    })))
}
