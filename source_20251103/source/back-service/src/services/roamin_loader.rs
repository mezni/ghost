use crate::core::config::Source;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_manager::BatchManager;
use crate::services::file_manager;
use crate::services::lookup::PrefixLookup;
use chrono::{Local, NaiveDate};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, QueryBuilder};
use std::fs;
use std::path::PathBuf;

const FILE_TO_PROCESS: usize = 5;
const BATCH_NAME: &str = "LOADER";
const FILE_TYPE: &str = "IN";
const BATCH_INSERT_SIZE: usize = 500;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileProcessed {
    pub file_path: PathBuf,
    pub file_type: String,
    pub file_action: String,
    pub archive_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RoamInDataRecord {
    pub hlraddr: String,
    pub nsub: i32,
    pub nsuba: i32,
}

#[derive(Debug, Clone)]
pub struct RoamInBatch {
    pub batch_id: i32,
    pub batch_date: String,
    pub hlraddr: String,
    pub nsub: i32,
    pub nsuba: i32,
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

#[derive(Debug)]
struct Metadata {
    creation_date: String,
}

#[derive(Debug)]
struct SummaryRecord {
    totnsub: u64,
    totnsuba: u64,
    nsubpr: u64,
    nsubxp: u64,
    nsubpxou: u64,
    nsubsgs: u64,
    nsubgs: u64,
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

        Logger::debug(&format!("Processing file: {:?}", file_path));

        match process_file(&file, &file_processed, pool, batch_mgr, prefix_lookup).await {
            Ok(count) => Logger::info(&format!("Processed {} records from {:?}", count, file_path)),
            Err(e) => Logger::error(&format!("Error processing file {:?}: {}", file_path, e)),
        }

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

fn extract_creation_date(line: &str) -> Option<String> {
    line.split_whitespace().nth(4).and_then(|date_str| {
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

    for line in content.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if line.starts_with("ACT") && line.contains("TIME") {
            creation_date = extract_creation_date(line);
            continue;
        }

        if line.contains("MT MOBILE SUBSCRIBER SURVEY RESULT") {
            in_data_section = true;
            continue;
        }

        if in_data_section {
            for caps in re_row.captures_iter(line) {
                let nsub = caps[2].parse::<i32>().map_err(|e| {
                    AppError::new(format!("Failed to parse nsub '{}': {}", &caps[2], e))
                })?;
                let nsuba = caps[3].parse::<i32>().map_err(|e| {
                    AppError::new(format!("Failed to parse nsuba '{}': {}", &caps[3], e))
                })?;
                records.push(RoamInDataRecord {
                    hlraddr: caps[1].to_string(),
                    nsub,
                    nsuba,
                });
            }

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

    Logger::debug(&format!(
        "📊 Parsed {} records | TOTNSUB={}, TOTNSUBA={}",
        records.len(),
        summary.totnsub,
        summary.totnsuba
    ));

    Ok((metadata, records))
}

fn read_file(
    file_processed: &FileProcessed,
) -> Result<(Metadata, Vec<RoamInDataRecord>), AppError> {
    Logger::info(&format!("Reading file: {:?}", file_processed.file_path));
    let file_content = fs::read_to_string(&file_processed.file_path)?;
    parse_file_content(&file_content)
}

async fn process_file(
    file_name: &str,
    file_processed: &FileProcessed,
    pool: &Pool<Postgres>,
    batch_mgr: &BatchManager,
    prefix_lookup: &PrefixLookup,
) -> Result<usize, AppError> {
    let (metadata, records) = read_file(file_processed)?;

    if records.is_empty() {
        Logger::info(&format!("No records in file: {file_name}"));
        return Ok(0);
    }

    let batch_id = batch_mgr
        .batch_start(BATCH_NAME, FILE_TYPE, file_name)
        .await?;
    Logger::info(&format!("Batch started with ID: {}", batch_id));

    let batch_date = metadata.creation_date;

    let batches: Vec<RoamInBatch> = records
        .iter()
        .map(|r| {
            let hlraddr_number = r.hlraddr.split('-').nth(1).unwrap_or("").to_string();
            let p = prefix_lookup.lookup(hlraddr_number);
            RoamInBatch {
                batch_id,
                batch_date: batch_date.clone(),
                hlraddr: r.hlraddr.clone(),
                nsub: r.nsub,
                nsuba: r.nsuba,
                prefix: p.prefix,
                country_id: p.country_id,
                operator_id: p.operator_id,
            }
        })
        .collect();

    insert_batches(pool, &batches).await?;
    batch_mgr.batch_succeeded(batch_id).await?;
    Logger::info(&format!("✅ Batch {} succeeded", batch_id));

    Ok(records.len())
}

async fn insert_batches(pool: &Pool<Postgres>, batches: &[RoamInBatch]) -> Result<(), AppError> {
    for chunk in batches.chunks(BATCH_INSERT_SIZE) {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO stg_roam_in (batch_id, batch_date, hlraddr, nsub, nsuba, prefix, country_id, operator_id) ",
        );

        qb.push_values(chunk.iter(), |mut b, batch| {
            b.push_bind(batch.batch_id)
                .push_bind(&batch.batch_date)
                .push_bind(&batch.hlraddr)
                .push_bind(batch.nsub)
                .push_bind(batch.nsuba)
                .push_bind(&batch.prefix)
                .push_bind(batch.country_id)
                .push_bind(batch.operator_id);
        });

        qb.build().execute(pool).await?;
    }
    Ok(())
}
