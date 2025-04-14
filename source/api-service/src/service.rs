use crate::repo;
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
pub struct RoamOutCountResponse {
    pub date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

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

pub async fn overview_service(db: &DBManager) -> Result<OverviewResponse, AppError> {
    let client = db.get_client().await?;
    let last_date = repo::last_date(&client).await?;
    let count_roam_out = repo::last_roam_out(&client).await?;
    let count_roam_in = repo::count_roam_in(&client).await?;

    Ok(OverviewResponse {
        last_date: last_date,
        count_roam_in,
        count_roam_out,
        count_anomalies: 0,     // Add logic here for anomalies if needed
        count_notifications: 0, // Add logic here for notifications if needed
    })
}

pub async fn health_service() -> HealthResponse {
    HealthResponse {
        status: "Health check passed".to_string(),
    }
}

pub async fn roamout_by_date_service(
    db: &DBManager,
) -> Result<Vec<RoamOutCountResponse>, AppError> {
    let client = db.get_client().await?;
    let raw = repo::roamout_by_date(&client).await?;

    // Map the raw data to RoamOutCountResponse
    let wrapped = raw
        .into_iter()
        .map(|(date, count)| RoamOutCountResponse {
            date,
            country: None,
            count,
        }) // country is None for date-based data
        .collect();

    Ok(wrapped)
}

pub async fn roamout_by_country_service(
    db: &DBManager,
) -> Result<Vec<RoamOutCountResponse>, AppError> {
    let client = db.get_client().await?;
    let raw = repo::roamout_by_country(&client).await?;

    // Map the raw data to RoamOutCountResponse
    let wrapped = raw
        .into_iter()
        .map(|(date, country, count)| RoamOutCountResponse {
            date,
            country: Some(country), // Include the country in the response
            count,
        })
        .collect();

    Ok(wrapped)
}
