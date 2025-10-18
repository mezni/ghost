use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::{
    batch_mgr, config_mgr::AppConfig, file_mgr, lookup, roamin_loader, roamout_loader, transformer,
};
use deadpool_postgres::Pool;
use std::path::PathBuf;
use tokio::time::{Duration, interval};

const TIME_OUT_SECONDS: u64 = 5;
const ROAM_IN: &str = "ROAM_IN";
const ROAM_OUT: &str = "ROAM_OUT";

pub async fn execute(pool: &Pool, config: &AppConfig) -> Result<(), AppError> {
    let batch_mgr = batch_mgr::BatchManager::new(pool.clone());

    // Process all sources
    process_sources(pool, &batch_mgr, config).await?;

    // Transform data
    transformer::transform(pool, &batch_mgr).await?;

    Ok(())
}

async fn process_sources(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    config: &AppConfig,
) -> Result<(), AppError> {
    let prefix_lookup = lookup::PrefixLookup::new(pool)
        .await
        .map_err(AppError::from)?;

    for source in &config.sources {
        Logger::debug(&format!("Processing source: {:?}", source));

        if let Err(e) = process_single_source(pool, batch_mgr, source, &prefix_lookup).await {
            Logger::error(&format!("Failed to process source {:?}: {}", source, e));
            // Continue with other sources even if one fails
        }
    }

    Ok(())
}

async fn process_single_source(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    source: &crate::services::config_mgr::Source,
    prefix_lookup: &lookup::PrefixLookup,
) -> Result<(), AppError> {
    let directory = PathBuf::from(&source.source_directory);

    if !file_mgr::check_directory(&directory) {
        Logger::warn(&format!("Directory not found: {:?}", directory));
        return Ok(());
    }

    match source.source_type.as_str() {
        ROAM_IN => {
            roamin_loader::load(pool, batch_mgr, source, prefix_lookup).await?;
        }
        ROAM_OUT => {
            roamout_loader::load(pool, batch_mgr, source, prefix_lookup).await?;
        }
        unknown_type => {
            Logger::warn(&format!("Unknown source type: {}", unknown_type));
        }
    }

    Ok(())
}

pub async fn run(pool: Pool, config: AppConfig) -> Result<(), AppError> {
    let mut interval = interval(Duration::from_secs(TIME_OUT_SECONDS));

    loop {
        if let Err(e) = execute(&pool, &config).await {
            Logger::error(&format!("Error executing task: {}", e));
        }

        interval.tick().await;
    }
}
