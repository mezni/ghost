mod repo;
mod service;

use core::config::read_srv_config;
use core::errors::AppError;
use core::logger::Logger;
use service::{AnalyticsService, SERVICE_NAME};
use std::process;
use tokio::time::{Duration, sleep};
#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info(&format!("{} : Start", SERVICE_NAME));

    let interval_secs: u64 = 60;

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
            Logger::info("Stop.");
            process::exit(1);
        }
    };

    let service = match AnalyticsService::new(srv_config).await {
        Ok(s) => s,
        Err(e) => {
            Logger::error(&format!(
                "{} : Service initialization failed: {:?}",
                SERVICE_NAME, e
            ));
            process::exit(1);
        }
    };

    Logger::info(&format!("{} : Store - initialized", SERVICE_NAME));
    loop {
        if let Err(e) = service.execute().await {
            Logger::warn(&format!(
                "{} : Service execution failed: {:?}",
                SERVICE_NAME, e
            ));
        }

        sleep(Duration::from_secs(interval_secs)).await;
    }

    Logger::info(&format!("{} : Stop", SERVICE_NAME));

    Ok(())
}
