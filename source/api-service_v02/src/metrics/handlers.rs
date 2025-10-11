use crate::core::errors::AppError;
use crate::metrics::models::MetricsRequest;
use crate::metrics::services::MetricsService;
use actix_web::{HttpResponse, Scope, post, web};
use deadpool_postgres::Pool;
use crate::core::logger::Logger;

pub fn scope() -> Scope {
    Logger::info("scope POST request");    
    web::scope("").service(handle_metrics)
}

#[post("/metrics")]
async fn handle_metrics(
    pool: web::Data<Pool>,
    req: web::Json<MetricsRequest>,
) -> Result<HttpResponse, AppError> {
    Logger::info("📥 Received metrics POST request");
    let validated_req = req.into_inner().validate()?;
    let result = MetricsService::handle_metric_request(&pool, validated_req).await?;

    Ok(HttpResponse::Ok().json(result))
}
