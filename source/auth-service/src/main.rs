mod db;
mod errors;
mod handlers;
mod logger;
mod models;
mod repositories;

use actix_cors::Cors;
use actix_web::http::header; // <-- Add this
use actix_web::{App, HttpServer, web};
use db::get_pool;
use errors::AppError;
use handlers::{login, register};
use logger::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    Logger::init();
    Logger::info("Starting Auth Service...");

    let allowed_origin = std::env::var("CORS_ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    match get_pool().await {
        Ok(pool) => {
            Logger::info("Database connection successful!");

            HttpServer::new(move || {
                let cors = Cors::default()
                    .allowed_origin(&allowed_origin)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600);

                App::new()
                    .wrap(cors)
                    .app_data(web::Data::new(pool.clone()))
                    .configure(handlers::init_routes)
            })
            .bind(("0.0.0.0", 3000))?
            .run()
            .await
        }
        Err(e) => {
            eprintln!("Failed to initialize database pool: {}", e);
            std::process::exit(1);
        }
    }
}
