use actix_web::{HttpResponse, Scope, post, web};
use deadpool_postgres::Pool;

use crate::analytics::models::{ApiResponse, MetricRequest};
use crate::analytics::services::MetricsService;
use crate::core::errors::AppError;

pub fn scope() -> Scope {
    web::scope("/metrics").service(get_metric)
}

#[post("")]
pub async fn get_metric(
    pool: web::Data<Pool>,
    req: web::Json<MetricRequest>,
) -> Result<HttpResponse, AppError> {
    let result = MetricsService::handle_metric_request(&pool, req.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        data: result,
    }))
}
