use crate::core::errors::AppError;
use crate::metrics::models::{
    CountryMetric, Filter, GlobalMetric, NotifMetric, ValidatedMetricsRequest,
};
use deadpool_postgres::Pool;
use serde_json::json;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;

pub struct MetricsRepository;

impl MetricsRepository {
    const GET_GLOBAL_METRICS_QUERY: &str = "SELECT rd.date_str AS date, fct.value
                                        FROM trx_metrics_global fct
                                        JOIN ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
                                        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
                                        JOIN ref_dates rd ON rd.date_id = fct.date_id
                                        WHERE 1=1
                                        AND (fct.date_id, fct.batch_id) IN (SELECT date_id, MAX(batch_id) AS max_batch_id
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
            //            "country" => MetricsRepository::get_country_metrics(&client, req).await?,
            //            "notification" => MetricsRepository::get_notif_metrics(&client, req).await?,
            _ => return Err(AppError::bad_request("Unsupported dimension")),
        };

        Ok(result_json)
    }

    pub async fn get_global_metrics(
        client: &deadpool_postgres::Client,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let direction = get_direction_from_filters(req.filter.as_ref())?;
        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;

        let (query, params) = match aggregation.as_str() {
            "latest" => {
                let query = format!(
                    "{} AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_global) ORDER BY rd.date_str",
                    Self::GET_GLOBAL_METRICS_QUERY
                );
                let params: Vec<&(dyn ToSql + Sync)> = vec![&direction];
                (query, params)
            }
            "history" => {
                let query = format!(
                    "{} AND rd.date >= CURRENT_DATE - INTERVAL '{} days' ORDER BY rd.date_str",
                    Self::GET_GLOBAL_METRICS_QUERY,
                    size
                );
                let params: Vec<&(dyn ToSql + Sync)> = vec![&direction];
                (query, params)
            }
            _ => {
                return Err(AppError::bad_request(
                    "Aggregation 'latest' or 'history' is required",
                ));
            }
        };

        let rows = client
            .query(&query, &params)
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let metrics: Vec<GlobalMetric> = rows.iter().map(Self::map_global_metric).collect();

        Ok(json!({
            "data": metrics,
            "status": "success",
        }))
    }

    fn map_global_metric(row: &Row) -> GlobalMetric {
        GlobalMetric {
            date: row.get("date"),
            value: row.get("value"),
        }
    }

    fn map_country_metric(row: &Row) -> CountryMetric {
        CountryMetric {
            date: row.get("date"),
            country: row.get("country"),
            value: row.get("value"),
        }
    }

    fn map_notif_metric(row: &Row) -> NotifMetric {
        NotifMetric {
            date: row.get("date"),
            value: row.get("value"),
        }
    }
}

fn get_direction_from_filters(filters: Option<&Vec<Filter>>) -> Result<String, AppError> {
    if let Some(filters) = filters {
        filters
            .iter()
            .find_map(|filter| {
                if filter.key.to_lowercase() == "direction" {
                    Some(filter.value.clone())
                } else {
                    None
                }
            })
            .ok_or(AppError::bad_request("Direction is required"))
            .and_then(|dir| {
                if ["in", "out"].contains(&dir.to_lowercase().as_str()) {
                    Ok(dir)
                } else {
                    Err(AppError::bad_request("Direction must be 'in' or 'out'"))
                }
            })
    } else {
        Err(AppError::bad_request("Direction is required"))
    }
}

fn get_size_for_aggregation(aggregation: &str, size: Option<u32>) -> Result<u32, AppError> {
    match aggregation {
        "history" => Ok(size.unwrap_or(30)),
        _ => Ok(5),
    }
}
