// src/main.rs

mod app;
mod domain; // Contains Country, CountryRepository, CountryService
mod errors;
mod infra; // Now contains all app-specific modules, including countries

use errors::AppError;
use infra::logger::Logger;
use sqlx::PgPool;

// Actix Web imports
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};

// Use types from your domain module
use crate::domain::countries::{CountryRepository, CountryService};
// Import the consolidated countries module
use crate::app::countries::configure_routes; // Import the new countries module

use crate::infra::postgres::countries::PostgresCountryRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("Application starting up...");

    let server_config = infra::config::load_config()?;
    let db_pool: PgPool = infra::postgres::db::init_db_pool(&server_config.database).await?;
    Logger::info("Application configuration and database pool initialized.");

    // Create the concrete repository instance
    let country_repo = Arc::new(PostgresCountryRepository::new(Arc::new(db_pool.clone())));

    // Create the service instance, injecting the repository
    let country_service = web::Data::new(CountryService::new(country_repo));

    let app_data_db_pool = web::Data::new(db_pool);

    let server_address = format!(
        "{}:{}",
        server_config.service.host, server_config.service.port
    );
    Logger::info(&format!("Starting server at {}", server_address));

    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(app_data_db_pool.clone())
            .app_data(country_service.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            // Use the configure_routes function from the new countries module
            .configure(configure_routes)
    })
    .bind(&server_address)
    .map_err(|e| {
        AppError::ServiceError(format!(
            "Failed to bind server to {}: {}",
            server_address, e
        ))
    })?
    .run();

    server
        .await
        .map_err(|e| AppError::ServiceError(format!("Server failed to run: {}", e)))?;

    Logger::info("Application shut down.");
    Ok(())
}
