mod config;
mod db;
mod errors;
mod handlers;
mod logger;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use config::load_config;
use db::create_pg_pool;
use errors::AppError;
use handlers::init_routes;
use logger::Logger;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let allowed_origins = vec![
        "https://frontend1.com",
        "https://frontend2.com",
        "https://api.microservice1.com",
        "https://api.microservice2.com",
    ];
    Logger::init();
    Logger::info("Starting Auth Service...");

    let config = load_config()?;
    let pg_pool = create_pg_pool(&config.database)?;
    let bind_addr = format!("{}:{}", config.service.host, config.service.port);

    Logger::info(&format!("Starting HTTP server at http://{}", bind_addr));

    HttpServer::new(move || {
        let allowed_origins = allowed_origins.clone(); // clone per new app instance

        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600)
            .allowed_origin_fn(move |origin, _req_head| {
                if let Ok(origin_str) = origin.to_str() {
                    allowed_origins.iter().any(|allowed| *allowed == origin_str)
                } else {
                    false
                }
            });

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pg_pool.clone()))
            .configure(init_routes)
    })
    .bind(bind_addr)
    .map_err(|e| AppError::Other(format!("Failed to bind server: {}", e)))?
    .workers(2)
    .run()
    .await
    .map_err(|e| AppError::Other(format!("Server run error: {}", e)))
}
