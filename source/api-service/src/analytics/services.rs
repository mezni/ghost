use crate::analytics::models::MetricsRequest;
use crate::analytics::repositories::MetricsRepository;
use crate::core::errors::AppError;
use deadpool_postgres::Pool;
use serde_json::json;

pub struct MetricsService;

impl MetricsService {
    pub async fn handle_metric_request(
        pool: &Pool,
        req: MetricsRequest, // Use MetricsRequest here
    ) -> Result<serde_json::Value, AppError> {
        // Validate request type
        let metric = req.metric.to_lowercase();
        if !matches!(metric.as_str(), "metric" | "definition") {
            return Err(AppError::bad_request("Invalid metric"));
        }

        // Validate dimension
        let dimension = req.dimension.to_lowercase();
        if !matches!(
            dimension.as_str(),
            "global" | "country" | "operator" | "subscriber"
        ) {
            return Err(AppError::bad_request("Invalid dimension"));
        }

        // Validate direction
        let direction = req.direction.to_uppercase();
        if !matches!(direction.as_str(), "IN" | "OUT") {
            return Err(AppError::bad_request(
                "Invalid direction: must be IN or OUT",
            ));
        }

        // Call repository and wrap immediately in JSON
        let result_json = match dimension.as_str() {
            "global" => {
                let metrics = MetricsRepository::get_global_metrics(pool, &req).await?;
                json!(metrics)
            }

            "country" => {
                let metrics = MetricsRepository::get_country_metrics(pool, &req).await?;
                json!(metrics)
            }

            //            "operator" => {
            //                // Handle operator dimension if necessary
            //                let metrics = MetricsRepository::get_operator_metrics(pool, &req).await?;
            //                json!(metrics)
            //            }

            //            "subscriber" => {
            //                // Handle subscriber dimension if necessary
            //                let metrics = MetricsRepository::get_subscriber_metrics(pool, &req).await?;
            //                json!(metrics)
            //            }
            _ => {
                return Err(AppError::bad_request("Unsupported dimension"));
            }
        };

        Ok(json!({
            "data": result_json,
            "status": "success"
        }))
    }
}
