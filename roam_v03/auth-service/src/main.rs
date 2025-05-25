// src/main.rs
use actix_web::{App, HttpResponse, HttpServer, web};
use dotenvy::dotenv;

mod config;
mod db;
mod errors;
mod logger; // <- import the db module

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
        App::new()
            .app_data(web::Data::new(db_pool.clone())) 
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Auth server up") }),
            )
    })
    .bind(&bind_addr)?
    .run()
    .await
    .map_err(AppError::from)
}
