mod errors;
mod infra;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use errors::AppError;
use infra::config::load_config;
use infra::logger::Logger;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logger
    Logger::init();
    Logger::info("Starting service...");

    // Load configuration
    let config = load_config()?;
    Logger::info("Configuration loaded successfully.");
    Logger::debug(&format!("Loaded config: {:?}", config));

    let host = config.service.host.clone();
    let port = config.service.port;

    Logger::info(&format!("Starting HTTP server at http://{}:{}", host, port));

    HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .bind((host.as_str(), port as u16))?
        .run()
        .await
        .map_err(Into::into) // convert Actix error to AppError if needed
}
