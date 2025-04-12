use crate::service::test_service; // Import test_service function
use actix_web::{HttpResponse, Responder, get, web};
use core::db::DBManager;
use std::sync::Arc;

// Health check endpoint that checks if the server is up
#[get("/health")]
async fn health_check() -> impl Responder {
    // You could add a DB check here if needed
    HttpResponse::Ok().body("Health check passed")
}

// Test endpoint that returns a count of rows from the DB
#[get("/test")]
async fn test_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match test_service(db.as_ref()).await {
        Ok(count) => HttpResponse::Ok().body(format!("Count: {}", count)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

// Add the health check and test endpoints to the service config
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check).service(test_endpoint); // Register the health check endpoint
}
