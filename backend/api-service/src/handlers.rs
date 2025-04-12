use crate::service::test_service;
use actix_web::{HttpResponse, Responder, get, web};
use core::db::DBManager;
use std::sync::Arc; // import test_service function

#[get("/test")]
async fn test_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    // Call test_service with DBManager directly
    match test_service(db.as_ref()).await {
        Ok(count) => HttpResponse::Ok().body(format!("Count: {}", count)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(test_endpoint);
}
