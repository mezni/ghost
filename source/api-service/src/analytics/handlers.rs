use crate::analytics::models::MetricRequest;
use crate::analytics::services::MetricsService;
use crate::core::errors::AppError;
use actix_web::{HttpResponse, Scope, post, web};
use deadpool_postgres::Pool;

pub fn scope() -> Scope {
    web::scope("").service(handle_metrics)
}

#[post("/metrics")]
async fn handle_metrics(
    pool: web::Data<Pool>,
    req: web::Json<MetricRequest>,
) -> Result<HttpResponse, AppError> {
    let result = MetricsService::handle_metric_request(&pool, req.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}
