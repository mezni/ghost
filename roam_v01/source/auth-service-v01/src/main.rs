mod config;
mod db;
mod errors;
mod handlers;
mod logger;
mod models;
mod repositories;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use config::load_config;
use db::create_pg_pool;
use errors::AppError;
use handlers::init_routes;
use logger::Logger;
use repositories::UserRepository;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // Initialize logger
    Logger::init();
    Logger::info("Starting Auth Service...");

    // Load configuration
    let config = load_config()?;
    let bind_addr = format!("{}:{}", config.service.host, config.service.port);

    // Create database pool
    let pg_pool = create_pg_pool(&config.database)
        .await
        .map_err(|e| AppError::DBError(e.to_string()))?;

    // Initialize repository
    let user_repo = UserRepository::new(pg_pool.clone());

    // Configure CORS
    let cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::CONTENT_TYPE,
            http::header::ACCEPT,
        ])
        .supports_credentials()
        .max_age(3600);

    // Configure allowed origins from config
    let allowed_origins = config.service.allowed_origins
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let cors = if !allowed_origins.is_empty() {
        cors.allowed_origin_fn(move |origin, _req_head| {
            origin.as_bytes().first() == Some(&b'*') ||  // Allow all if wildcard
            allowed_origins.iter().any(|&allowed| {
                origin.to_str().map_or(false, |o| o == allowed)
            })
        })
    } else {
        cors
    };

    Logger::info(&format!("Starting HTTP server at http://{}", bind_addr));
    Logger::info(&format!("Allowed origins: {:?}", allowed_origins));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(cors.clone())
            .wrap(Logger::new())
            .app_data(web::Data::new(user_repo.clone()))
            .configure(init_routes)
    })
    .bind(&bind_addr)
    .map_err(|e| AppError::Other(format!("Failed to bind server: {}", e)))?
    .workers(config.service.worker_count)
    .run()
    .await
    .map_err(|e| AppError::Other(format!("Server run error: {}", e)))
}