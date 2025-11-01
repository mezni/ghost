use crate::analytics::models::{CountryMetric, Filter, NotifMetric, ValidatedMetricsRequest};
use crate::core::errors::AppError;
use serde_json::json;
use sqlx::{PgPool, Row};

pub struct MetricsRepository;

impl MetricsRepository {
    const GET_GLOBAL_METRICS_QUERY: &str = r#"
        SELECT rd.date_str AS date, fct.value
        FROM trx_metrics_global fct
        JOIN ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
        JOIN ref_dates rd ON rd.date_id = fct.date_id
        WHERE (fct.date_id, fct.batch_id) IN (
            SELECT date_id, MAX(batch_id) AS max_batch_id
            FROM trx_metrics_global
            GROUP BY date_id
        )
        AND UPPER(rrd.direction) = UPPER($1)
    "#;

    const GET_COUNTRY_METRICS_QUERY: &str = r#"
        SELECT rd.date_str AS date, cc.country_name AS country, fct.value
        FROM trx_metrics_country fct
        JOIN ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
        JOIN ref_dates rd ON rd.date_id = fct.date_id
        JOIN cfg_countries cc ON cc.country_id = fct.country_id
        WHERE (fct.date_id, fct.batch_id) IN (
            SELECT date_id, MAX(batch_id) AS max_batch_id
            FROM trx_metrics_country
            GROUP BY date_id
        )
        AND UPPER(rrd.direction) = UPPER($1)
    "#;

    const GET_COUNTRY_METRICS_TOP_QUERY: &str = r#"
        SELECT date, country, SUM(value)::bigint AS value
        FROM (
            SELECT 
                rd.date_str AS date,
                CASE 
                    WHEN ROW_NUMBER() OVER (PARTITION BY rd.date_str ORDER BY fct.value DESC) <= CAST ($2 AS INT)
                    THEN cc.country_name 
                    ELSE 'Others'
                END AS country,
                fct.value
            FROM trx_metrics_country fct
            JOIN ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
            JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
            JOIN ref_dates rd ON rd.date_id = fct.date_id
            JOIN cfg_countries cc ON cc.country_id = fct.country_id
            WHERE (fct.date_id, fct.batch_id) IN (
                SELECT date_id, MAX(batch_id) AS max_batch_id
                FROM trx_metrics_country
                GROUP BY date_id
            )
            AND UPPER(rrd.direction) = UPPER($1)
            AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_country)
        ) AS ranked
        GROUP BY date, country
        ORDER BY value DESC
    "#;

    const GET_NOTIF_DET_METRICS_QUERY: &str = r#"
        SELECT rd.date_str AS date, fct.message AS value
        FROM trx_notifications fct
        JOIN ref_dates rd ON rd.date_id = fct.date_id
        WHERE fct.date_id = (SELECT MAX(date_id) FROM trx_notifications)
    "#;

    const GET_NOTIF_SUM_METRICS_QUERY: &str = r#"
        SELECT rd.date_str AS date, COUNT(*)::text AS value
        FROM trx_notifications fct
        JOIN ref_dates rd ON rd.date_id = fct.date_id
        WHERE fct.date_id = (SELECT MAX(date_id) FROM trx_notifications)
        GROUP BY rd.date_str
    "#;

    // Main dispatcher
    pub async fn get_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        match req.dimension.to_lowercase().as_str() {
            "global" => Self::get_global_metrics(pool, req).await,
            "country" => Self::get_country_metrics(pool, req).await,
            "notification" => Self::get_notif_metrics(pool, req).await,
            _ => Err(AppError::BadRequest("Invalid dimension".to_string())),
        }
    }

    // ------------------- Global -------------------
    async fn get_global_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let direction = get_direction_from_filters(req.filter.as_ref())?;
        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;

        let query = match aggregation.as_str() {
            "latest" => format!(
                "{} AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_global) ORDER BY rd.date_str",
                Self::GET_GLOBAL_METRICS_QUERY
            ),
            "history" => format!(
                "{} AND rd.date >= CURRENT_DATE - INTERVAL '{} days' ORDER BY rd.date_str",
                Self::GET_GLOBAL_METRICS_QUERY,
                size
            ),
            _ => {
                return Err(AppError::BadRequest(
                    "Aggregation 'latest' or 'history' is required".to_string(),
                ));
            }
        };

        let rows = sqlx::query(&query)
            .bind(direction)
            .fetch_all(pool)
            .await
            .map_err(AppError::Sqlx)?;

        let mut metrics = Vec::new();
        for row in rows {
            let date: String = row.try_get("date")?;
            let value: i64 = row.try_get("value")?;
            metrics.push(json!({ "date": date, "value": value }));
        }

        Ok(json!({ "data": metrics, "status": "success" }))
    }

    // ------------------- Country -------------------
    async fn get_country_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let direction = get_direction_from_filters(req.filter.as_ref())?;
        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;
        let country = get_country_from_filters(req.filter.as_ref());

        match aggregation.as_str() {
            "latest" | "history" => {
                let mut query = Self::GET_COUNTRY_METRICS_QUERY.to_string();
                if !country.is_empty() {
                    query.push_str(" AND UPPER(cc.country_name) = UPPER($2)");
                }
                if aggregation == "latest" {
                    query.push_str(
                        " AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_country)",
                    );
                } else {
                    query.push_str(&format!(
                        " AND rd.date >= CURRENT_DATE - INTERVAL '{} days'",
                        size
                    ));
                }
                query.push_str(" ORDER BY rd.date_str");

                let mut q = sqlx::query(&query).bind(direction.clone());
                if !country.is_empty() {
                    q = q.bind(country);
                }

                let rows = q.fetch_all(pool).await.map_err(AppError::Sqlx)?;
                let mut metrics = Vec::new();
                for row in rows {
                    let date: String = row.try_get("date")?;
                    let country: String = row.try_get("country")?;
                    let value: i64 = row.try_get("value")?;
                    metrics.push(json!({ "date": date, "country": country, "value": value }));
                }

                Ok(json!({ "data": metrics, "status": "success" }))
            }

            "top" => {
                let rows = sqlx::query(Self::GET_COUNTRY_METRICS_TOP_QUERY)
                    .bind(direction)
                    .bind(size)
                    .fetch_all(pool)
                    .await
                    .map_err(AppError::Sqlx)?;

                let mut metrics = Vec::new();
                for row in rows {
                    let date: String = row.try_get("date")?;
                    let country: String = row.try_get("country")?;
                    let value: i64 = row.try_get("value")?;
                    metrics.push(json!({ "date": date, "country": country, "value": value }));
                }

                Ok(json!({ "data": metrics, "status": "success" }))
            }

            _ => Err(AppError::BadRequest(
                "Aggregation 'latest', 'history' or 'top' is required".to_string(),
            )),
        }
    }

    // ------------------- Notification -------------------
    async fn get_notif_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let aggregation = &req.aggregation;
        let query = match aggregation.as_str() {
            "summary" => Self::GET_NOTIF_SUM_METRICS_QUERY,
            "detail" => Self::GET_NOTIF_DET_METRICS_QUERY,
            _ => {
                return Err(AppError::BadRequest(
                    "Aggregation 'summary' or 'detail' is required".to_string(),
                ));
            }
        };

        let rows = sqlx::query(query)
            .fetch_all(pool)
            .await
            .map_err(AppError::Sqlx)?;
        let mut metrics = Vec::new();
        for row in rows {
            let date: String = row.try_get("date")?;
            let value: String = row.try_get("value")?;
            metrics.push(json!({ "date": date, "value": value }));
        }

        Ok(json!({ "data": metrics, "status": "success" }))
    }
}

// ------------------- Filter Helpers -------------------
fn get_direction_from_filters(filters: Option<&Vec<Filter>>) -> Result<String, AppError> {
    if let Some(filters) = filters {
        let dir = filters
            .iter()
            .find_map(|f| {
                if f.key.to_lowercase() == "direction" {
                    Some(f.value.clone())
                } else {
                    None
                }
            })
            .ok_or(AppError::BadRequest("Direction is required".to_string()))?;

        if ["in", "out"].contains(&dir.to_lowercase().as_str()) {
            Ok(dir)
        } else {
            Err(AppError::BadRequest(
                "Direction must be 'in' or 'out'".to_string(),
            ))
        }
    } else {
        Err(AppError::BadRequest("Direction is required".to_string()))
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
            .find_map(|f| {
                if f.key.to_lowercase() == "country" {
                    Some(f.value.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    } else {
        "".to_string()
    }
}
