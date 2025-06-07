// src/main.rs

mod domain;
mod errors;
mod infra;
use errors::AppError; // Ensure AppError is imported
use infra::logger::Logger;
use sqlx::PgPool;

// Actix Web imports
use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

// Handler for a simple "hello world" route
async fn hello_world_handler() -> impl Responder {
    Logger::info("Received request for /hello");
    HttpResponse::Ok().body("Hello, World!")
}

// Handler that demonstrates access to the database pool
async fn db_test_handler(db_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    Logger::info("Received request for /db_test");

    // Ping the database. If it fails, sqlx::Error will be converted to AppError::DatabaseError
    // No need for match statement or format! - the `?` operator handles the conversion now.
    sqlx::query("SELECT 1").execute(db_pool.get_ref()).await?; // This will convert sqlx::Error to AppError::DatabaseError automatically

    Logger::info("Database ping successful from handler.");
    Ok(HttpResponse::Ok().body("Database connection test successful!"))
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info("Application starting up...");

    let server_config = infra::config::load_config()?;

    let db_pool: PgPool = infra::postgres::db::init_db_pool(&server_config.database).await?;

    Logger::info("Application configuration and database pool initialized.");

    let app_data_db_pool = web::Data::new(db_pool);

    let server_address = format!(
        "{}:{}",
        server_config.service.host, server_config.service.port
    );
    Logger::info(&format!("Starting server at {}", server_address));

    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(app_data_db_pool.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(web::resource("/hello").to(hello_world_handler))
            .service(web::resource("/db_test").to(db_test_handler))
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
