mod db;
mod errors;
mod handlers;
mod logger;
mod models;
mod repositories;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web};
use db::get_pool;
use errors::AppError;
use logger::Logger;

const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u16 = 3000;
const UPSTREAM_SERVER_HOST: &str = "http://localhost";
const UPSTREAM_SERVER_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    let pool = get_pool()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Logger::info("Database connection successful!");

    // Format the allowed origin as a String
    let allowed_origin = format!("{}:{}", UPSTREAM_SERVER_HOST, UPSTREAM_SERVER_PORT);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&allowed_origin)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::init_routes)
    })
    .bind((SERVER_HOST, SERVER_PORT))?
    .run()
    .await
    .map_err(|e| AppError::Other(e.to_string()))?;

    Ok(())
}
