mod analytics;
mod core;
mod settings;

use crate::core::db::Db;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::core::middleware::{ErrorMiddleware, RequestLogger};
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    Logger::init();
    Logger::info("🔹 Starting API service");

    // Initialize PostgreSQL pool
    let pool = Db::pool().await?;
    Logger::info("✅ Database pool initialized");

    // Start HTTP server
    Logger::info("🚀 Starting server on http://0.0.0.0:3000");
    HttpServer::new(move || {
        // CORS configuration
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            // Add Db pool to app data
            .app_data(web::Data::new(pool.clone()))
            // Middlewares
            .wrap(RequestLogger)
            .wrap(ErrorMiddleware)
            .wrap(middleware::Logger::default()) // optional
            .wrap(cors)
            // Configure your routes
            .configure(core::health::config)
            .configure(settings::routes::config)
            .configure(analytics::routes::config)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(())
}
