use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_mgr;
use deadpool_postgres::{Client, Pool};

const SOURCE_TYPE_IN: &str = "IN";
const SOURCE_TYPE_OUT: &str = "OUT";

const INSERT_GLOBAL_OUT_QUERY: &str = "
    INSERT INTO trx_metrics_global (metric_definition_id, batch_id, date_id, value)
    SELECT md.metric_definition_id, sro.batch_id, rd.date_id, COUNT(*) 
    FROM stg_roam_out sro 
    JOIN ref_dates rd ON rd.date_str = sro.batch_date
    JOIN (
        SELECT rmd.metric_definition_id 
        FROM ref_metric_definitions rmd 
        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
        JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
        WHERE rrd.direction = $1
        AND rmt.name = 'GLOBAL'
    ) AS md ON TRUE
    WHERE sro.batch_id = $2
    GROUP BY md.metric_definition_id, sro.batch_id, rd.date_id
";

const INSERT_GLOBAL_IN_QUERY: &str = "
    INSERT INTO trx_metrics_global (metric_definition_id, batch_id, date_id, value)
    SELECT md.metric_definition_id, sri.batch_id, rd.date_id, SUM(nsub::INT) 
    FROM stg_roam_in sri 
    JOIN ref_dates rd ON rd.date_str = sri.batch_date
    JOIN (
        SELECT rmd.metric_definition_id 
        FROM ref_metric_definitions rmd 
        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
        JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
        WHERE rrd.direction = $1
        AND rmt.name = 'GLOBAL'
    ) AS md ON TRUE
    WHERE sri.batch_id = $2
    GROUP BY md.metric_definition_id, sri.batch_id, rd.date_id
";

const INSERT_COUNTRY_OUT_QUERY: &str = "
    INSERT INTO trx_metrics_country (metric_definition_id, batch_id, date_id, country_id, value)
    SELECT md.metric_definition_id, sro.batch_id, rd.date_id, 
           sro.country_id , COUNT(*) 
    FROM stg_roam_out sro 
    JOIN ref_dates rd ON rd.date_str = sro.batch_date
    JOIN (
        SELECT rmd.metric_definition_id 
        FROM ref_metric_definitions rmd 
        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
        JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
        WHERE rrd.direction = $1
        AND rmt.name = 'COUNTRY'
    ) AS md ON TRUE
    WHERE sro.batch_id = $2
    GROUP BY md.metric_definition_id, sro.batch_id, rd.date_id, sro.country_id;
";

const INSERT_COUNTRY_IN_QUERY: &str = "
    INSERT INTO trx_metrics_country (metric_definition_id, batch_id, date_id, country_id, value)
    SELECT md.metric_definition_id, sri.batch_id, rd.date_id, 
           sri.country_id as country_id, SUM(nsub::INT) 
    FROM stg_roam_in sri 
    JOIN ref_dates rd ON rd.date_str = sri.batch_date
    JOIN (
        SELECT rmd.metric_definition_id 
        FROM ref_metric_definitions rmd 
        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
        JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
        WHERE rrd.direction = $1
        AND rmt.name = 'COUNTRY'
    ) AS md ON TRUE
    WHERE sri.batch_id = $2
    GROUP BY md.metric_definition_id, sri.batch_id, rd.date_id, sri.country_id
";

const INSERT_OPERATOR_OUT_QUERY: &str = "
    INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
    SELECT md.metric_definition_id, sro.batch_id, rd.date_id, sro.country_id,
           sro.operator_id, COUNT(*) 
    FROM stg_roam_out sro 
    JOIN ref_dates rd ON rd.date_str = sro.batch_date
    JOIN (
        SELECT rmd.metric_definition_id 
        FROM ref_metric_definitions rmd 
        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
        JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
        WHERE rrd.direction = $1
        AND rmt.name = 'OPERATOR'
    ) AS md ON TRUE
    WHERE sro.batch_id = $2
    GROUP BY md.metric_definition_id, sro.batch_id, rd.date_id, sro.country_id, sro.operator_id
";

const INSERT_OPERATOR_IN_QUERY: &str = "
    INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id,country_id, operator_id, value)
    SELECT md.metric_definition_id, sri.batch_id, rd.date_id, sri.country_id,
           sri.operator_id, SUM(nsub::INT)
    FROM stg_roam_in sri 
    JOIN ref_dates rd ON rd.date_str = sri.batch_date
    JOIN (
        SELECT rmd.metric_definition_id 
        FROM ref_metric_definitions rmd 
        JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
        JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
        WHERE rrd.direction = $1
        AND rmt.name = 'OPERATOR'
    ) AS md ON TRUE
    WHERE sri.batch_id = $2
    GROUP BY md.metric_definition_id, sri.batch_id, rd.date_id, sri.country_id, sri.operator_id
";

const DELETE_IN_QUERY: &str = "DELETE FROM stg_roam_in WHERE batch_id = $1";
const DELETE_OUT_QUERY: &str = "DELETE FROM stg_roam_out WHERE batch_id = $1";

pub async fn transform(pool: &Pool, batch_mgr: &batch_mgr::BatchManager) -> Result<(), AppError> {
    Logger::info("Starting data transformation process");

    for source_type in [SOURCE_TYPE_IN, SOURCE_TYPE_OUT] {
        Logger::info(&format!("Processing {} batches", source_type));
        
        while let Some(corr_id) = batch_mgr.get_corr_id(source_type.to_string()).await? {
            process_batch(pool, batch_mgr, source_type, corr_id).await?;
        }
    }

    Logger::info("Completed data transformation process");
    Ok(())
}

async fn process_batch(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    source_type: &str,
    corr_id: i32,
) -> Result<(), AppError> {
    Logger::debug(&format!("Processing {} batch ID: {}", source_type, corr_id));

    let batch_id = batch_mgr
        .insert_batch("TRANSFORMER", source_type, "", "STARTED")
        .await?;

    match perform_transformation(pool, source_type, corr_id).await {
        Ok(()) => {
            batch_mgr.update_corr_id(batch_id, corr_id).await?;
            batch_mgr.update_status(batch_id, "COMPLETED").await?;
            Logger::info(&format!(
                "Successfully transformed {} batch {}",
                source_type, corr_id
            ));
            Ok(())
        }
        Err(e) => {
            Logger::error(&format!(
                "Transformation failed for {} batch {}: {}",
                source_type, corr_id, e
            ));
            batch_mgr.update_status(batch_id, "FAILED").await?;
            Err(e)
        }
    }
}

async fn perform_transformation(
    pool: &Pool,
    source_type: &str,
    corr_id: i32,
) -> Result<(), AppError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (global_query, country_query, operator_query, delete_query) = match source_type {
        SOURCE_TYPE_IN => (
            INSERT_GLOBAL_IN_QUERY,
            INSERT_COUNTRY_IN_QUERY,
            INSERT_OPERATOR_IN_QUERY,
            DELETE_IN_QUERY,
        ),
        SOURCE_TYPE_OUT => (
            INSERT_GLOBAL_OUT_QUERY,
            INSERT_COUNTRY_OUT_QUERY,
            INSERT_OPERATOR_OUT_QUERY,
            DELETE_OUT_QUERY,
        ),
        _ => {
            return Err(AppError::Other(format!(
                "Invalid source type: {}",
                source_type
            )));
        }
    };

    // Execute all operations in sequence
    let global_rows = transaction
        .execute(global_query, &[&source_type, &corr_id])
        .await?;

    Logger::debug(&format!(
        "Inserted {} global rows for batch {}",
        global_rows, corr_id
    ));

    let country_rows = transaction
        .execute(country_query, &[&source_type, &corr_id])
        .await?;

    Logger::debug(&format!(
        "Inserted {} country rows for batch {}",
        country_rows, corr_id
    ));

    let operator_rows = transaction
        .execute(operator_query, &[&source_type, &corr_id])
        .await?;

    Logger::debug(&format!(
        "Inserted {} operator rows for batch {}",
        operator_rows, corr_id
    ));

    // Delete processed records from staging table
    let deleted_rows = transaction.execute(delete_query, &[&corr_id]).await?;

    Logger::debug(&format!(
        "Deleted {} staging rows for batch {}",
        deleted_rows, corr_id
    ));

    // Commit the transaction
    transaction.commit().await?;

    Logger::info(&format!(
        "Completed transformation for {} batch {}: Global({}) Country({}) Operator({}) Deleted({})",
        source_type, corr_id, global_rows, country_rows, operator_rows, deleted_rows
    ));

    Ok(())
}