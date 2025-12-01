mod repo;
mod service;

use core::errors::AppError;
use core::logger::Logger;
use service::AnalyticsService;

use std::process;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let interval_secs: u64 = 60;
    Logger::init();

    Logger::info("Starting process");

    let service = match AnalyticsService::new().await {
        Ok(srv) => {
            Logger::info("Config loaded");
            srv
        }
        Err(e) => {
            Logger::error(&format!("{}", e));
            Logger::info("Stopping process");
            process::exit(1);
        }
    };
    loop {
        if let Err(e) = service.execute().await {
            Logger::error(&format!("Execution failed: {}", e));
        }
        sleep(Duration::from_secs(interval_secs)).await;
    }
    Logger::info("Stopping process");

    Ok(())
}
