mod errors;
mod infra;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger as ActixLogger};
use infra::{config::load_config, db::DBManager, logger::Logger};
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber logger (your Logger wrapper)
    Logger::init();
    Logger::info("Starting API server...");

    // Load config
    let config = match load_config() {
        Ok(cfg) => {
            Logger::info("Configuration loaded successfully.");
            cfg
        }
        Err(e) => {
            Logger::error(&format!("Failed to load configuration: {}", e));
            std::process::exit(1);
        }
    };

    // Initialize database manager with the loaded config
    let db_manager = match DBManager::new(config.database.clone()) {
        Ok(manager) => {
            Logger::info("Database pool created.");
            Arc::new(manager)
        }
        Err(e) => {
            Logger::error(&format!("Failed to initialize DB pool: {}", e));
            std::process::exit(1);
        }
    };

    let bind_addr = format!("{}:{}", config.service.host, config.service.port);
    Logger::info(&format!("Listening on http://{}", bind_addr));

    HttpServer::new(move || {
        // Build CORS inside the closure to avoid Send/Clone errors
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials();

        App::new()
            .wrap(ActixLogger::default())
            .wrap(cors)
            .app_data(actix_web::web::Data::from(db_manager.clone()))
        // configure your routes here
    })
    .bind(bind_addr)?
    .run()
    .await
}
