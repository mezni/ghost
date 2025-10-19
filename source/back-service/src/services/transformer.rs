use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_manager::BatchManager;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

const BATCH_NAME: &str = "TRANSFORMER";
const BATCH_INSERT_SIZE: usize = 500;

const INSERT_GLOBAL_OUT_QUERY: &str = r#"
INSERT INTO trx_metrics_global (metric_definition_id, batch_id, date_id, value)
SELECT md.metric_definition_id, $3, rd.date_id, COUNT(*) 
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
GROUP BY md.metric_definition_id, rd.date_id
"#;

const INSERT_GLOBAL_IN_QUERY: &str = r#"
INSERT INTO trx_metrics_global (metric_definition_id, batch_id, date_id, value)
SELECT md.metric_definition_id, $3, rd.date_id, SUM(nsub::INT) 
FROM stg_roam_in sri
JOIN ref_dates rd ON rd.date_str = sri.batch_date
JOIN cfg_operators co ON co.operator_id = sri.operator_id
JOIN cfg_countries cc ON cc.country_id = co.country_id
JOIN (
    SELECT rmd.metric_definition_id 
    FROM ref_metric_definitions rmd 
    JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id 
    JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
    WHERE rrd.direction = $1
    AND rmt.name = 'GLOBAL'
) AS md ON TRUE
WHERE sri.batch_id = $2
AND sri.country_id NOT IN (
    SELECT country_id 
    FROM cfg_countries 
    WHERE upper(country_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_COUNTRY')
)
AND sri.operator_id NOT IN (
    SELECT operator_id 
    FROM cfg_operators 
    WHERE upper(operator_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_OPERATOR')
)
GROUP BY md.metric_definition_id, rd.date_id
"#;

const INSERT_COUNTRY_OUT_QUERY: &str = r#"
INSERT INTO trx_metrics_country (metric_definition_id, batch_id, date_id, country_id, value)
SELECT md.metric_definition_id, $3, rd.date_id, sro.country_id, COUNT(*) 
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
GROUP BY md.metric_definition_id, sro.batch_id, rd.date_id, sro.country_id
"#;

const INSERT_COUNTRY_IN_QUERY: &str = r#"
INSERT INTO trx_metrics_country (metric_definition_id, batch_id, date_id, country_id, value)
SELECT md.metric_definition_id, $3, rd.date_id, sri.country_id, SUM(nsub::INT)
FROM stg_roam_in sri
JOIN ref_dates rd ON rd.date_str = sri.batch_date
JOIN cfg_operators co ON co.operator_id = sri.operator_id
JOIN cfg_countries cc ON cc.country_id = co.country_id
JOIN (
    SELECT rmd.metric_definition_id
    FROM ref_metric_definitions rmd
    JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
    JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
    WHERE rrd.direction = $1
    AND rmt.name = 'COUNTRY'
) AS md ON TRUE
WHERE sri.batch_id = $2
AND sri.country_id NOT IN (
    SELECT country_id FROM cfg_countries
    WHERE upper(country_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_COUNTRY')
)
AND sri.operator_id NOT IN (
    SELECT operator_id FROM cfg_operators
    WHERE upper(operator_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_OPERATOR')
)
GROUP BY md.metric_definition_id, sri.batch_id, rd.date_id, sri.country_id
"#;

const INSERT_OPERATOR_OUT_QUERY: &str = r#"
INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT md.metric_definition_id, $3, rd.date_id, sro.country_id, sro.operator_id, COUNT(*)
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
"#;

const INSERT_OPERATOR_IN_QUERY: &str = r#"
INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT md.metric_definition_id, $3, rd.date_id, sri.country_id, sri.operator_id, SUM(nsub::INT)
FROM stg_roam_in sri
JOIN ref_dates rd ON rd.date_str = sri.batch_date
JOIN cfg_operators co ON co.operator_id = sri.operator_id
JOIN cfg_countries cc ON cc.country_id = co.country_id
JOIN (
    SELECT rmd.metric_definition_id
    FROM ref_metric_definitions rmd
    JOIN ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
    JOIN ref_metric_types rmt ON rmt.metric_type_id = rmd.metric_type_id
    WHERE rrd.direction = $1
    AND rmt.name = 'OPERATOR'
) AS md ON TRUE
WHERE sri.batch_id = $2
AND sri.country_id NOT IN (
    SELECT country_id FROM cfg_countries
    WHERE upper(country_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_COUNTRY')
)
AND sri.operator_id NOT IN (
    SELECT operator_id FROM cfg_operators
    WHERE upper(operator_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_OPERATOR')
)
GROUP BY md.metric_definition_id, sri.batch_id, rd.date_id, sri.country_id, sri.operator_id
"#;

const DELETE_IN_QUERY: &str = r#"
DELETE FROM stg_roam_in WHERE batch_id = $1
"#;
const DELETE_OUT_QUERY: &str = r#"
DELETE FROM stg_roam_out WHERE batch_id = $1
"#;

pub async fn run(pool: &Pool<Postgres>, batch_mgr: &BatchManager) -> Result<(), AppError> {
    let mut last_batch_id = 0;
    for source_type in ["IN", "OUT"] {
        loop {
            let corr_id = batch_mgr
                .get_corr_id(source_type.to_string(), last_batch_id)
                .await?;
            match corr_id {
                Some(id) => {
                    let batch_id = batch_mgr.batch_start(BATCH_NAME, source_type, "").await?;
                    process_metrics(pool, source_type, batch_id, id).await?;
                    process_alerts(pool, source_type, batch_id, id).await?;
                    batch_mgr.update_corr_id(batch_id, id).await?;
                    batch_mgr.batch_succeeded(batch_id).await?;
                    last_batch_id = id;
                }
                None => break,
            }
        }
    }
    Ok(())
}

async fn process_metrics(
    pool: &Pool<Postgres>,
    source_type: &str,
    batch_id: i32,
    corr_id: i32,
) -> Result<(), AppError> {
    Logger::info(&format!(
        "Processing metrics for {} with batch_id={} corr_id={}",
        source_type, batch_id, corr_id
    ));

    let global_query = match source_type {
        "IN" => INSERT_GLOBAL_IN_QUERY,
        "OUT" => INSERT_GLOBAL_OUT_QUERY,
        _ => {
            return Err(AppError::new(format!(
                "Unknown source_type {}",
                source_type
            )));
        }
    };

    let country_query = match source_type {
        "IN" => INSERT_COUNTRY_IN_QUERY,
        "OUT" => INSERT_COUNTRY_OUT_QUERY,
        _ => {
            return Err(AppError::new(format!(
                "Unknown source_type {}",
                source_type
            )));
        }
    };

    let operator_query = match source_type {
        "IN" => INSERT_OPERATOR_IN_QUERY,
        "OUT" => INSERT_OPERATOR_OUT_QUERY,
        _ => {
            return Err(AppError::new(format!(
                "Unknown source_type {}",
                source_type
            )));
        }
    };

    let delete_query = match source_type {
        "IN" => DELETE_IN_QUERY,
        "OUT" => DELETE_OUT_QUERY,
        _ => {
            return Err(AppError::new(format!(
                "Unknown source_type {}",
                source_type
            )));
        }
    };

    sqlx::query(global_query)
        .bind(source_type)
        .bind(corr_id)
        .bind(batch_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

    sqlx::query(country_query)
        .bind(source_type)
        .bind(corr_id)
        .bind(batch_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

    sqlx::query(operator_query)
        .bind(source_type)
        .bind(corr_id)
        .bind(batch_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

    sqlx::query(delete_query)
        .bind(corr_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

    Logger::info(&format!(
        "Metrics processed for {} with batch_id={} corr_id={}",
        source_type, batch_id, corr_id
    ));

    Ok(())
}

// ------------------------
// Process Alerts (placeholder)
// ------------------------
async fn process_alerts(
    _pool: &Pool<Postgres>,
    _source_type: &str,
    _batch_id: i32,
    _corr_id: i32,
) -> Result<(), AppError> {
    // Add alert logic here
    Ok(())
}
