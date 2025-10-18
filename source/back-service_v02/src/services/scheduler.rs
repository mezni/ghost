use crate::core::{errors::AppError, logger::Logger};
use crate::services::{
    batch_mgr, config_mgr::AppConfig, file_mgr, lookup, roamin_loader, roamout_loader, transformer,
};
use deadpool_postgres::Pool;
use std::path::PathBuf;
use tokio::time::{self, Duration};

const TIMEOUT_SECONDS: u64 = 5;
const ROAM_IN: &str = "ROAM_IN";
const ROAM_OUT: &str = "ROAM_OUT";

/// Executes one scheduler cycle
async fn execute_once(pool: &Pool, config: &AppConfig) -> Result<(), AppError> {
    Logger::info("⚙️  Executing scheduled task...");

    // Prepare dependencies
    let batch_mgr = batch_mgr::BatchManager::new(pool.clone());
    let prefix_lookup = lookup::PrefixLookup::new(pool)
        .await
        .map_err(AppError::from)?;

    for source in &config.sources {
        Logger::debug(&format!("📂 Processing source: {:?}", source));

        let directory = PathBuf::from(&source.source_directory);

        if !file_mgr::check_directory(&directory) {
            Logger::warn(&format!("⚠️  Directory not found: {:?}", directory));
            continue;
        }

        match source.source_type.as_str() {
            ROAM_IN => {
                Logger::info("📥 Detected ROAM_IN source — starting loader...");
                roamin_loader::load(pool, &batch_mgr, source, &prefix_lookup).await?;
            }
            ROAM_OUT => {
                Logger::info("📤 Detected ROAM_OUT source — starting loader...");
                roamout_loader::load(pool, &batch_mgr, source, &prefix_lookup).await?;
            }
            unknown => {
                Logger::warn(&format!("❓ Unknown source type: {}", unknown));
            }
        }
    }

    transformer::run(pool, &batch_mgr).await?;
    Logger::info("✅ Task completed successfully.");
    Ok(())
}

/// Starts the scheduler loop
pub async fn run(pool: Pool, config: AppConfig) -> Result<(), AppError> {
    let mut ticker = time::interval(Duration::from_secs(TIMEOUT_SECONDS));

    Logger::info(&format!(
        "⏳ Scheduler started with interval: {}s",
        TIMEOUT_SECONDS
    ));

    loop {
        if let Err(err) = execute_once(&pool, &config).await {
            Logger::error(&format!("❌ Error executing task: {}", err));
        }

        ticker.tick().await;
    }
}
