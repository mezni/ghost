// src/main.rs
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, http, web};
use dotenvy::dotenv;

mod config;
mod db;
mod errors;
mod handlers;
mod logger;

use crate::config::Config;
use crate::db::create_pg_pool;
use crate::errors::AppError;
use crate::logger::Logger;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    dotenv().ok();
    let config = Config::from_env()?;

    let db_config = config.database.clone();
    let db_pool = create_pg_pool(&db_config)?;

    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    Logger::info(&format!("Starting server at http://{}", bind_addr));

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
            .app_data(web::Data::new(db_pool.clone()))
            .configure(handlers::init_routes)
    })
    .bind(&bind_addr)?
    .run()
    .await
    .map_err(AppError::from)
}
