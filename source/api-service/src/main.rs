mod handlers;
mod repo;
mod service;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use core::config::read_srv_config;
use core::db::DBManager;
use core::errors::AppError;
use core::logger::Logger;
use std::process;
use std::sync::Arc;
pub const SERVICE_NAME: &str = "api-srv";

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info(&format!("{} : Starting up", SERVICE_NAME));

    let srv_config = match read_srv_config() {
        Ok(cfg) => {
            Logger::info(&format!("{} : Config Server - loaded", SERVICE_NAME));
            cfg
        }
        Err(e) => {
            Logger::error(&format!(
                "{} : Config Server - failed : {:?}",
                SERVICE_NAME, e
            ));
            Logger::info("Stopping.");
            process::exit(1);
        }
    };

    let db_manager = match DBManager::new(srv_config) {
        Ok(manager) => Arc::new(manager),
        Err(e) => {
            Logger::error(&format!("{} : DB Init - failed : {:?}", SERVICE_NAME, e));
            process::exit(1);
        }
    };

    let db_data = web::Data::new(db_manager);

    Logger::info(&format!(
        "{} : Server is running at http://127.0.0.1:3000",
        SERVICE_NAME
    ));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("http://frontend:8080")
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(db_data.clone())
            .configure(handlers::config)
    })
    .bind(("0.0.0.0", 3000))?
    //    .bind(("127.0.0.1", 3000))?
    .run()
    .await?;

    Ok(())
}
