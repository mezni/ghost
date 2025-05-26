mod errors;
mod traces;

use crate::{
    errors::AppError,
    traces::init_tracer,
};
use actix_web::{web, App, HttpServer, middleware::Logger};
use tracing_actix_web::TracingLogger;
use opentelemetry::global;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Initialize OpenTelemetry tracer
    let tracer = init_tracer()?;
    let _tracer_guard = global::shutdown_tracer_provider(); // Ensure cleanup on shutdown

    // Create HTTP server with tracing middleware
    HttpServer::new(move || {
        App::new()
            // Add OpenTelemetry tracing
            .wrap(TracingLogger::default())
            // Add regular Actix logger
            .wrap(Logger::default())
            // Register your tracer as application data
            .app_data(web::Data::new(tracer.clone()))
            // Add your routes here
            .service(
                web::resource("/")
                    .to(|| async { "Hello OpenTelemetry!" }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}