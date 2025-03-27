mod errors;
mod fs;
mod logger;
mod service;
mod store;

use errors::AppError;
use logger::Logger;
use service::LoadService;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize the logger
    Logger::init();
    Logger::info("Start Service");

    // Initialize the service
    let service = LoadService::new().await?;

    // Execute the service with sample data
    if let Err(e) = service.execute().await {
        Logger::error(&format!("Service execution failed: {}", e));
    }

    // Log the stop of the service
    Logger::info("Stop Service");

    Ok(())
}
