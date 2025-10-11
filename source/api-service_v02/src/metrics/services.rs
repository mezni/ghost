use crate::core::errors::AppError;
use crate::metrics::models::ValidatedMetricsRequest;
use crate::metrics::repositories::MetricsRepository;
use deadpool_postgres::Pool;
use serde_json::json;

pub struct MetricsService;

impl MetricsService {
    pub async fn handle_metric_request(
        pool: &Pool,
        req: ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let result_json = MetricsRepository::get_metrics(pool, &req).await?;

        Ok(result_json)
    }
}
