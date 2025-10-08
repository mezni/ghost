use crate::core::errors::AppError;
use crate::metrics::models::{GlobalMetric, ValidatedMetricsRequest};
use deadpool_postgres::Pool;
use serde_json::json;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;

pub struct MetricsRepository;

impl MetricsRepository {
    const GET_GLOBAL_METRICS_QUERY: &str = "SELECT rd.date_str AS date, tmg.value
                                        FROM trx_metrics_global tmg
                                        JOIN ref_metric_definitions rmd ON rmd.metric_definition_id = tmg.metric_definition_id
                                        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
                                        JOIN ref_dates rd ON rd.date_id = tmg.date_id
                                        WHERE 1=1
                                        AND (tmg.date_id, tmg.batch_id) IN (SELECT date_id, MAX(batch_id) AS max_batch_id
                                                                            FROM trx_metrics_global
                                                                            GROUP BY date_id)
                                        AND UPPER(rrd.direction) = UPPER($1)";

    pub async fn get_metrics(
        pool: &Pool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        println!("{:?}", req);

        let client = pool
            .get()
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let dimension = req.dimension.to_lowercase();
        let result_json = match dimension.as_str() {
            "global" => MetricsRepository::get_global_metrics(&client, req).await?,
            "country" => json!({ "dimension": "country", "value": 2 }),
            _ => return Err(AppError::bad_request("Unsupported dimension")),
        };

        Ok(result_json)
    }

    pub async fn get_global_metrics(
        client: &deadpool_postgres::Client,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let (query, params) = if req.window == 0 {
            let query = format!(
                "{} AND tmg.date_id = (SELECT MAX(date_id) FROM trx_metrics_global)",
                Self::GET_GLOBAL_METRICS_QUERY
            );
            let params: Vec<&(dyn ToSql + Sync)> = vec![&req.direction];
            (query, params)
        } else {
            let query = format!(
                "{} AND rd.date >= CURRENT_DATE - INTERVAL '{} days'",
                Self::GET_GLOBAL_METRICS_QUERY,
                req.window
            );
            let params: Vec<&(dyn ToSql + Sync)> = vec![&req.direction];
            (query, params)
        };

        let rows = client
            .query(&query, &params)
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let metrics: Vec<GlobalMetric> = rows.iter().map(Self::map_global_metric).collect();

        Ok(serde_json::json!(
            metrics
        ))
    }

    fn map_global_metric(row: &Row) -> GlobalMetric {
        GlobalMetric {
            date: row.get("date"),
            value: row.get("value"),
        }
    }
}
