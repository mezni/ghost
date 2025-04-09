mod service;

use core::config::read_srv_config;
use core::errors::AppError;
use core::logger::Logger;
use service::AnalyticsService;
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

    let service = AnalyticsService::new(srv_config).await?;
    service.execute().await?;

    Logger::info("Stop.");
    Ok(())
}
