use crate::core::errors::AppError;
use crate::metrics::models::MetricsRequest;
use crate::metrics::services::MetricsService;
use actix_web::{HttpResponse, Scope, post, web};
use deadpool_postgres::Pool;

pub fn scope() -> Scope {
    web::scope("").service(handle_metrics)
}

#[post("/metrics")]
async fn handle_metrics(
    pool: web::Data<Pool>,
    req: web::Json<MetricsRequest>,
) -> Result<HttpResponse, AppError> {
    let validated_req = req.into_inner().validate()?;
    let result = MetricsService::handle_metric_request(&pool, validated_req).await?;

    Ok(HttpResponse::Ok().json(result))
}
