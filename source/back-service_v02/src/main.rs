mod core;
mod services;

use crate::core::{db::Db, errors::AppError, logger::Logger};
use crate::services::{config_mgr, scheduler};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    Logger::init();
    Logger::info("🚀 Starting application...");

    // Load configuration
    let config_path = "../../config.yaml";
    let config = config_mgr::read(config_path)?;
    Logger::info(&format!("✅ Loaded configuration from {}", config_path));

    // Create database pool
    let pool = Db::create_pool();

    // Start scheduler
    if let Err(err) = scheduler::run(pool, config).await {
        Logger::error(&format!("❌ Scheduler error: {}", err));
        return Err(err);
    }

    Logger::info("🟢 Application stopped gracefully.");
    Ok(())
}
