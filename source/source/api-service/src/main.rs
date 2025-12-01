mod analytics;
mod auth;
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
    Logger::init();
    Logger::info("🔹 Starting API service");

    let pool = Db::pool().await?;
    Logger::info("✅ Database pool initialized");

    let auth_service = auth::service::AuthService::new(pool.clone());

    let bind_address = ("0.0.0.0", 3000);
    Logger::info(&format!(
        "🚀 Starting server on http://{}:{}",
        bind_address.0, bind_address.1
    ));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone())) // This works now because we added Clone to AuthService
            .wrap(RequestLogger)
            .wrap(ErrorMiddleware)
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api/v1")
                    .configure(core::health::config)
                    .configure(settings::handlers::config)
                    .configure(analytics::handlers::config)
                    .configure(auth::handlers::config),
            )
    })
    .bind(bind_address)
    .map_err(|e| AppError::Internal(format!("Failed to bind server: {}", e)))?
    .run()
    .await
    .map_err(|e| AppError::Internal(format!("Server error: {}", e)))?;

    Ok(())
}
