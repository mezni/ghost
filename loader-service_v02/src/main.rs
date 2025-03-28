mod config;
mod errors;
mod fs;
mod logger;
mod service;
mod store;

use config::read_config;
use errors::AppError;
use logger::Logger;
use service::LoadService;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize the logger
    Logger::init();
    Logger::info("Start Service");

    let config_file = "config.yaml";
    let config = read_config(config_file).unwrap();

    // Initialize the service
    let service = LoadService::new(config).await?;

    // Execute the service with sample data
    if let Err(e) = service.execute().await {
        Logger::error(&format!("Service execution failed: {}", e));
    }

    // Log the stop of the service
    Logger::info("Stop Service");

    Ok(())
}
