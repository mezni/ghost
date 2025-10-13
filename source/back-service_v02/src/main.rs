mod core;
mod services;

use crate::core::db::Db;
use crate::core::logger::Logger;
use crate::core::errors::AppError;
use crate::services::scheduler;
use crate::services::config_reader::AppConfig;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logger
    Logger::init();
    Logger::info("Starting application");

    // Create PostgreSQL pool
    let pool = Db::create_pool();

    // Load configuration
    let config_file = "config.yaml";
    let config: AppConfig = AppConfig::from_file(config_file)?;

    // Run scheduler
    scheduler::run(pool, config).await?;

    Logger::info("Application finished");
    Ok(())
}
