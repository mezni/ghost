use crate::service::{health_service, overview_service, roam_out_counts_service};
use actix_web::{HttpResponse, Responder, get, web};
use core::db::DBManager;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(serde::Serialize)]
pub struct RoamOutCountResponse {
    pub date: String,
    pub count: i64,
}

// Health check endpoint
#[get("/health")]
async fn health_check() -> impl Responder {
    // call your health_service (if you want to do more than a static string)
    let resp = health_service().await;
    let body = HealthResponse {
        status: resp.status.to_string(),
    };
    HttpResponse::Ok().json(body)
}

// Overview endpoint returning wrapped data
#[get("/overview")]
async fn overview_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match overview_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("{:?}", e),
        }),
    }
}

#[get("/roam-out-counts")]
async fn roam_out_counts_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match roam_out_counts_service(db.as_ref()).await {
        Ok(data) => {
            // wraps the Vec<RoamOutCountResponse> under "data"
            HttpResponse::Ok().json(json!({ "data": data }))
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("{:?}", e),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(overview_endpoint)
        .service(roam_out_counts_endpoint);
}
