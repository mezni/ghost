use crate::analytics::models::MetricRequest;
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
        // Validate request type
        if req.request_type != "Metric" {
            return Err(AppError::bad_request("Invalid type"));
        }

        // Validate aggregation
        let aggregation = req.dataset.aggregation.as_str();
        if !matches!(
            aggregation,
            "Global" | "Country" | "Operator" | "Subscriber"
        ) {
            return Err(AppError::bad_request("Invalid aggregation"));
        }

        // Validate direction
        let direction = req.dataset.direction.to_uppercase();
        if !matches!(direction.as_str(), "IN" | "OUT") {
            return Err(AppError::bad_request(
                "Invalid direction: must be IN or OUT",
            ));
        }

        // Call repository and wrap immediately in JSON
        let result_json = match aggregation {
            "Global" => {
                let metrics = MetricsRepository::get_global_metrics(pool, &req).await?;
                json!(metrics)
            }
            "Country" => {
                let metrics = MetricsRepository::get_country_metrics(pool, &req).await?;
                json!(metrics)
            }
            "Operator" => {
                let metrics = MetricsRepository::get_operator_metrics(pool, &req).await?;
                json!(metrics)
            }
            "Subscriber" => {
                let metrics = MetricsRepository::get_subscriber_metrics(pool, &req).await?;
                json!(metrics)
            }
            _ => unreachable!(),
        };

        Ok(json!({
            "aggregation": aggregation,
            "direction": direction,
            "data": result_json,
            "status": "success"
        }))
    }
}
