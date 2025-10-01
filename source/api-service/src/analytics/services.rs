use deadpool_postgres::Pool;
use serde_json::Value;

use crate::analytics::models::{CountryMetric, GlobalMetric, MetricRequest};
use crate::analytics::repositories::MetricRepository;
use crate::core::errors::AppError;

pub struct MetricsService;

impl MetricsService {
    /// Handle POST /metrics request dynamically
    pub async fn handle_metric_request(pool: &Pool, req: MetricRequest) -> Result<Value, AppError> {
        match req.aggregation.as_str() {
            "GLOBAL" => {
                let metrics: Vec<GlobalMetric> = match req.direction.as_str() {
                    "IN" => MetricRepository::fetch_global_in_metrics(pool).await?,
                    "OUT" => MetricRepository::fetch_global_out_metrics(pool).await?,
                    _ => return Err(AppError::BadRequest("Invalid direction".into())),
                };
                Ok(serde_json::to_value(metrics)
                    .map_err(|e| AppError::Other(format!("JSON serialization error: {}", e)))?)
            }
            "COUNTRY" => {
                let metrics: Vec<CountryMetric> = match req.direction.as_str() {
                    "IN" => MetricRepository::fetch_country_in_metrics(pool).await?,
                    "OUT" => MetricRepository::fetch_country_out_metrics(pool).await?,
                    _ => return Err(AppError::BadRequest("Invalid direction".into())),
                };
                Ok(serde_json::to_value(metrics)
                    .map_err(|e| AppError::Other(format!("JSON serialization error: {}", e)))?)
            }
            _ => return Err(AppError::BadRequest("Invalid aggregation type".into())),
        }
    }
}
