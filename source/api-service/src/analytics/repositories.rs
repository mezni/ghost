use crate::analytics::models::{CountryMetric, GlobalMetric};
use crate::core::errors::AppError;
use deadpool_postgres::Pool;

pub struct MetricRepository;

impl MetricRepository {
    // ---------- GLOBAL ----------
    pub async fn fetch_global_in_metrics(pool: &Pool) -> Result<Vec<GlobalMetric>, AppError> {
        Self::fetch_global_metrics_by_direction(pool, "IN").await
    }

    pub async fn fetch_global_out_metrics(pool: &Pool) -> Result<Vec<GlobalMetric>, AppError> {
        Self::fetch_global_metrics_by_direction(pool, "OUT").await
    }

    async fn fetch_global_metrics_by_direction(
        pool: &Pool,
        direction: &str,
    ) -> Result<Vec<GlobalMetric>, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::Other(format!("Database error: {}", e)))?;
        let stmt = client
            .prepare(&format!(
                "
            SELECT dd.date_str AS date, mg.value
            FROM metrics_global mg
            JOIN dim_dates dd ON mg.date_id = dd.date_id
            JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
            JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
            JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
            WHERE crd.direction = '{}'
              AND cmt.name = 'GLOBAL'
            ORDER BY mg.date_id;
        ",
                direction
            ))
            .await
            .map_err(|e| AppError::Other(format!("Database error: {}", e)))?;

        let rows = client
            .query(&stmt, &[])
            .await
            .map_err(|e| AppError::Other(format!("Database error: {}", e)))?;
        Ok(rows
            .iter()
            .map(|row| GlobalMetric {
                date: row.get("date"),
                value: row.get("value"),
            })
            .collect())
    }

    // ---------- COUNTRY ----------
    pub async fn fetch_country_in_metrics(pool: &Pool) -> Result<Vec<CountryMetric>, AppError> {
        Self::fetch_country_metrics_by_direction(pool, "IN").await
    }

    pub async fn fetch_country_out_metrics(pool: &Pool) -> Result<Vec<CountryMetric>, AppError> {
        Self::fetch_country_metrics_by_direction(pool, "OUT").await
    }

    async fn fetch_country_metrics_by_direction(
        pool: &Pool,
        direction: &str,
    ) -> Result<Vec<CountryMetric>, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::Other(format!("Database error: {}", e)))?;
        let stmt = client
            .prepare(&format!(
                "
            SELECT dd.date_str AS date,
                   dc.country_name AS country,
                   mg.value
            FROM metrics_global mg
            JOIN dim_dates dd ON mg.date_id = dd.date_id
            JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
            JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
            JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
            JOIN dim_countries dc ON cmd.country_id = dc.country_id
            WHERE crd.direction = '{}'
              AND cmt.name = 'GLOBAL'
            ORDER BY mg.date_id, dc.country_name;
        ",
                direction
            ))
            .await
            .map_err(|e| AppError::Other(format!("Database error: {}", e)))?;

        let rows = client
            .query(&stmt, &[])
            .await
            .map_err(|e| AppError::Other(format!("Database error: {}", e)))?;
        Ok(rows
            .iter()
            .map(|row| CountryMetric {
                date: row.get("date"),
                country: row.get("country"),
                value: row.get("value"),
            })
            .collect())
    }
}
