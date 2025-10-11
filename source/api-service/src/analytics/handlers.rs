use crate::analytics::models::MetricsRequest;
use crate::analytics::services::MetricsService;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use actix_web::{HttpResponse, Scope, get, post, web};
use deadpool_postgres::Pool;

pub fn scope() -> Scope {
    web::scope("/analytics")  // Changed from "" to "/analytics"
        .service(handle_analytics)
        .service(test_analytics)
}

#[post("")]  // Changed from "/analytics" to ""
async fn handle_analytics(
    pool: web::Data<Pool>,
    req: web::Json<MetricsRequest>,
) -> Result<HttpResponse, AppError> {
    let validated_req = req.into_inner().validate()?;
    let result = MetricsService::handle_metric_request(&pool, validated_req).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/test")]  // Changed from "/analytics/test" to "/test"
async fn test_analytics(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    Logger::info("Testing analytics endpoint");

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Analytics test endpoint is working!",
        "status": "success",
    })))
}