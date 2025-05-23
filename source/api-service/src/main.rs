mod logger;
mod errors;
mod db;
mod models;
mod repositories;
mod handlers;

use actix_web::{web, App, HttpServer};
use logger::Logger;
use errors::AppError;
use db::get_pool;

const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u16 = 3000;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    let pool = get_pool()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Logger::info("Database connection successful!");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1")
                    .configure(handlers::init_config)
            )
    })
    .bind((SERVER_HOST, SERVER_PORT))?
    .run()
    .await
    .map_err(|e| AppError::Other(e.to_string()))?;

    Ok(())
}
