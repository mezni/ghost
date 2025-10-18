use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::{batch_mgr, config_mgr::Source, file_mgr, lookup};
use chrono::{Local, NaiveDate};
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

const FILE_TO_PROCESS: usize = 1;

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

/// Extract date from filename (YYYYMMDD). Defaults to current date if not found.
fn get_date_from_file_name(file_name: &str) -> String {
    file_name
        .as_bytes()
        .windows(8)
        .filter_map(|slice| std::str::from_utf8(slice).ok())
        .find_map(|s| NaiveDate::parse_from_str(s, "%Y%m%d").ok())
        .unwrap_or_else(|| Local::now().naive_local().date())
        .format("%Y-%m-%d")
        .to_string()
}

/// Validate a CSV line and return a record if valid.
fn parse_line(line: &str, line_number: usize) -> Option<RoamOutRecord> {
    let parts: Vec<&str> = line.split(',').map(str::trim).collect();
    if parts.len() != 3 {
        Logger::warn(&format!(
            "Skipping line {}: expected 3 columns, got {}",
            line_number,
            parts.len()
        ));
        return None;
    }

    let (imsi, msisdn, vlr_number) = (parts[0], parts[1], parts[2]);
    if imsi.is_empty() || msisdn.is_empty() || vlr_number.is_empty() {
        Logger::warn(&format!(
            "Skipping line {}: empty required field",
            line_number
        ));
        return None;
    }

    Some(RoamOutRecord {
        imsi: imsi.to_string(),
        msisdn: msisdn.to_string(),
        vlr_number: vlr_number.to_string(),
    })
}

/// Read and parse file into RoamOutRecords.
pub fn read_file(file_processed: &FileProcessed) -> Result<Vec<RoamOutRecord>, AppError> {
    Logger::info(&format!("Reading file: {:?}", file_processed.file_path));

    let file_content = fs::read_to_string(&file_processed.file_path)?;
    let records: Vec<RoamOutRecord> = file_content
        .lines()
        .enumerate()
        .skip(1) // skip header
        .filter_map(|(idx, line)| parse_line(line.trim(), idx + 1))
        .collect();

    Logger::info(&format!(
        "✅ Parsed {} records from {:?}",
        records.len(),
        file_processed.file_path
    ));
    Ok(records)
}

/// Insert roam out batches into staging table.
pub async fn insert_roam_out_batches(
    pool: &Pool,
    batches: &[RoamOutBatch],
) -> Result<u64, AppError> {
    if batches.is_empty() {
        Logger::info("No roam out batches to insert");
        return Ok(0);
    }

    let mut client: Client = pool.get().await?;
    let transaction = client.transaction().await?;

    let stmt = transaction
        .prepare(
            "INSERT INTO stg_roam_out (
                batch_id, batch_date, imsi, msisdn, vlr_number, 
                prefix, country_id, operator_id
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
        )
        .await?;

    let mut inserted = 0;
    for b in batches {
        if let Ok(rows) = transaction
            .execute(
                &stmt,
                &[
                    &b.batch_id,
                    &b.batch_date,
                    &b.imsi,
                    &b.msisdn,
                    &b.vlr_number,
                    &b.prefix,
                    &b.country_id,
                    &b.operator_id,
                ],
            )
            .await
        {
            inserted += rows;
        } else {
            Logger::error(&format!(
                "❌ Failed to insert record IMSI={}, MSISDN={}, VLR={}",
                b.imsi, b.msisdn, b.vlr_number
            ));
        }
    }

    transaction.commit().await?;
    Logger::info(&format!("✅ Inserted {} roam out batch records", inserted));
    Ok(inserted)
}

/// Load and process roam out files.
pub async fn load(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    source: &Source,
    prefix_lookup: &lookup::PrefixLookup,
) -> Result<(), AppError> {
    Logger::info("🚀 Starting ROAMOUT load");

    let files = file_mgr::get_files(
        &PathBuf::from(&source.source_directory),
        source.file_pattern.as_deref(),
        FILE_TO_PROCESS,
    )?;

    if files.is_empty() {
        Logger::debug("No files to process");
        return Ok(());
    }

    Logger::info(&format!("Found {} file(s) to process", files.len()));

    let mut stats = (0, 0, 0); // processed, failed, records

    for file in files {
        let file_path = PathBuf::from(&source.source_directory).join(&file);
        let archive_path = source.archive_directory.as_ref().map(PathBuf::from);
        let file_processed = FileProcessed {
            file_path: file_path.clone(),
            file_type: "OUT".into(),
            file_action: source.post_action.clone(),
            archive_path: archive_path.clone(),
        };

        match process_single_file(&file, &file_processed, pool, batch_mgr, prefix_lookup).await {
            Ok(count) => {
                stats.0 += 1;
                stats.2 += count;
                Logger::info(&format!("✅ Processed {file} ({count} records)"));
                let action = file_processed.file_action.as_deref().unwrap_or("delete");
                let _ = file_mgr::handle_file_action(&file_path, &archive_path, action).await;
            }
            Err(e) => {
                stats.1 += 1;
                Logger::error(&format!("❌ Failed processing file {file}: {e}"));
            }
        }
    }

    Logger::info(&format!(
        "🏁 ROAMOUT load completed: {} processed | {} failed | {} records",
        stats.0, stats.1, stats.2
    ));
    Ok(())
}

/// Process a single file.
async fn process_single_file(
    file_name: &str,
    file_processed: &FileProcessed,
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    prefix_lookup: &lookup::PrefixLookup,
) -> Result<usize, AppError> {
    let records = read_file(file_processed)?;
    if records.is_empty() {
        Logger::info(&format!("No records in file: {file_name}"));
        return Ok(0);
    }

    let batch_id = batch_mgr
        .insert_batch("LOADER", "OUT", file_name, "STARTED")
        .await?;
    let batch_date = get_date_from_file_name(file_name);

    let batches: Vec<RoamOutBatch> = records
        .into_iter()
        .map(|r| {
            let p = prefix_lookup.lookup(r.vlr_number.clone());
            RoamOutBatch {
                batch_id,
                batch_date: batch_date.clone(),
                imsi: r.imsi,
                msisdn: r.msisdn,
                vlr_number: r.vlr_number,
                prefix: p.prefix,
                country_id: p.country_id,
                operator_id: p.operator_id,
            }
        })
        .collect();

    insert_roam_out_batches(pool, &batches).await?;
    batch_mgr.update_status(batch_id, "COMPLETED").await?;
    Ok(batches.len())
}
