use crate::repo;
use core::db::DBManager;
use core::errors::AppError;
use serde::Serialize;

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

pub async fn overview_service(_db: &DBManager) -> Result<OverviewResponse, AppError> {
    Ok(OverviewResponse {
        last_date: "2025-04-11".to_string(),
        count_roam_in: 12,
        count_roam_out: 123,
        count_anomalies: 40,
        count_notifications: 55,
    })
}

pub async fn health_service() -> HealthResponse {
    HealthResponse {
        status: "Health check passed",
    }
}
