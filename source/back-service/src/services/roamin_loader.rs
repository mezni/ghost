use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_mgr;
use crate::services::config_mgr::Source;
use crate::services::file_mgr;
use crate::services::lookup;
use chrono::{Local, NaiveDate};
use deadpool_postgres::{Client, Pool};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const FILE_TO_PROCESS: usize = 0;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileProcessed {
    pub file_path: PathBuf,
    pub file_type: String,
    pub file_action: Option<String>,
    pub archive_path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct RoamInDataRecord {
    pub hlraddr: String,
    pub nsub: i32,
    pub nsuba: i32,
}

#[derive(Debug, Serialize)]
struct SummaryRecord {
    totnsub: u64,
    totnsuba: u64,
    nsubpr: u64,
    nsubxp: u64,
    nsubpxou: u64,
    nsubsgs: u64,
    nsubgs: u64,
}

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub creation_date: String,
}

#[derive(Debug, Serialize)]
pub struct RoamInData {
    pub metadata: Metadata,
    pub records: Vec<RoamInDataRecord>,
}

#[derive(Debug)]
pub struct RoamInBatch {
    pub batch_id: i32,
    pub batch_date: String,
    pub hlraddr: String,
    pub nsub: String,  // Changed to String
    pub nsuba: String, // Changed to String
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

fn extract_creation_date(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts.get(4).and_then(|&date_str| {
        NaiveDate::parse_from_str(date_str, "%Y%m%d")
            .map(|date| date.format("%Y-%m-%d").to_string())
            .ok()
    })
}

fn update_summary(summary: &mut SummaryRecord, key: &str, value: u64) {
    match key {
        "TOTNSUB" => summary.totnsub = value,
        "TOTNSUBA" => summary.totnsuba = value,
        "NSUBPR" => summary.nsubpr = value,
        "NSUBXP" => summary.nsubxp = value,
        "NSUBPXOU" => summary.nsubpxou = value,
        "NSUBSGS" => summary.nsubsgs = value,
        "NSUBGS" => summary.nsubgs = value,
        _ => {}
    }
}

fn parse_file_content(content: &str) -> Result<(Metadata, Vec<RoamInDataRecord>), AppError> {
    let re_row = Regex::new(r"(4-\d+)\s+(\d+)\s+(\d+)")?;
    let re_summary = Regex::new(r"([A-Z]+)\s+(\d+)")?;

    let mut creation_date: Option<String> = None;
    let mut in_data_section = false;
    let mut records = Vec::new();
    let mut summary = SummaryRecord {
        totnsub: 0,
        totnsuba: 0,
        nsubpr: 0,
        nsubxp: 0,
        nsubpxou: 0,
        nsubsgs: 0,
        nsubgs: 0,
    };

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("ACT") && line.contains("TIME") {
            creation_date = extract_creation_date(line);
            continue;
        }

        if line.contains("MT MOBILE SUBSCRIBER SURVEY RESULT") {
            in_data_section = true;
            continue;
        }

        if in_data_section {
            // Parse data records
            for caps in re_row.captures_iter(line) {
                let record = RoamInDataRecord {
                    hlraddr: caps[1].to_string(),
                    nsub: caps[2].parse().map_err(|e| {
                        AppError::new(format!("Failed to parse nsub '{}': {}", &caps[2], e))
                    })?,
                    nsuba: caps[3].parse().map_err(|e| {
                        AppError::new(format!("Failed to parse nsuba '{}': {}", &caps[3], e))
                    })?,
                };
                records.push(record);
            }

            // Parse summary records
            if let Some(caps) = re_summary.captures(line) {
                if let (Some(key), Some(value_str)) = (caps.get(1), caps.get(2)) {
                    if let Ok(value) = value_str.as_str().parse::<u64>() {
                        update_summary(&mut summary, key.as_str(), value);
                    }
                }
            }
        }
    }

    let metadata = Metadata {
        creation_date: creation_date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string()),
    };

    // Log parsing summary
    Logger::debug(&format!(
        "Parsed {} records. Summary: TOTNSUB={}, TOTNSUBA={}",
        records.len(),
        summary.totnsub,
        summary.totnsuba
    ));

    Ok((metadata, records))
}

pub fn read_file(file_processed: FileProcessed) -> Result<RoamInData, AppError> {
    let file_content = fs::read_to_string(&file_processed.file_path)?;
    let (metadata, records) = parse_file_content(&file_content)?;
    Logger::info(&format!("Reading file: {:?}", &file_processed.file_path));

    Ok(RoamInData { metadata, records })
}

pub async fn insert_roam_in_batches(pool: &Pool, batches: &[RoamInBatch]) -> Result<u64, AppError> {
    if batches.is_empty() {
        Logger::info("No roam in batches to insert");
        return Ok(0);
    }

    let mut client: Client = pool.get().await?;
    let mut inserted_rows = 0;

    // Use a transaction for atomicity
    let transaction = client.transaction().await?;

    let stmt = transaction
        .prepare(
            "INSERT INTO stg_roam_in (
                batch_id, batch_date, hlraddr, nsub, nsuba,
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
                    &batch.hlraddr,
                    &batch.nsub,  // Now String
                    &batch.nsuba, // Now String
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
                    "Failed to insert batch record: HLR={}, NSUB={}, NSUBA={}, Error: {}",
                    batch.hlraddr, batch.nsub, batch.nsuba, e
                ));
                // Continue with other records even if one fails
            }
        }
    }

    // Commit the transaction
    transaction.commit().await?;

    Logger::info(&format!("Inserted {} roam in batch records", inserted_rows));
    Ok(inserted_rows)
}

pub async fn load(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    source: &Source,
    prefix_lookup: &lookup::PrefixLookup,
) -> Result<(), AppError> {
    Logger::info("Starting ROAMIN data load process");

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
            file_type: "IN".to_string(),
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
                } else {
                    file_mgr::handle_file_action(&file_path, &archive_path, "delete").await;
                }
            }
            Err(e) => {
                total_failed += 1;
                Logger::error(&format!("❌ Failed to process file {}: {}", file, e));
            }
        }
    }

    Logger::info(&format!(
        "Completed ROAMIN data load process. Files: {} processed, {} failed. Total records: {}",
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
    // Read file data
    let roam_data = read_file(file_processed)?;
    let records = roam_data.records;

    if records.is_empty() {
        Logger::info(&format!("No records to process in file: {}", file_name));
        return Ok(0);
    }

    // Start batch
    let batch_id = batch_mgr
        .insert_batch("LOADER", "IN", file_name, "STARTED")
        .await?;

    let batch_date = roam_data.metadata.creation_date;
    let mut batches = Vec::new();

    // Convert records to batches with prefix lookup
    for record in records {
        let hlraddr = record.hlraddr.clone();
        let parts: Vec<&str> = hlraddr.split('-').collect();
        let hlraddr_number = parts.get(1).unwrap_or(&"").to_string();

        let prefixes = prefix_lookup.lookup(hlraddr_number);
        batches.push(RoamInBatch {
            batch_id,
            batch_date: batch_date.clone(),
            hlraddr: record.hlraddr,
            nsub: record.nsub.to_string(),   // Convert i32 to String
            nsuba: record.nsuba.to_string(), // Convert i32 to String
            prefix: prefixes.prefix,
            country_id: prefixes.country_id,
            operator_id: prefixes.operator_id,
        });
    }

    // Insert batches into database
    let inserted_count = insert_roam_in_batches(pool, &batches).await?;

    // Update batch status
    batch_mgr.update_status(batch_id, "COMPLETED").await?;

    Ok(batches.len())
}
