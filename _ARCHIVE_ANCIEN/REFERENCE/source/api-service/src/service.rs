use crate::repo;
use chrono::{Duration, NaiveDate, Utc};
use core::db::DBManager;
use core::errors::AppError;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    pub date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,

    pub count: i32,
}

#[derive(Serialize)]
pub struct NotificationResponse {
    pub date: String,
    pub rule: String,
    pub count: i32,
}

#[derive(Serialize)]
pub struct NotificationCountResponse {
    pub count: i32,
}

#[derive(Serialize)]
pub struct AlertCountResponse {
    pub count: i32,
}

#[derive(Serialize)]
pub struct NotificationDetailResponse {
    pub operator: String,
    pub perct_configure: String,
    pub perct_reel: String,
}

pub async fn health_service() -> HealthResponse {
    HealthResponse {
        status: "Healthy".to_string(),
    }
}

fn resolve_date_range(
    start_date: Option<&str>,
    end_date: Option<&str>,
    max_date: NaiveDate,
) -> Result<(NaiveDate, NaiveDate), AppError> {
    match (start_date, end_date) {
        (Some(s), Some(e)) => Ok((
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|e| AppError::Unexpected(format!("Invalid start date: {}", e)))?,
            NaiveDate::parse_from_str(e, "%Y-%m-%d")
                .map_err(|e| AppError::Unexpected(format!("Invalid end date: {}", e)))?,
        )),
        (Some(s), None) => {
            let sd = NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|e| AppError::Unexpected(format!("Invalid start date: {}", e)))?;
            Ok((sd, max_date))
        }
        (None, Some(e)) => {
            let ed = NaiveDate::parse_from_str(e, "%Y-%m-%d")
                .map_err(|e| AppError::Unexpected(format!("Invalid end date: {}", e)))?;
            Ok((ed - Duration::days(10), ed))
        }
        (None, None) => Ok((max_date - Duration::days(10), max_date)),
    }
}

pub async fn get_metrics(
    db: &DBManager,
    direction: &str,
    dimensions: &str,
    kind: &str,
    limit: i64,
) -> Result<Vec<MetricsResponse>, AppError> {
    let db_client = db.get_client().await?;

    let rows = repo::get_metrics(&db_client, direction, dimensions, kind, limit).await?;

    let metrics = rows
        .into_iter()
        .map(|(date, country, operator, count)| MetricsResponse {
            date,
            country,
            operator,
            count,
        })
        .collect();

    Ok(metrics)
}

pub async fn get_notifications_summary(
    db: &DBManager,
) -> Result<Vec<NotificationResponse>, AppError> {
    let db_client = db.get_client().await?;

    let rows = repo::get_notifications_summary(&db_client).await?;

    let notifications = rows
        .into_iter()
        .map(|(date, rule, count)| NotificationResponse { date, rule, count })
        .collect();

    Ok(notifications)
}

pub async fn get_notifications_details(
    db: &DBManager,
) -> Result<Vec<NotificationDetailResponse>, AppError> {
    let db_client = db.get_client().await?;

    let rows = repo::get_notifications_details(&db_client).await?;

    let notifications = rows
        .into_iter()
        .map(
            |(operator, perct_configure, perct_reel)| NotificationDetailResponse {
                operator,
                perct_configure,
                perct_reel,
            },
        )
        .collect();

    Ok(notifications)
}

pub async fn get_notifications_count(
    db: &DBManager,
) -> Result<Vec<NotificationCountResponse>, AppError> {
    // Acquire the database client
    let db_client = db.get_client().await?;

    // Fetch raw data from the repository
    let counts = repo::get_notifications_count(&db_client).await?;

    // Transform into response struct (more efficient with `into_iter`)
    let notifications = counts
        .into_iter()
        .map(|count| NotificationCountResponse { count })
        .collect();

    Ok(notifications)
}


pub async fn get_alerts_count(
    db: &DBManager,
) -> Result<Vec<AlertCountResponse>, AppError> {
    // Acquire the database client
    let db_client = db.get_client().await?;

    // Fetch raw data from the repository
    let counts = repo::get_alerts_count(&db_client).await?;

    // Transform into response struct (more efficient with `into_iter`)
    let alerts = counts
        .into_iter()
        .map(|count| AlertCountResponse { count })
        .collect();

    Ok(alerts)
}