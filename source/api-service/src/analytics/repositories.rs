use crate::analytics::models::{CountryMetric, GlobalMetric, MetricRequest};
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

        let rows = client
            .query(
                "
                SELECT dd.date_str as date, mg.value as value
                FROM metrics_global mg 
                JOIN dim_dates dd ON mg.date_id = dd.date_id
                JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
                JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
                JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
                WHERE crd.direction = $1
                AND cmt.name = 'GLOBAL'
                ORDER BY mg.date_id
                ",
                &[
                    &req.dataset.direction,
                ],
            )
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| Self::map_global_metric(row))
            .collect())
    }

    // -------------------- Country --------------------
    pub async fn get_country_metrics(
        pool: &Pool,
        req: &MetricRequest,
    ) -> Result<Vec<CountryMetric>, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let rows = client
            .query(
                "
                SELECT d.date, c.country_name, o.operator_name, SUM(m.value) AS value
                FROM metrics_country m
                JOIN dim_dates d ON d.date_id = m.date_id
                JOIN dim_countries c ON c.country_id = m.country_id
                JOIN dim_operators o ON o.operator_id = m.operator_id
                JOIN cfg_metric_definitions def ON def.metric_definition_id = m.metric_definition_id
                WHERE def.direction = $1
                AND ($2::DATE IS NULL OR d.date >= $2::DATE)
                AND ($3::DATE IS NULL OR d.date <= $3::DATE)
                AND ($4::TEXT IS NULL OR c.country_name = $4::TEXT)
                GROUP BY d.date, c.country_name, o.operator_name
                ORDER BY d.date
                ",
                &[
                    &req.dataset.direction,
                    &req.timePeriod.from,
                    &req.timePeriod.to,
                    &req.filter.country,
                ],
            )
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| Self::map_country_metric(row))
            .collect())
    }

    // -------------------- Operator --------------------
    pub async fn get_operator_metrics(
        pool: &Pool,
        req: &MetricRequest,
    ) -> Result<Vec<CountryMetric>, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let rows = client
            .query(
                "
                SELECT d.date, c.country_name, o.operator_name, SUM(m.value) AS value
                FROM metrics_operator m
                JOIN dim_dates d ON d.date_id = m.date_id
                JOIN dim_countries c ON c.country_id = m.country_id
                JOIN dim_operators o ON o.operator_id = m.operator_id
                JOIN cfg_metric_definitions def ON def.metric_definition_id = m.metric_definition_id
                WHERE def.direction = $1
                AND ($2::DATE IS NULL OR d.date >= $2::DATE)
                AND ($3::DATE IS NULL OR d.date <= $3::DATE)
                AND ($4::TEXT IS NULL OR c.country_name = $4::TEXT)
                AND ($5::TEXT IS NULL OR o.operator_name = $5::TEXT)
                GROUP BY d.date, c.country_name, o.operator_name
                ORDER BY d.date
                ",
                &[
                    &req.dataset.direction,
                    &req.timePeriod.from,
                    &req.timePeriod.to,
                    &req.filter.country,
                    &req.filter.operator,
                ],
            )
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| Self::map_country_metric(row))
            .collect())
    }

    // -------------------- Subscriber --------------------
    pub async fn get_subscriber_metrics(
        pool: &Pool,
        req: &MetricRequest,
    ) -> Result<Vec<GlobalMetric>, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        let rows = client
            .query(
                "
                SELECT d.date, SUM(m.value) AS value
                FROM metrics_subscriber m
                JOIN dim_dates d ON d.date_id = m.date_id
                JOIN cfg_metric_definitions def ON def.metric_definition_id = m.metric_definition_id
                WHERE def.direction = $1
                AND ($2::DATE IS NULL OR d.date >= $2::DATE)
                AND ($3::DATE IS NULL OR d.date <= $3::DATE)
                AND ($4::TEXT IS NULL OR m.subscriber = $4::TEXT)
                GROUP BY d.date
                ORDER BY d.date
                ",
                &[
                    &req.dataset.direction,
                    &req.timePeriod.from,
                    &req.timePeriod.to,
                    &req.filter.subscriber,
                ],
            )
            .await
            .map_err(|e| AppError::db_error(&e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| Self::map_global_metric(row))
            .collect())
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
            country: row.get("country_name"),
            operator: row.get("operator_name"),
            value: row.get("value"),
        }
    }
}
