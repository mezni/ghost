use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use actix_web_prom::PrometheusMetrics;
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry_jaeger::new_pipeline;

mod config;
mod db;
mod errors;
mod handlers;
mod logger;
mod metrics;
mod models;
mod repositories;
mod utils;

use crate::config::Config;
use crate::db::create_pg_pool;
use crate::errors::AppError;
use crate::logger::Logger;
use crate::metrics::init_metrics;

fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let tracer = new_pipeline()
        .with_service_name("auth_service")
        .with_auto_split_batch(true)
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry)
        .try_init()?;

    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    dotenv().ok();

    // Init tracing
    init_tracing().expect("Failed to initialize tracing");

    let config = Config::from_env()?;
    let db_config = config.database.clone();
    let db_pool = create_pg_pool(&db_config)?;
    let bind_addr = format!("{}:{}", config.server.host, config.server.port);

    Logger::info(&format!("Starting server at http://{}", bind_addr));

    // Init Prometheus
    let prometheus = init_metrics();

    // Start HTTP server
    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        for origin in &config.cors_allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default()) 
            .wrap(prometheus.clone())      
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config.jwt.clone()))
            .configure(handlers::init_routes)
    })
    .bind(&bind_addr)?
    .run()
    .await
    .map_err(AppError::from)?;

    // Ensure spans are flushed
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
