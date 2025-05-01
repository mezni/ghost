mod config;
mod routes;
mod handlers;
mod models;
mod db;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use config::get_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger (optional, useful for debugging)
    env_logger::init();

    // Create database pool
    let pool = get_db_pool().expect("Failed to create PostgreSQL connection pool");

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone())) // Share DB pool across handlers
            .configure(routes::init)           // Register all routes
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
