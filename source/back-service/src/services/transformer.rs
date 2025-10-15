use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch;
use chrono::NaiveDateTime;
use chrono::format::ParseError;
use deadpool_postgres::{Pool, Transaction};

const FILE_TO_PROCESS: usize = 5;

pub async fn transform(pool: &Pool, batch_mgr: &batch::BatchManager) -> Result<(), AppError> {
    let batch_mgr = batch::BatchManager::new(pool.clone());

    for i in (1..=FILE_TO_PROCESS).rev() {
        if let Some(corr_id) = batch_mgr.get_corr_id().await? {
            if corr_id != 0 {
                let batch_id = batch_mgr
                    .insert_batch("TRANSFORMER", "OUT", "", "STARTED")
                    .await?;

                // Insert metric values
                let insert_global_out_query = "
                INSERT INTO trx_metrics_global (metric_definition_id, batch_id, date_id, value)
                SELECT md.metric_definition_id, sro.batch_id, rd.date_id, COUNT(*) 
                FROM stg_roam_out sro 
                JOIN ref_dates rd ON rd.date_str = sro.batch_date
                JOIN (
                    SELECT rmd.metric_definition_id 
                    FROM ref_metric_definitions rmd 
                    JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
                    JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
                    WHERE rrd.direction = 'OUT'
                    AND rmt.name = 'GLOBAL'
                ) AS md ON TRUE
                WHERE sro.batch_id = $1 
                GROUP BY md.metric_definition_id, sro.batch_id, rd.date_id
            ";
                let client = pool.get().await?;
                client.execute(insert_global_out_query, &[&corr_id]).await?;

                let insert_country_out_query = "
                INSERT INTO trx_metrics_country (metric_definition_id, batch_id, date_id, country_id ,value)
                SELECT md.metric_definition_id, sro.batch_id, rd.date_id, sro.country_id, COUNT(*) 
                FROM stg_roam_out sro 
                JOIN ref_dates rd ON rd.date_str = sro.batch_date
                JOIN (
                    SELECT rmd.metric_definition_id 
                    FROM ref_metric_definitions rmd 
                    JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
                    JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
                    WHERE rrd.direction = 'OUT'
                    AND rmt.name = 'COUNTRY'
                ) AS md ON TRUE
                WHERE sro.batch_id = $1 
                GROUP BY md.metric_definition_id, sro.batch_id, rd.date_id, sro.country_id
            ";
                let client = pool.get().await?;
                client
                    .execute(insert_country_out_query, &[&corr_id])
                    .await?;

                let insert_operator_out_query = "
                INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id , operator_id,value)
                SELECT md.metric_definition_id, sro.batch_id, rd.date_id, sro.country_id, sro.operator_id, COUNT(*) 
                FROM stg_roam_out sro 
                JOIN ref_dates rd ON rd.date_str = sro.batch_date
                JOIN (
                    SELECT rmd.metric_definition_id 
                    FROM ref_metric_definitions rmd 
                    JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
                    JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
                    WHERE rrd.direction = 'OUT'
                    AND rmt.name = 'OPERATOR'
                ) AS md ON TRUE
                WHERE sro.batch_id = $1 
                GROUP BY md.metric_definition_id, sro.batch_id, rd.date_id, sro.country_id, sro.operator_id
            ";
                let client = pool.get().await?;
                client
                    .execute(insert_operator_out_query, &[&corr_id])
                    .await?;

                let delete_staging_query = "DELETE FROM stg_roam_out WHERE batch_id = $1";
                let client = pool.get().await?;
                client.execute(delete_staging_query, &[&corr_id]).await?;

                batch_mgr.update_corr_id(batch_id, corr_id).await?;

                batch_mgr.update_status(batch_id, "COMPLETED").await?;
            }
        }
    }

    Ok(())
}
