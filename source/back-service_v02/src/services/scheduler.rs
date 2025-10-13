use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::config_reader::AppConfig;
use crate::services::loader::process;
use deadpool_postgres::Pool;
use std::path::Path;

pub async fn run(pool: Pool, config: AppConfig) -> Result<(), AppError> {
    Logger::info("📅 Scheduler started");

    for source in &config.sources {
        Logger::debug(&format!("Checking source type: {}", source.source_type));

        let source_type = source.source_type.as_str();
        let dir = &source.source_directory;

        if source_type != "ROAM_OUT" && source_type != "ROAM_IN" {
            Logger::warn(&format!("Skipping unsupported type: {}", source_type));
            continue;
        }

        if !Path::new(dir).exists() {
            Logger::warn(&format!("Directory not found: {}", dir));
            continue;
        }

        Logger::debug(&format!("📂 Directory exists: {}", dir));
        process(pool.clone(), source.clone()).await?;
    }

    Logger::info("✅ Scheduler finished");
    Ok(())
}
