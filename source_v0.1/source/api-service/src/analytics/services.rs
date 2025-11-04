use crate::analytics::models::ValidatedMetricsRequest;
use crate::analytics::repositories::MetricsRepository;
use crate::core::errors::AppError;
use serde_json::json;
use sqlx::PgPool;

pub struct MetricsService;

impl MetricsService {
    pub async fn handle_metric_request(
        pool: &PgPool,
        req: ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let result_json = MetricsRepository::get_metrics(pool, &req).await?;
        Ok(result_json)
    }
}
