use crate::service::{self, ErrorResponse};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{HttpResponse, Result, get, web};
use core::db::DBManager;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

const VALID_DIRECTIONS: &[&str] = &["TOTIN", "ACTIN", "OUT"];
const VALID_DIMENSIONS: &[&str] = &["GLOBAL", "COUNTRY", "OPERATOR"];

fn default_kind() -> String {
    "LATEST".to_string()
}

fn default_limit() -> i64 {
    0
}

#[derive(Debug, Deserialize)]
pub struct MetricsQuery {
    pub direction: Option<String>,

    pub dimensions: Option<String>,

    #[serde(default = "default_kind")]
    pub kind: String,

    #[serde(default = "default_limit")]
    pub limit: i64,
}

#[get("/api/v1/health")]
async fn health_check() -> HttpResponse {
    let resp = service::health_service().await;
    HttpResponse::Ok().json(resp)
}

#[get("/api/v1/metrics")]
async fn get_metrics(
    db: web::Data<Arc<DBManager>>,
    params: web::Query<MetricsQuery>,
) -> Result<HttpResponse> {
    // Validate presence
    let direction = match &params.direction {
        Some(d) => d.to_ascii_uppercase(),
        None => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                error: "Missing 'direction' parameter.".into(),
            }));
        }
    };

    let dimensions = match &params.dimensions {
        Some(d) => d.to_ascii_uppercase(),
        None => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                error: "Missing 'dimensions' parameter.".into(),
            }));
        }
    };

    // Validate values
    if !VALID_DIRECTIONS.contains(&direction.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid direction. Must be TOTIN, ACTIN or OUT.".into(),
        }));
    }

    if !VALID_DIMENSIONS.contains(&dimensions.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid dimensions. Must be GLOBAL, COUNTRY or OPERATOR.".into(),
        }));
    }

    // Call service
    let data = service::get_metrics(
        db.as_ref(),
        &direction,
        &dimensions,
        &params.kind,
        params.limit,
    )
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

#[get("/api/v1/notifications/count")]
async fn get_notifications_count(db: web::Data<Arc<DBManager>>) -> Result<HttpResponse> {
    let data = service::get_notifications_count(db.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

#[get("/api/v1/notifications/summary")]
async fn get_notifications_summary(db: web::Data<Arc<DBManager>>) -> Result<HttpResponse> {
    let data = service::get_notifications_summary(db.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

#[get("/api/v1/notifications/details")]
async fn get_notifications_details(db: web::Data<Arc<DBManager>>) -> Result<HttpResponse> {
    let data = service::get_notifications_details(db.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

#[get("/api/v1/alerts/count")]
async fn get_alerts_count(db: web::Data<Arc<DBManager>>) -> Result<HttpResponse> {
    let data = service::get_alerts_count(db.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(get_metrics)
        .service(get_notifications_count)
        .service(get_notifications_summary)
        .service(get_notifications_details)
        .service(get_alerts_count);
}
