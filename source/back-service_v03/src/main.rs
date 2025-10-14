mod core;
mod services;

use crate::core::db::Db;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::config::{AppConfig, read};
use crate::services::scheduler;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logger
    Logger::init();
    Logger::info("Starting application");

    let config_file = "../../../config.yaml";
    let config: AppConfig = read(config_file)?;

    // Create PostgreSQL pool
    let pool = Db::create_pool();
    scheduler::run(pool, config).await?;

    Logger::info("Application finished");
    Ok(())
}
