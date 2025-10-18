use crate::core::config::Source;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_manager;
use crate::services::file_manager;
use crate::services::lookup::PrefixLookup;
use sqlx::Pool;
use std::path::PathBuf;

const FILE_TO_PROCESS: usize = 10;
const BATCH_NAME: &str = "LOADER";
const DIRECTION: &str = "OUT";

pub async fn load(
    pool: &Pool<sqlx::Postgres>,
    batch_mgr: &batch_manager::BatchManager,
    source: &Source,
    prefix_lookup: &PrefixLookup,
) -> Result<(), AppError> {
    let files = file_manager::get_files(
        &PathBuf::from(&source.source_directory),
        source.file_pattern.as_deref(),
        FILE_TO_PROCESS,
    )?;

    if files.is_empty() {
        Logger::debug("No files to process");
        return Ok(());
    }

    Logger::info(&format!("Found {} file(s) to process", files.len()));

    // TODO: Process files here
    for file in files {
        Logger::debug(&format!("Processing file: {:?}", file));
        // You can read the file and use prefix_lookup here
        // Start a batch
        let batch_id = batch_mgr.batch_start(BATCH_NAME, DIRECTION, &file).await?;
        Logger::info(&format!("Batch started with ID: {}", batch_id));
        // Mark batch as succeeded
        batch_mgr.batch_succeeded(batch_id).await?;
        Logger::info(&format!("✅ Batch {} succeeded", batch_id));
    }

    Ok(())
}
