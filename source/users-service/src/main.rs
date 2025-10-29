mod core;
mod user_model;
mod auth_model;
mod user_service;
mod keycloak_service;
mod user_handler;
mod auth_handler;
mod routes;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::time::sleep;

use core::config::Config;
use user_service::UserService;
use keycloak_service::KeycloakService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().expect("Failed to load configuration");
    
    // Extract the values we need for binding
    let host = config.host.clone();
    let port = config.port;
    
    // Create database connection pool with retry logic
    let pool = create_db_pool_with_retry(&config.database_url).await;

    // Create services
    let user_service = UserService::new(pool.clone());
    let keycloak_service = KeycloakService::new(config.clone());

    println!("Server running on http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(keycloak_service.clone()))
            .configure(routes::configure_routes)
    })
    .bind((host, port))?
    .run()
    .await
}

async fn create_db_pool_with_retry(database_url: &str) -> sqlx::PgPool {
    let max_retries = 5;
    let mut retry_count = 0;
    
    loop {
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
        {
            Ok(pool) => {
                println!("✅ Successfully connected to database");
                return pool;
            }
            Err(e) => {
                retry_count += 1;
                if retry_count >= max_retries {
                    panic!("Failed to create pool after {} retries: {}", max_retries, e);
                }
                println!("⚠️  Failed to connect to database (attempt {}/{}): {}", retry_count, max_retries, e);
                println!("🕐 Retrying in 5 seconds...");
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}