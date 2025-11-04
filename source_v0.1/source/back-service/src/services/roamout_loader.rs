use crate::core::config::Source;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_manager::BatchManager;
use crate::services::file_manager;
use crate::services::lookup::PrefixLookup;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, QueryBuilder};
use std::fs;
use std::path::PathBuf;

const FILE_TO_PROCESS: usize = 5;
const BATCH_NAME: &str = "LOADER";
const FILE_TYPE: &str = "OUT";
const BATCH_INSERT_SIZE: usize = 500;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileProcessed {
    pub file_path: PathBuf,
    pub file_type: String,
    pub file_action: String,
    pub archive_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RoamOutRecord {
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
}

#[derive(Debug, Clone)]
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

pub async fn load(
    pool: &Pool<Postgres>,
    batch_mgr: &BatchManager,
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

    for file in files {
        Logger::debug(&format!("Processing file: {:?}", file));

        let file_path = PathBuf::from(&source.source_directory).join(&file);
        let archive_path = source.archive_directory.as_ref().map(PathBuf::from);
        let action = source
            .post_action
            .clone()
            .unwrap_or_else(|| "delete".to_string());

        let file_processed = FileProcessed {
            file_path: file_path.clone(),
            file_type: FILE_TYPE.to_string(),
            file_action: action.clone(),
            archive_path: archive_path.clone(),
        };

        // Process the file
        match process_file(&file, &file_processed, pool, batch_mgr, prefix_lookup).await {
            Ok(count) => {
                Logger::info(&format!("Processed {} records from {:?}", count, file_path));
            }
            Err(e) => {
                Logger::error(&format!("Error processing file {:?}: {}", file_path, e));
            }
        }

        // Handle post-processing of the file: archive or delete
        match file_processed.file_action.to_lowercase().as_str() {
            "delete" => {
                Logger::info(&format!("Deleting file: {:?}", file_processed.file_path));
                file_manager::delete_file(&file_processed.file_path)?;
            }
            "archive" => {
                if let Some(archive_dir) = &source.archive_directory {
                    let archive_path = PathBuf::from(archive_dir);
                    Logger::info(&format!(
                        "Archiving file {:?} to {:?}",
                        file_processed.file_path, archive_path
                    ));
                    file_manager::archive_file(&file_processed.file_path, &archive_path, "archive")
                        .await?;
                } else {
                    Logger::warn(&format!(
                        "Archive directory not set, cannot archive file {:?}, skipping",
                        file_processed.file_path
                    ));
                }
            }
            other => {
                Logger::warn(&format!(
                    "Unknown post_action '{}', skipping file {:?}",
                    other, file_processed.file_path
                ));
            }
        }
    }

    Ok(())
}

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

fn read_file(file_processed: &FileProcessed) -> Result<Vec<RoamOutRecord>, AppError> {
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

async fn process_file(
    file_name: &str,
    file_processed: &FileProcessed,
    pool: &Pool<Postgres>,
    batch_mgr: &BatchManager,
    prefix_lookup: &PrefixLookup,
) -> Result<usize, AppError> {
    let records = read_file(file_processed)?;
    if records.is_empty() {
        Logger::info(&format!("No records in file: {file_name}"));
        return Ok(0);
    }

    let batch_id = batch_mgr
        .batch_start(BATCH_NAME, FILE_TYPE, file_name)
        .await?;
    Logger::info(&format!("Batch started with ID: {}", batch_id));

    let batch_date = get_date_from_file_name(file_name);

    let batches: Vec<RoamOutBatch> = records
        .iter()
        .map(|r| {
            let p = prefix_lookup.lookup(r.vlr_number.clone());
            RoamOutBatch {
                batch_id,
                batch_date: batch_date.clone(),
                imsi: r.imsi.clone(),
                msisdn: r.msisdn.clone(),
                vlr_number: r.vlr_number.clone(),
                prefix: p.prefix,
                country_id: p.country_id,
                operator_id: p.operator_id,
            }
        })
        .collect();

    // Insert batches into DB in chunks
    insert_batches(pool, &batches).await?;

    batch_mgr.batch_succeeded(batch_id).await?;
    Logger::info(&format!("✅ Batch {} succeeded", batch_id));

    Ok(records.len())
}

async fn insert_batches(pool: &Pool<Postgres>, batches: &[RoamOutBatch]) -> Result<(), AppError> {
    for chunk in batches.chunks(BATCH_INSERT_SIZE) {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO stg_roam_out \
            (batch_id, batch_date, imsi, msisdn, vlr_number, prefix, country_id, operator_id) ",
        );

        qb.push_values(chunk.iter(), |mut b, batch| {
            b.push_bind(batch.batch_id)
                .push_bind(&batch.batch_date)
                .push_bind(&batch.imsi)
                .push_bind(&batch.msisdn)
                .push_bind(&batch.vlr_number)
                .push_bind(&batch.prefix)
                .push_bind(batch.country_id)
                .push_bind(batch.operator_id);
        });

        qb.build().execute(pool).await?;
    }
    Ok(())
}
