use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch;
use crate::services::config::{AppConfig, Source};
use crate::services::file;
use crate::services::lookup;
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
    pool: &Pool,
    batch_mgr: &batch::BatchManager,
    source: &Source,
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
    pool: &Pool,
    batch_mgr: &batch::BatchManager,
    source: &Source,
    file_name: String,
    prefix_lookup: lookup::PrefixLookup,
) -> Result<(), AppError> {
    Logger::info("CALL ROAMOUT");
    let batch_id = batch_mgr
        .insert_batch("LOADER", "OUT", &file_name, "STARTED")
        .await?;

    let batch_date = if let Some(pattern) = source.file_pattern.as_deref() {
        let file_pattern = Regex::new(pattern)?;
        if file_pattern.is_match(&file_name) {
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
        }
    } else {
        "1999-12-01".to_string()
    };

    let file_path = PathBuf::from(source.source_directory.clone()).join(file_name);
    let records =
        file::RoamOutFileReader::read(file_path.to_str().unwrap()).map_err(AppError::from)?;

    let mut batches = Vec::new();
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

    let mut client = pool.get().await.map_err(AppError::from)?;
    let transaction = client.transaction().await.map_err(AppError::from)?;

    let query = "INSERT INTO stg_roam_out (batch_id, batch_date, imsi, msisdn, vlr_number, prefix, country_id, operator_id)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
    for batch in batches {
        transaction
            .execute(
                query,
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
            .map_err(AppError::from)?;
    }

    transaction.commit().await.map_err(AppError::from)?;

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
                    load_roamin(&pool, &batch_mgr, source, file_name.clone()).await?;
                }
            }
            "ROAM_OUT" => {
                let prefix_lookup = lookup::PrefixLookup::new(&pool)
                    .await
                    .map_err(AppError::from)?;
                if let Some(file_name) = get_first_file(&source.source_directory) {
                    load_roamout(&pool, &batch_mgr, source, file_name.clone(), prefix_lookup)
                        .await?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
