use crate::analytics::models::{CountryMetric, GlobalMetric};
use crate::core::errors::AppError;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

pub struct MetricsRepository;

impl MetricsRepository {
    pub async fn get_global_metrics(
        pool: &Pool,
        _aggregation: String,
        _granularity: String,
        _window: i32,
        _from: Option<String>,
        _to: Option<String>,
    ) -> Result<Vec<GlobalMetric>, AppError> {
        let client = pool.get().await?;
        let rows = client
            .query("SELECT '2025-09-01' as date, 100 as value", &[])
            .await?;

        let metrics: Vec<GlobalMetric> = rows.iter().map(|row| Self::map_global(row)).collect();

        Ok(metrics)
    }

    pub async fn get_country_metrics(
        pool: &Pool,
        _aggregation: String,
        _granularity: String,
        _window: i32,
        _from: Option<String>,
        _to: Option<String>,
        country: Option<String>, // ✅ corrected spelling
        operator: Option<String>,
        _subscriber: Option<String>,
    ) -> Result<Vec<CountryMetric>, AppError> {
        let client = pool.get().await?;

        let rows = client
            .query(
                "SELECT '2025-09-01' as date, $1::text as country, $2::text as operator, 200 as value",
                &[&country.unwrap_or("Unknown".into()), &operator.unwrap_or("Unknown".into())],
            )
            .await?;

        let metrics: Vec<CountryMetric> = rows.iter().map(|row| Self::map_country(row)).collect();

        Ok(metrics)
    }

    fn map_global(row: &Row) -> GlobalMetric {
        GlobalMetric {
            date: row.get("date"),
            value: row.get("value"),
        }
    }

    fn map_country(row: &Row) -> CountryMetric {
        CountryMetric {
            date: row.get("date"),
            country: row.get("country"),
            operator: row.get("operator"),
            value: row.get("value"),
        }
    }
}
