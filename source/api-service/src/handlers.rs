use crate::service::{
    ErrorResponse, HealthResponse, RoamOutCountResponse, health_service, overview_service,
    roamout_by_country_service, roamout_by_date_service,
};
use actix_web::{HttpResponse, Responder, get, web};
use core::db::DBManager;
use serde_json::json;
use std::sync::Arc;

#[get("/api/v1/health")]
async fn health_check() -> impl Responder {
    let resp = health_service().await;
    let body = HealthResponse {
        status: resp.status.to_string(),
    };
    HttpResponse::Ok().json(body)
}

#[get("/api/v1/overview")]
async fn overview_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match overview_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to fetch overview".to_string(),
        }),
    }
}

#[get("/api/v1/roamout-by-date")]
async fn roamout_by_date_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match roamout_by_date_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to fetch roamout by date".to_string(),
        }),
    }
}

#[get("/api/v1/roamout-by-country")]
async fn roamout_by_country_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match roamout_by_country_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to fetch roamout by country".to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(overview_endpoint)
        .service(roamout_by_date_endpoint)
        .service(roamout_by_country_endpoint);
}
