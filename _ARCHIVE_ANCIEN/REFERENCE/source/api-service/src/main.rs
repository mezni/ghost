mod handlers;
mod repo;
mod service;

use core::config;
use core::db::DBManager;
use core::errors::AppError;
use core::logger::Logger;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use std::process;
use std::sync::Arc;

const FRONTEND_URL_LOCAL: &str = "http://localhost:8080";
const FRONTEND_URL_DOCKER: &str = "http://frontend:8080";
const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info("Starting process");

    let srv_config = match config::read_srv_config() {
        Ok(cfg) => {
            Logger::info(&format!("Config Server - loaded"));
            cfg.validate()?;
            cfg
        }
        Err(e) => {
            Logger::error(&format!("Config Server - failed : {:?}", e));
            Logger::info("Stopping.");
            process::exit(1);
        }
    };

    let db_manager = match DBManager::new(srv_config) {
        Ok(manager) => Arc::new(manager),
        Err(e) => {
            Logger::error(&format!("DB Init - failed : {:?}", e));
            process::exit(1);
        }
    };

    let db_data = web::Data::new(db_manager);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(FRONTEND_URL_LOCAL)
                    .allowed_origin(FRONTEND_URL_DOCKER)
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(db_data.clone())
            .configure(handlers::config)
    })
    .bind((SERVER_ADDR, SERVER_PORT))?
    .run()
    .await?;

    Logger::info("Stopping process");

    Ok(())
}
