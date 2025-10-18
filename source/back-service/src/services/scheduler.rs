use crate::core::config::AppConfig;
use crate::core::db::Db;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_manager::BatchManager;
use crate::services::file_manager;
use crate::services::lookup::PrefixLookup;
use crate::services::roamin_loader;
use crate::services::roamout_loader;
use sqlx::PgPool;
use std::path::PathBuf;
use tokio::time::{self, Duration};

const TIMEOUT_SECONDS: u64 = 5;
const ROAM_IN: &str = "ROAM_IN";
const ROAM_OUT: &str = "ROAM_OUT";

pub struct Scheduler;

impl Scheduler {
    pub fn new() -> Self {
        Scheduler
    }
}

async fn execute_once(
    pool: &PgPool,
    batch_manager: &BatchManager,
    config: &AppConfig,
) -> Result<(), AppError> {
    Logger::info("⚙️ Executing scheduled task...");

    // Initialize PrefixLookup once
    let prefix_lookup = PrefixLookup::new(&pool).await?;

    for source in &config.sources {
        Logger::debug(&format!("📂 Processing source: {:?}", source));

        let directory = PathBuf::from(&source.source_directory);

        if !file_manager::check_directory(&directory) {
            Logger::warn(&format!("⚠️ Directory not found: {:?}", directory));
            continue;
        }

        match source.source_type.as_str() {
            ROAM_IN => {
                Logger::info("📥 Detected ROAM_IN source — starting loader...");
                roamin_loader::load(pool, batch_manager, source, &prefix_lookup).await?;
            }
            ROAM_OUT => {
                Logger::info("📤 Detected ROAM_OUT source — starting loader...");
                roamout_loader::load(pool, batch_manager, source, &prefix_lookup).await?;
            }
            unknown => {
                Logger::warn(&format!("❓ Unknown source type: {}", unknown));
            }
        }
    }

    Ok(())
}

/// Run the scheduler in a loop
pub async fn run(config: AppConfig) -> Result<(), AppError> {
    Logger::info("Scheduler started");

    // Initialize DB pool once
    let pool = Db::pool().await?;

    // Initialize BatchManager once
    let manager = BatchManager::from_global().await?;

    let mut ticker = time::interval(Duration::from_secs(TIMEOUT_SECONDS));

    loop {
        if let Err(err) = execute_once(&pool, &manager, &config).await {
            Logger::error(&format!("Error executing task: {}", err));
        }

        ticker.tick().await;
    }
}
