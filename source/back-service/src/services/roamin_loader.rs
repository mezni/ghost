use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_mgr;
use crate::services::config_mgr::Source;
use crate::services::file_mgr;
use chrono::{Local, NaiveDate};
use deadpool_postgres::Pool;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const FILE_TO_PROCESS: usize = 5;

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

pub async fn parse_file(file: FileProcessed) -> Result<RoamInData, AppError> {
    let file_content = fs::read_to_string(&file.file_path)?;

    let (metadata, records) = parse_file_content(&file_content)?;

    // Archive the file if archive path is specified
    if let (Some(archive_path), Some(file_action)) = (&file.archive_path, &file.file_action) {
        if let Err(e) = archive_file(&file.file_path, archive_path, file_action).await {
            Logger::error(&format!(
                "Failed to archive file {:?}: {}",
                file.file_path, e
            ));
        }
    }

    Ok(RoamInData { metadata, records })
}

async fn archive_file(
    source_path: &Path,
    archive_path: &Path,
    action: &str,
) -> Result<(), AppError> {
    match action.to_lowercase().as_str() {
        "move" => {
            if !archive_path.exists() {
                fs::create_dir_all(archive_path)?;
            }
            let destination = archive_path.join(
                source_path
                    .file_name()
                    .ok_or_else(|| AppError::invalid_file_name(source_path.to_string_lossy()))?,
            );
            fs::rename(source_path, destination)?;
        }
        "copy" => {
            if !archive_path.exists() {
                fs::create_dir_all(archive_path)?;
            }
            let destination = archive_path.join(
                source_path
                    .file_name()
                    .ok_or_else(|| AppError::invalid_file_name(source_path.to_string_lossy()))?,
            );
            fs::copy(source_path, destination)?;
        }
        "delete" => {
            fs::remove_file(source_path)?;
        }
        _ => {
            // Unknown action, just log and continue
            Logger::warn(&format!("Unknown file action: {}", action));
        }
    }
    Ok(())
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

pub async fn load(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    source: &Source,
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

    for file in files {
        let file_path = PathBuf::from(&source.source_directory).join(&file);
        let archive_path = source.archive_directory.as_ref().map(PathBuf::from);

        let file_processed = FileProcessed {
            file_path,
            file_type: "IN".to_string(),
            file_action: source.post_action.clone(),
            archive_path,
        };

        Logger::debug(&format!("Processing file: {}", file));
        let file_processed_clone = file_processed.clone();
        match parse_file(file_processed).await {
            Ok(roam_data) => {
                Logger::debug(&format!(
                    "Successfully parsed file: {} ({} records)",
                    file,
                    roam_data.records.len()
                ));

                // TODO: Add database insertion logic here using pool and batch_mgr
                // Example:
                // batch_mgr.insert_roam_data(pool, &roam_data).await?;

                if let Some(action) = &file_processed_clone.file_action {
                    if action.to_lowercase() == "delete" {
                        let _ = file_mgr::delete_file(&file_processed_clone.file_path);
                    }
                } else {
                    let _ = file_mgr::delete_file(&file_processed_clone.file_path);
                }
            }
            Err(e) => {
                Logger::error(&format!("Failed to parse file {}: {}", file, e));
            }
        }
    }

    Ok(())
}
