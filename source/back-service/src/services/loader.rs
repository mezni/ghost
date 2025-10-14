use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch;
use crate::services::config::Source;
use crate::services::file;
use crate::services::lookup;
use chrono::NaiveDateTime;
use chrono::format::ParseError;
use deadpool_postgres::Pool;
use regex::Regex;
use std::path::PathBuf;

const FILE_TO_PROCESS: usize = 5;

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

pub fn get_date_from_file_name(file_name: &str) -> String {
    let date_str = file_name.split('_').nth(2).unwrap_or("");
    if date_str.len() < 8 {
        return "1999-12-31".to_string();
    }
    let date_str = &date_str[..8];
    format!(
        "{}-{}-{}",
        &date_str[0..4],
        &date_str[4..6],
        &date_str[6..8]
    )
}

pub async fn load_roamout(
    pool: &Pool,
    batch_mgr: &batch::BatchManager,
    source_directory: &str,
    file_name: String,
    prefix_lookup: &lookup::PrefixLookup,
) -> Result<(), AppError> {
    let batch_id = batch_mgr
        .insert_batch("LOADER", "OUT", &file_name, "STARTED")
        .await?;

    let file_path = PathBuf::from(source_directory.clone()).join(&file_name);
    let records =
        file::RoamOutFileReader::read(file_path.to_str().unwrap()).map_err(AppError::from)?;
    let batch_date = get_date_from_file_name(&file_name);
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

pub async fn load(
    pool: &Pool,
    batch_mgr: &batch::BatchManager,
    source: &Source,
) -> Result<(), AppError> {
    let batch_mgr = batch::BatchManager::new(pool.clone());

    match source.source_type.as_str() {
        "ROAM_IN" => {
            println!("ROAM_IN");
        }
        "ROAM_OUT" => {
            println!("ROAM_OUT");
            let files = file::get_first_n_files(&source.source_directory, FILE_TO_PROCESS)
                .map_err(AppError::from)?;

            let filtered_files = if let Some(pattern) = &source.file_pattern {
                let regex = Regex::new(pattern).map_err(AppError::from)?;
                files.into_iter().filter(|f| regex.is_match(f)).collect()
            } else {
                files
            };

            let prefix_lookup = lookup::PrefixLookup::new(&pool)
                .await
                .map_err(AppError::from)?;

            for file_name in filtered_files {
                let date_str = get_date_from_file_name(&file_name);
                load_roamout(
                    &pool,
                    &batch_mgr,
                    &source.source_directory,
                    file_name.clone(),
                    &prefix_lookup,
                )
                .await?;
                //                load_roamout(&pool, batch_id, source_directory, file_name.clone(), prefix_lookup);
            }
        }
        _ => {}
    }

    Ok(())
}
