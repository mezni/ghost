use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_mgr;
use crate::services::config_mgr::Source;
use crate::services::file_mgr;
use crate::services::lookup;
use chrono::Local;
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const FILE_TO_PROCESS: usize = 5;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileProcessed {
    pub file_path: PathBuf,
    pub file_type: String,
    pub file_action: Option<String>,
    pub archive_path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct RoamOutRecord {
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
}

#[derive(Debug)]
pub struct RoamOutBatch {
    pub batch_id: i32,
    pub batch_date: String,
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

fn get_date_from_file_name(file_name: &str) -> String {
    if file_name.len() >= 8 {
        for i in 0..file_name.len().saturating_sub(8) {
            let slice = &file_name[i..i + 8];
            if slice.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(slice, "%Y%m%d") {
                    return date.format("%Y-%m-%d").to_string();
                }
            }
        }
    }
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn read_file(file_processed: FileProcessed) -> Result<Vec<RoamOutRecord>, AppError> {
    let file_path = &file_processed.file_path;

    Logger::info(&format!("Reading file: {:?}", file_path));

    let file_content = fs::read_to_string(file_path)?;
    let mut records = Vec::new();
    let mut line_number = 0;

    for line in file_content.lines().skip(1) {
        // Skip header
        line_number += 1;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            Logger::warn(&format!(
                "Skipping line {}: invalid record length: expected 3 columns, got {}",
                line_number,
                parts.len()
            ));
            continue;
        }

        let imsi = parts[0].trim().to_string();
        let msisdn = parts[1].trim().to_string();
        let vlr_number = parts[2].trim().to_string();

        // Validate data before adding to records
        if imsi.is_empty() {
            Logger::warn(&format!("Skipping line {}: empty IMSI", line_number));
            continue;
        }
        if msisdn.is_empty() {
            Logger::warn(&format!("Skipping line {}: empty MSISDN", line_number));
            continue;
        }
        if vlr_number.is_empty() {
            Logger::warn(&format!("Skipping line {}: empty VLR number", line_number));
            continue;
        }

        records.push(RoamOutRecord {
            imsi,
            msisdn,
            vlr_number,
        });
    }

    Logger::info(&format!(
        "Successfully read {} records from file: {:?}",
        records.len(),
        file_path
    ));

    Ok(records)
}

pub async fn insert_roam_out_batches(
    pool: &Pool,
    batches: &[RoamOutBatch],
) -> Result<u64, AppError> {
    if batches.is_empty() {
        Logger::info("No roam out batches to insert");
        return Ok(0);
    }

    let mut client: Client = pool.get().await?;
    let mut inserted_rows = 0;

    // Use a transaction for atomicity
    let transaction = client.transaction().await?;

    let stmt = transaction
        .prepare(
            "INSERT INTO stg_roam_out (
                batch_id, batch_date, imsi, msisdn, vlr_number, 
                prefix, country_id, operator_id
             ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .await?;

    for batch in batches {
        match transaction
            .execute(
                &stmt,
                &[
                    &batch.batch_id,
                    &batch.batch_date,
                    &batch.imsi,
                    &batch.msisdn,
                    &batch.vlr_number,
                    &batch.prefix,
                    &batch.country_id,
                    &batch.operator_id,
                ],
            )
            .await
        {
            Ok(result) => {
                inserted_rows += result;
            }
            Err(e) => {
                Logger::error(&format!(
                    "Failed to insert batch record: IMSI={}, MSISDN={}, VLR={}, Error: {}",
                    batch.imsi, batch.msisdn, batch.vlr_number, e
                ));
                // Continue with other records even if one fails
            }
        }
    }

    // Commit the transaction
    transaction.commit().await?;

    Logger::info(&format!(
        "Inserted {} roam out batch records",
        inserted_rows
    ));
    Ok(inserted_rows)
}

pub async fn load(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    source: &Source,
    prefix_lookup: &lookup::PrefixLookup,
) -> Result<(), AppError> {
    Logger::info("Starting ROAMOUT data load process");

    let files = file_mgr::get_files(
        &PathBuf::from(&source.source_directory),
        source.file_pattern.as_deref(),
        FILE_TO_PROCESS,
    )?;

    if files.is_empty() {
        Logger::debug("No files found to process");
        return Ok(());
    }

    Logger::info(&format!("Found {} files to process", files.len()));

    let mut total_processed = 0;
    let mut total_failed = 0;
    let mut total_records = 0;

    for file in files {
        let file_path = PathBuf::from(&source.source_directory).join(&file);
        let archive_path = source.archive_directory.as_ref().map(PathBuf::from);

        let file_processed = FileProcessed {
            file_path: file_path.clone(),
            file_type: "OUT".to_string(),
            file_action: source.post_action.clone(),
            archive_path: archive_path.clone(),
        };

        match process_single_file(&file, file_processed, pool, batch_mgr, prefix_lookup).await {
            Ok(record_count) => {
                total_processed += 1;
                total_records += record_count;
                Logger::info(&format!(
                    "✅ Successfully processed file: {} ({} records)",
                    file, record_count
                ));

                // Handle file actions after successful processing
                if let Some(file_action) = &source.post_action {
                    file_mgr::handle_file_action(&file_path, &archive_path, file_action).await;
                }
            }
            Err(e) => {
                total_failed += 1;
                Logger::error(&format!("❌ Failed to process file {}: {}", file, e));
            }
        }
    }

    Logger::info(&format!(
        "Completed ROAMOUT data load process. Files: {} processed, {} failed. Total records: {}",
        total_processed, total_failed, total_records
    ));
    Ok(())
}

async fn process_single_file(
    file_name: &str,
    file_processed: FileProcessed,
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    prefix_lookup: &lookup::PrefixLookup,
) -> Result<usize, AppError> {
    // Read file records
    let records = read_file(file_processed)?;

    if records.is_empty() {
        Logger::info(&format!("No records to process in file: {}", file_name));
        return Ok(0);
    }

    // Start batch
    let batch_id = batch_mgr
        .insert_batch("LOADER", "OUT", file_name, "STARTED")
        .await?;

    let batch_date = get_date_from_file_name(file_name);
    let mut batches = Vec::new();

    // Convert records to batches with prefix lookup
    for record in records {
        let prefixes = prefix_lookup.lookup(record.vlr_number.clone());
        batches.push(RoamOutBatch {
            batch_id,
            batch_date: batch_date.clone(),
            imsi: record.imsi,
            msisdn: record.msisdn,
            vlr_number: record.vlr_number,
            prefix: prefixes.prefix,
            country_id: prefixes.country_id,
            operator_id: prefixes.operator_id,
        });
    }

    // Insert batches into database
    let inserted_count = insert_roam_out_batches(pool, &batches).await?;

    // Update batch status
    batch_mgr.update_status(batch_id, "COMPLETED").await?;

    Ok(batches.len())
}
