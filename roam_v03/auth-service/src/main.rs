use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use crate::config::Config;
use crate::errors::AppError;
use crate::logger::Logger;

mod config;
mod errors;
mod logger;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info("Starting authentication service...");

    let config = Config::from_env()?;

    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    Logger::info(&format!("Listening on http://{}", &bind_addr));

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
        // Add your routes, middleware, etc.
    })
    .bind(bind_addr)
    .map_err(|e| {
        Logger::error(&format!("Failed to bind server: {}", e));
        AppError::IoError(e)
    })?
    .run()
    .await
    .map_err(|e| {
        Logger::error(&format!("Server error: {}", e));
        AppError::InternalServerError
    })
}
