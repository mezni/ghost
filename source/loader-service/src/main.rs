mod service;

use core::config::{read_app_config, read_srv_config};
use core::errors::AppError;
use core::logger::Logger;
use service::LoadService;

use std::process;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info("Start.");

    let srv_config = match read_srv_config() {
        Ok(cfg) => {
            Logger::info("Config Server - Loaded");
            cfg
        }
        Err(e) => {
            Logger::error(&format!("Config Server - Failed : {:?}", e));
            Logger::info("Stop.");
            process::exit(1);
        }
    };
    let config_file = "config.yaml";
    let app_config = match read_app_config(config_file) {
        Ok(cfg) => {
            Logger::info("App Server - Loaded");
            cfg
        }
        Err(e) => {
            Logger::error(&format!("App Server - Failed : {:?}", e));
            Logger::info("Stop.");
            process::exit(1);
        }
    };

    let service = LoadService::new(srv_config, app_config).await?;
    service.execute().await?;
    Logger::info("Stop.");

    Ok(())
}
