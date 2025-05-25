use actix_web::{web, App, HttpServer};
use deadpool_postgres::{Manager, Pool};
use dotenvy::dotenv;
use tokio_postgres::NoTls;
use std::{env, sync::Arc};

mod config;
mod db;
mod errors;
mod handlers;
mod logger;
mod models;
mod repositories;
mod routes;
mod utils;

use crate::{
    config::{load_config, LoggerConfig},
    logger::init as init_logger,
};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Initialize configuration
    let config = load_config()?;
    let bind_addr = format!("{}:{}", config.service.host, config.service.port);

    // Initialize logger
    let logger_config = LoggerConfig {
        level: "info".to_string(),
        format: if config.service.environment == "production" {
            logger::LoggerFormat::Json
        } else {
            logger::LoggerFormat::Pretty
        },
        file_path: None,
        enable_otel: config.service.environment == "production",
        service_name: "auth-service".to_string(),
    };
    let _logger_guard = init_logger(logger_config)?;

    // Create database pool
    let pg_pool = create_pg_pool(&config.database).await?;
    let pg_pool = Arc::new(pg_pool);

    // Log startup info
    log::info!("Starting auth service on {}", bind_addr);
    log::info!(
        "Database connected to {}@{}:{}",
        config.database.user,
        config.database.host,
        config.database.port
    );

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .wrap(logger::RequestLogger::new())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().first() == Some(&b'*')
                            || config
                                .service
                                .allowed_origins
                                .split(',')
                                .any(|url| origin.to_str().unwrap_or("") == url.trim())
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .configure(routes::configure)
    })
    .bind(&bind_addr)?
    .workers(config.service.worker_count)
    .run()
    .await?;

    Ok(())
}

async fn create_pg_pool(db_config: &config::DatabaseConfig) -> Result<Pool, Box<dyn std::error::Error>> {
    let pg_config = tokio_postgres::Config::new()
        .host(&db_config.host)
        .port(db_config.port)
        .user(&db_config.user)
        .password(&db_config.password)
        .dbname(&db_config.name)
        .to_owned();

    let mgr_config = deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);

    Pool::builder(mgr)
        .max_size(db_config.max_connections.unwrap_or(20))
        .build()
        .map_err(Into::into)
}