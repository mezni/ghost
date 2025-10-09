use crate::core::errors::AppError;
use crate::metrics::models::{CountryMetric, GlobalMetric, ValidatedMetricsRequest};
use deadpool_postgres::Pool;
use serde_json::json;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;

const DEFAULT_SIZE: u32 = 5;

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
                                                    WHEN ROW_NUMBER() OVER (PARTITION BY rd.date_str ORDER BY fct.value DESC) <= $2
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
                "{} AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_global) ORDER BY rd.date_str",
                Self::GET_GLOBAL_METRICS_QUERY
            );
            let params: Vec<&(dyn ToSql + Sync)> = vec![&req.direction];
            (query, params)
        } else {
            let query = format!(
                "{} AND rd.date_str >= CURRENT_DATE - INTERVAL '{} days' ORDER BY rd.date_str",
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

        Ok(serde_json::json!(metrics))
    }

    pub async fn get_country_metrics(
        client: &deadpool_postgres::Client,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        // Solution 1: Store parameters in a tuple with the same lifetime
        let (query, params): (String, Vec<Box<dyn ToSql + Sync>>) = match req.aggregation.as_deref()
        {
            Some("top") => {
                let size = req.size.unwrap_or(DEFAULT_SIZE) as i64;
                let query = Self::GET_COUNTRY_METRICS_TOP_QUERY.to_string();
                let params: Vec<Box<dyn ToSql + Sync>> =
                    vec![Box::new(req.direction.clone()), Box::new(size)];
                (query, params)
            }
            _ => {
                let base_query = if req.window == 0 {
                    format!(
                        "{} AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_country) ORDER BY rd.date_str, fct.value DESC",
                        Self::GET_COUNTRY_METRICS_QUERY
                    )
                } else {
                    format!(
                        "{} AND rd.date_str >= CURRENT_DATE - INTERVAL '{} days' ORDER BY rd.date_str, fct.value DESC",
                        Self::GET_COUNTRY_METRICS_QUERY,
                        req.window
                    )
                };
                let params: Vec<Box<dyn ToSql + Sync>> = vec![Box::new(req.direction.clone())];
                (base_query, params)
            }
        };

        // Convert Box<dyn ToSql> to &[&(dyn ToSql + Sync)] for the query method
        let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| &**p).collect();

        let rows = client
            .query(&query, &param_refs)
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let metrics: Vec<CountryMetric> = rows.iter().map(Self::map_country_metric).collect();

        Ok(serde_json::json!(metrics))
    }

    // Alternative solution 2: Use a helper function for the "top" case
    async fn get_country_metrics_top(
        client: &deadpool_postgres::Client,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let size = req.size.unwrap_or(DEFAULT_SIZE) as i64;
        let params: Vec<&(dyn ToSql + Sync)> = vec![&req.direction, &size];

        let rows = client
            .query(Self::GET_COUNTRY_METRICS_TOP_QUERY, &params)
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let metrics: Vec<CountryMetric> = rows.iter().map(Self::map_country_metric).collect();

        Ok(serde_json::json!(metrics))
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
}
