use crate::analytics::models::{CountryMetric, GlobalMetric, MetricRequest};
use crate::analytics::repositories::MetricsRepository;
use crate::core::errors::AppError;
use deadpool_postgres::Pool;
use serde_json::json;

pub struct MetricsService;

impl MetricsService {
    pub async fn handle_metric_request(
        pool: &Pool,
        req: MetricRequest,
    ) -> Result<serde_json::Value, AppError> {
        match req.dataset.aggregation.as_str() {
            "Global" => {
                let metrics = MetricsRepository::get_global_metrics(
                    pool,
                    req.dataset.aggregation,
                    req.dataset.granularity,
                    req.timePeriod.window,
                    req.timePeriod.from,
                    req.timePeriod.to,
                )
                .await?;

                Ok(json!({
                    "aggregation": "Global",
                    "metrics": metrics
                }))
            }
            "Country" => {
                let metrics = MetricsRepository::get_country_metrics(
                    pool,
                    req.dataset.aggregation,
                    req.dataset.granularity,
                    req.timePeriod.window,
                    req.timePeriod.from,
                    req.timePeriod.to,
                    req.filter.country, // ✅ fixed field name
                    req.filter.operator,
                    req.filter.subscriber,
                )
                .await?;

                Ok(json!({
                    "aggregation": "Country",
                    "metrics": metrics
                }))
            }
            _ => Err(AppError::bad_request("Invalid aggregation")),
        }
    }
}
