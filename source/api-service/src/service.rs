use crate::repo;
use core::db::DBManager;
use core::errors::AppError;
use serde::Serialize;

#[derive(Serialize)]
pub struct RoamOutCountResponse {
    pub date: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct OverviewResponse {
    pub last_date: String,
    pub count_roam_in: i64,
    pub count_roam_out: i64,
    pub count_anomalies: i64,
    pub count_notifications: i64,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn overview_service(db: &DBManager) -> Result<OverviewResponse, AppError> {
    let client = db.get_client().await?;
    let last_date = repo::last_date(&client).await?;
    let count_roam_out = repo::last_roam_out(&client).await?;

    Ok(OverviewResponse {
        last_date: last_date,
        count_roam_in: 0,
        count_roam_out: count_roam_out,
        count_anomalies: 0,
        count_notifications: 0,
    })
}

pub async fn health_service() -> HealthResponse {
    HealthResponse {
        status: "Health check passed",
    }
}

/// Fetches the raw `(String, i64)` pairs from repo and wraps them.
pub async fn roamout_by_date_service(
    db: &DBManager,
) -> Result<Vec<RoamOutCountResponse>, AppError> {
    let client = db.get_client().await?;
    let raw = repo::roamout_by_date(&client).await?;
    let wrapped = raw
        .into_iter()
        .map(|(date, count)| RoamOutCountResponse { date, count: count })
        .collect();
    Ok(wrapped)
}
