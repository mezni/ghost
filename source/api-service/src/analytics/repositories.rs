use crate::analytics::models::{CountryMetric, GlobalMetric, MetricsRequest};
use crate::core::errors::AppError;
use deadpool_postgres::Pool;
use serde_json::json;
use tokio_postgres::Row;

pub struct MetricsRepository;

impl MetricsRepository {
    // SQL Queries as constants
    const GLOBAL_QUERY_LATEST: &str = "
        SELECT dd.date_str AS date, mg.value AS value
        FROM metrics_global mg
        JOIN dim_dates dd ON mg.date_id = dd.date_id
        JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
        JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
        JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
        WHERE crd.direction = $1
        AND cmt.name = 'GLOBAL'
        AND dd.date_id = (SELECT max(date_id) FROM metrics_global)
        ORDER BY mg.date_id
    ";

    const GLOBAL_QUERY_WINDOW: &str = "
        SELECT dd.date_str AS date, mg.value AS value
        FROM metrics_global mg
        JOIN dim_dates dd ON mg.date_id = dd.date_id
        JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
        JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
        JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
        WHERE crd.direction = $1
        AND cmt.name = 'GLOBAL'
        AND dd.date >= CURRENT_DATE - make_interval(days =>  $2)
        ORDER BY dd.date_str
    ";

    const COUNTRY_QUERY_LATEST: &str = "
        SELECT dd.date_str AS date, dc.country_name AS country, mc.value AS value
        FROM metrics_country mc
        JOIN dim_dates dd ON mc.date_id = dd.date_id
        JOIN cfg_metric_definitions cmd ON mc.metric_definition_id = cmd.metric_definition_id
        JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
        JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
        JOIN dim_countries dc ON dc.country_id = mc.country_id
        WHERE crd.direction = $1
        AND cmt.name = 'COUNTRY'
        AND dd.date_id = (SELECT max(date_id) FROM metrics_country)
        ORDER BY mc.date_id, dc.country_name
    ";

    const COUNTRY_QUERY_WINDOW: &str = "
        SELECT dd.date_str AS date, dc.country_name AS country, mc.value AS value
        FROM metrics_country mc
        JOIN dim_dates dd ON mc.date_id = dd.date_id
        JOIN cfg_metric_definitions cmd ON mc.metric_definition_id = cmd.metric_definition_id
        JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
        JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
        JOIN dim_countries dc ON dc.country_id = mc.country_id
        WHERE crd.direction = $1
        AND cmt.name = 'COUNTRY'
        AND dd.date >= CURRENT_DATE - make_interval(days => $2)
        ORDER BY mc.date_id, dc.country_name
    ";

    const COUNTRY_QUERY_TOP: &str = "
    SELECT 
        date,
        country,
        SUM(value)::bigint AS value
    FROM (
        SELECT 
            dd.date_str AS date, 
            CASE 
                WHEN ROW_NUMBER() OVER (PARTITION BY dd.date_str ORDER BY mc.value DESC) <= CAST($2 AS integer)
                THEN dc.country_name 
                ELSE 'Others' 
            END AS country,
            mc.value
        FROM metrics_country mc
        JOIN dim_dates dd ON mc.date_id = dd.date_id
        JOIN cfg_metric_definitions cmd ON mc.metric_definition_id = cmd.metric_definition_id
        JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
        JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
        JOIN dim_countries dc ON dc.country_id = mc.country_id
        WHERE crd.direction = $1::text
        AND cmt.name = 'COUNTRY'
        AND dd.date_id = (SELECT max(date_id) FROM metrics_global)
    ) AS ranked
    GROUP BY date, country
    ORDER BY value DESC
";

    pub async fn get_metrics(
        pool: &Pool,
        req: &MetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let _client = pool
            .get()
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        // Validate metric type: Metric or Definition
        if req.metric != "metric" && req.metric != "definition" {
            return Err(AppError::bad_request("Invalid metric type"));
        }

        // Validate dimension and direction
        let dimension = req.dimension.to_lowercase();
        let direction = req.direction.to_uppercase();

        if !matches!(
            dimension.as_str(),
            "global" | "country" | "operator" | "subscriber"
        ) {
            return Err(AppError::bad_request("Invalid dimension"));
        }

        if !matches!(direction.as_str(), "IN" | "OUT") {
            return Err(AppError::bad_request(
                "Invalid direction: must be IN or OUT",
            ));
        }

        // Select the appropriate query based on the dimension
        let result_json = match dimension.as_str() {
            "global" => MetricsRepository::get_global_metrics(pool, req).await?,
            "country" => MetricsRepository::get_country_metrics(pool, req).await?,
            _ => return Err(AppError::bad_request("Unsupported dimension")),
        };

        Ok(result_json)
    }

    // -------------------- Global Metrics --------------------
    pub async fn get_global_metrics(
        pool: &Pool,
        req: &MetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let _client = pool
            .get()
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let query = if let Some(window) = req.timeWindow {
            Self::GLOBAL_QUERY_WINDOW
        } else {
            Self::GLOBAL_QUERY_LATEST
        };

        let rows = if let Some(window) = req.timeWindow {
            _client
                .query(query, &[&req.direction, &(window as i32)]) // Use &req.direction directly (String works)
                .await
                .map_err(|e| AppError::db_error(&e.to_string()))?
        } else {
            _client
                .query(query, &[&req.direction]) // Use &req.direction directly
                .await
                .map_err(|e| AppError::db_error(&e.to_string()))?
        };

        Ok(serde_json::json!(
            rows.iter()
                .map(Self::map_global_metric)
                .collect::<Vec<GlobalMetric>>()
        ))
    }

    // -------------------- Country Metrics --------------------
    pub async fn get_country_metrics(
        pool: &Pool,
        req: &MetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        let _client = pool
            .get()
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        // Check if there's a time window or an aggregation size
        let query = if let Some(_window) = req.timeWindow {
            Self::COUNTRY_QUERY_WINDOW
        } else {
            if let Some(aggregation) = &req.aggregation {
                if let Some(aggregation_size) = aggregation.size {
                    Self::COUNTRY_QUERY_TOP
                } else {
                    Self::COUNTRY_QUERY_LATEST
                }
            } else {
                Self::COUNTRY_QUERY_LATEST
            }
        };

        // Execute the query based on the determined query type
        let rows = if query == Self::COUNTRY_QUERY_TOP {
            let aggregation_size = req
                .aggregation
                .as_ref()
                .and_then(|agg| agg.size)
                .unwrap_or(5);
            _client
                .query(query, &[&req.direction, &(aggregation_size as i32)]) // Use i32 for aggregation size
                .await
                .map_err(|e| AppError::db_error(&e.to_string()))?
        } else if query == Self::COUNTRY_QUERY_WINDOW {
            _client
                .query(
                    query,
                    &[&req.direction, &(req.timeWindow.unwrap_or(0) as i32)], // Use i32 for window
                )
                .await
                .map_err(|e| AppError::db_error(&e.to_string()))?
        } else {
            // For COUNTRY_QUERY_LATEST
            _client
                .query(query, &[&req.direction])
                .await
                .map_err(|e| AppError::db_error(&e.to_string()))?
        };

        Ok(serde_json::json!(
            rows.iter()
                .map(Self::map_country_metric)
                .collect::<Vec<CountryMetric>>()
        ))
    }

    // -------------------- Mapping --------------------
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
