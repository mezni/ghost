use crate::core::{errors::AppError, logger::Logger};
use crate::services::batch_mgr;

use chrono::{Local, NaiveDate};
use deadpool_postgres::{Client, Pool};
use tokio_postgres::Statement;

const SOURCE_TYPE_IN: &str = "IN";
const SOURCE_TYPE_OUT: &str = "OUT";

async fn process_metrics() -> Result<(), AppError> {
    Ok(())
}
async fn process_perfs() -> Result<(), AppError> {
    Ok(())
}
async fn process_notifs() -> Result<(), AppError> {
    Ok(())
}

async fn process_batch(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    source_type: &str,
    corr_id: i32,
) -> Result<(), AppError> {
    let batch_id = batch_mgr
        .insert_batch("TRANSFORMER", source_type, "", "STARTED")
        .await?;

    Logger::info(&format!(
        "Processing batch {} for source type {}",
        batch_id, source_type
    ));

    if let Err(e) = process_metrics().await {
        Logger::error(&format!(
            "Error processing metrics for batch {}: {}",
            batch_id, e
        ));
        batch_mgr.update_status(batch_id, "FAILED").await?;
        return Err(e);
    }

    if let Err(e) = process_perfs().await {
        Logger::error(&format!(
            "Error processing perfs for batch {}: {}",
            batch_id, e
        ));
        batch_mgr.update_status(batch_id, "FAILED").await?;
        return Err(e);
    }

    if let Err(e) = process_notifs().await {
        Logger::error(&format!(
            "Error processing notifs for batch {}: {}",
            batch_id, e
        ));
        batch_mgr.update_status(batch_id, "FAILED").await?;
        return Err(e);
    }

    batch_mgr.update_status(batch_id, "COMPLETED").await?;
    Logger::info(&format!(
        "Batch {} for source type {} processed successfully",
        batch_id, source_type
    ));

    Ok(())
}

pub async fn run(pool: &Pool, batch_mgr: &batch_mgr::BatchManager) -> Result<(), AppError> {
    for source_type in [SOURCE_TYPE_IN, SOURCE_TYPE_OUT] {
        while let Some(corr_id) = batch_mgr.get_corr_id(source_type.to_string()).await? {
            process_batch(pool, batch_mgr, source_type, corr_id).await?;
        }
    }
    Ok(())
}
