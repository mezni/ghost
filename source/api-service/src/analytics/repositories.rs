use crate::analytics::models::{
    CountryMetric, Filter, GlobalMetric, NotifMetric, ValidatedMetricsRequest,
};
use crate::core::errors::AppError;
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

    const GET_COUNTRY_METRICS_QUERY: &str = "SELECT rd.date_str AS date, cc.country_name AS country,fct.value
                                        FROM trx_metrics_country fct
                                        JOIN ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
                                        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
                                        JOIN ref_dates rd ON rd.date_id = fct.date_id
                                        JOIN cfg_countries cc ON cc.country_id = fct.country_id
                                        WHERE 1=1
                                        AND (fct.date_id, fct.batch_id) IN (SELECT date_id, MAX(batch_id) AS max_batch_id
                                                                            FROM trx_metrics_country
                                                                            GROUP BY date_id)
                                        AND UPPER(rrd.direction) = UPPER($1)";

    const GET_COUNTRY_METRICS_TOP_QUERY: &str = "SELECT date, country, SUM(value)::bigint AS value
                                        FROM (
                                            SELECT 
                                                rd.date_str AS date, 
                                                CASE 
                                                    WHEN ROW_NUMBER() OVER (PARTITION BY rd.date_str ORDER BY fct.value DESC) <= CAST ($2 AS INT)
                                                    THEN cc.country_name 
                                                    ELSE 'Others' 
                                                END AS country,
                                                fct.value
                                            FROM 
                                                trx_metrics_country fct
                                            JOIN 
                                                ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
                                            JOIN 
                                                ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
                                            JOIN 
                                                ref_dates rd ON rd.date_id = fct.date_id
                                            JOIN 
                                                cfg_countries cc ON cc.country_id = fct.country_id
                                            WHERE 
                                                1 = 1
                                                AND (fct.date_id, fct.batch_id) IN (
                                                    SELECT 
                                                        date_id, 
                                                        MAX(batch_id) AS max_batch_id
                                                    FROM 
                                                        trx_metrics_country
                                                    GROUP BY 
                                                        date_id
                                                )
                                                AND UPPER(rrd.direction) = UPPER($1)
                                                AND fct.date_id = (
                                                    SELECT 
                                                        MAX(date_id) 
                                                    FROM 
                                                        trx_metrics_country
                                                )
                                        ) AS ranked
                                        GROUP BY 
                                            date, 
                                            country
                                        ORDER BY 
                                            value DESC";

    const GET_NOTIF_DET_METRICS_QUERY: &str = "SELECT rd.date_str AS date, fct.message AS value FROM trx_notifications fct
                                            JOIN ref_dates rd ON rd.date_id = fct.date_id 
                                            WHERE 1=1
                                            AND fct.date_id = (SELECT MAX(date_id) FROM trx_notifications)";

    const GET_NOTIF_SUM_METRICS_QUERY: &str = "SELECT rd.date_str AS date, COUNT(*)::text AS value FROM trx_notifications fct
                                            JOIN ref_dates rd ON rd.date_id = fct.date_id 
                                            WHERE 1=1
                                            AND fct.date_id = (SELECT MAX(date_id) FROM trx_notifications)
                                            GROUP BY rd.date_str";

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
            "country" => MetricsRepository::get_country_metrics(&client, req).await?,
            "notification" => MetricsRepository::get_notif_metrics(&client, req).await?,
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

    pub async fn get_country_metrics(
        client: &deadpool_postgres::Client,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let direction = get_direction_from_filters(req.filter.as_ref())?;
        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;
        let country = get_country_from_filters(req.filter.as_ref());

        match aggregation.as_str() {
            "latest" | "history" => {
                let mut query = Self::GET_COUNTRY_METRICS_QUERY.to_string();
                let mut params: Vec<&(dyn ToSql + Sync)> = vec![&direction];

                if !country.is_empty() {
                    query.push_str(" AND UPPER(cc.country_name) = UPPER($2)");
                    params.push(&country);
                }

                match aggregation.as_str() {
                    "latest" => {
                        query.push_str(
                            " AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_country)",
                        );
                    }
                    "history" => {
                        query.push_str(&format!(
                            " AND rd.date >= CURRENT_DATE - INTERVAL '{} days'",
                            size
                        ));
                    }
                    _ => unreachable!(),
                }

                query.push_str(" ORDER BY rd.date_str");

                let rows = client
                    .query(&query, &params)
                    .await
                    .map_err(|e| AppError::db_error(&e.to_string()))?;

                let metrics: Vec<CountryMetric> =
                    rows.iter().map(Self::map_country_metric).collect();

                Ok(json!({
                    "data": metrics,
                    "status": "success",
                }))
            }
            "top" => {
                let query = format!("{}", Self::GET_COUNTRY_METRICS_TOP_QUERY);

                let params: Vec<&(dyn ToSql + Sync)> = vec![&direction, &size];

                let rows = client
                    .query(&query, &params)
                    .await
                    .map_err(|e| AppError::db_error(&e.to_string()))?;

                let metrics: Vec<CountryMetric> =
                    rows.iter().map(Self::map_country_metric).collect();

                Ok(json!({
                    "data": metrics,
                    "status": "success",
                }))
            }
            _ => Err(AppError::bad_request(
                "Aggregation 'latest', 'history' or 'top' is required",
            )),
        }
    }

    pub async fn get_notif_metrics(
        client: &deadpool_postgres::Client,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let aggregation = &req.aggregation;

        let (query, params) = match aggregation.as_str() {
            "summary" => {
                let query = Self::GET_NOTIF_SUM_METRICS_QUERY.to_string();
                let params: Vec<&(dyn ToSql + Sync)> = vec![];
                (query, params)
            }
            "detail" => {
                let query = Self::GET_NOTIF_DET_METRICS_QUERY.to_string();
                let params: Vec<&(dyn ToSql + Sync)> = vec![];
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

        let metrics: Vec<NotifMetric> = rows.iter().map(Self::map_notif_metric).collect();

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

fn get_size_for_aggregation(aggregation: &str, size: Option<u32>) -> Result<i32, AppError> {
    match aggregation {
        "history" => Ok(size.map(|s| s as i32).unwrap_or(30)),
        "top" => Ok(size.map(|s| s as i32).unwrap_or(5)),
        _ => Ok(5),
    }
}

fn get_country_from_filters(filters: Option<&Vec<Filter>>) -> String {
    if let Some(filters) = filters {
        filters
            .iter()
            .find_map(|filter| {
                if filter.key.to_lowercase() == "country" {
                    Some(filter.value.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    } else {
        "".to_string()
    }
}
