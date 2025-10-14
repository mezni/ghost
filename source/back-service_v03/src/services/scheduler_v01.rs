use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch;
use crate::services::file;
use crate::services::lookup;
use chrono::Utc;
use deadpool_postgres::Pool;
use regex::Regex;
use std::path::PathBuf;

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

pub async fn run(pool: Pool) -> Result<(), AppError> {
    Logger::info("RUN");

    let prefix_lookup = lookup::PrefixLookup::new(&pool)
        .await
        .map_err(AppError::from)?;
    let batch_id = 1;

    let batch_mgr = batch::BatchManager::new(pool.clone());

    let file_pattern = Regex::new(r"^HSS\d{4}_\d{4}_\d{14}\.txt$").unwrap();
    let dir_name = "../../../WORK/INPUT/ROUT/";

    let file_name = "HSS9860_1549_20250912000000.txt";

    let batch_id = batch_mgr
        .insert_batch("LOADER", "OUT", file_name, "STARTED")
        .await?;

    let batch_date = if file_pattern.is_match(file_name) {
        let date_str = &file_name[13..21];
        format!(
            "{}-{}-{}",
            &date_str[0..4],
            &date_str[4..6],
            &date_str[6..8]
        )
    } else {
        return Err(AppError::from("Invalid file name"));
    };

    let file_path = PathBuf::from(dir_name).join(file_name);
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
    Logger::info("Batches loaded into stg_roam_out table");

    Ok(())
}
