use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch;
use crate::services::config::{AppConfig, Source};
use chrono::Utc;
use deadpool_postgres::Pool;
use regex::Regex;
use std::path::PathBuf;

use std::fs;

#[derive(Debug)]
struct RoamOutBatch {
    batch_id: i32,
    batch_date: String,
    imsi: String,
    msisdn: String,
    vlr_number: String,
    prefix: String,
    country_id: Option<i32>,
    operator_id: Option<i32>,
}

fn get_first_file(path: &str) -> Option<String> {
    let path = std::path::Path::new(path);
    if path.is_dir() {
        let files = fs::read_dir(path).ok()?;
        for file in files {
            let file = file.ok()?;
            let file_path = file.path();
            if file_path.is_file() {
                return file_path
                    .file_name()
                    .map(|f| f.to_string_lossy().into_owned());
            }
        }
    }
    None
}

pub async fn load_roamin(
    batch_mgr: &batch::BatchManager,
    source_directory: &str,
    file_name: String,
) -> Result<(), AppError> {
    Logger::info("CALL ROAMIN");
    let batch_id = batch_mgr
        .insert_batch("LOADER", "IN", &file_name, "STARTED")
        .await?;
    batch_mgr.update_status(batch_id, "COMPLETED").await?;
    Ok(())
}

pub async fn load_roamout(
    batch_mgr: &batch::BatchManager,
    source_directory: &str,
    file_name: String,
) -> Result<(), AppError> {
    Logger::info("CALL ROAMOUT");
    let batch_id = batch_mgr
        .insert_batch("LOADER", "OUT", &file_name, "STARTED")
        .await?;

    let file_pattern = Regex::new(r"your_pattern_here")?;
    let batch_date = if file_pattern.is_match(&file_name) {
        let date_str = &file_name[13..21];
        format!(
            "{}-{}-{}",
            &date_str[0..4],
            &date_str[4..6],
            &date_str[6..8]
        )
    } else {
        batch_mgr.update_status(batch_id, "FAILED").await?;
        return Err(AppError::new(&format!(
            "Invalid file name format: {}",
            file_name
        )));
    };

    batch_mgr.update_status(batch_id, "COMPLETED").await?;
    Ok(())
}

pub async fn run(pool: Pool, config: AppConfig) -> Result<(), AppError> {
    let batch_mgr = batch::BatchManager::new(pool.clone());

    for source in &config.sources {
        println!("{:?}", source);

        match source.source_type.as_str() {
            "ROAM_IN" => {
                if let Some(file_name) = get_first_file(&source.source_directory) {
                    load_roamin(&batch_mgr, &source.source_directory, file_name.clone()).await?;
                }
            }
            "ROAM_OUT" => {
                if let Some(file_name) = get_first_file(&source.source_directory) {
                    load_roamout(&batch_mgr, &source.source_directory, file_name.clone()).await?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
