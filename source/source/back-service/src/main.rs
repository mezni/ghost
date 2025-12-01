mod core;
mod services;

use crate::core::{config::AppConfig, errors::AppError, logger::Logger};
use crate::services::scheduler;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("App started");
    //    let config = AppConfig::read("../../config.yaml")?;
    let config = AppConfig::read("./config.yaml")?;

    if let Err(err) = scheduler::run(config).await {
        Logger::error(&format!("Scheduler error: {}", err));
        return Err(err);
    }
    Logger::info("App stopped");
    Ok(())
}
