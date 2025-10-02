use crate::analytics::models::{GlobalMetric, MetricRequest};
use crate::core::errors::AppError;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

pub struct MetricsRepository;

impl MetricsRepository {
    // -------------------- Global --------------------
    pub async fn get_global_metrics(
        pool: &Pool,
        req: &MetricRequest,
    ) -> Result<Vec<GlobalMetric>, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let rows = if req.timePeriod.window == 0 {
            // Latest date only
            client
                .query(
                    "
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
                    ",
                    &[&req.dataset.direction],
                )
                .await
                .map_err(|e| AppError::db_error(&e.to_string()))?
        } else {
            // Use interval syntax with proper binding
            client
                .query(
                    "
                    SELECT dd.date_str AS date, mg.value AS value
                    FROM metrics_global mg
                    JOIN dim_dates dd ON mg.date_id = dd.date_id
                    JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
                    JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
                    JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
                    WHERE crd.direction = $1
                    AND cmt.name = 'GLOBAL'
                    AND dd.date >= CURRENT_DATE - make_interval(days => $2)
                    ORDER BY dd.date_str
                    ",
                    &[&req.dataset.direction, &req.timePeriod.window],
                )
                .await
                .map_err(|e| AppError::db_error(&e.to_string()))?
        };

        Ok(rows.iter().map(Self::map_global_metric).collect())
    }

    // -------------------- Mapping --------------------
    fn map_global_metric(row: &Row) -> GlobalMetric {
        GlobalMetric {
            date: row.get("date"),
            value: row.get("value"),
        }
    }
}
