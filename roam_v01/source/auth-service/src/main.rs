mod config;
mod db;
mod errors;
mod logger;

use actix_web::{App, HttpServer, http::header, web};
use config::load_config;
use db::create_pg_pool;
use errors::AppError;
use logger::Logger;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("Starting Auth Service...");

    let config = load_config()?;
    let pg_pool = create_pg_pool(&config.database)?;

    Ok(())
}
