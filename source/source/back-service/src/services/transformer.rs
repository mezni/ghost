use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_manager::BatchManager;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Pool, Postgres, Row};

const BATCH_NAME: &str = "TRANSFORMER";

// ============================
// Structs
// ============================
#[derive(Debug)]
struct Rule {
    rule_id: i32,
    name: String,
}

// ============================
// Queries
// ============================
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

const DELETE_IN_QUERY: &str = r#"DELETE FROM stg_roam_in WHERE batch_id = $1"#;
const DELETE_OUT_QUERY: &str = r#"DELETE FROM stg_roam_out WHERE batch_id = $1"#;

const INSERT_PERF_OUT_QUERY: &str = r#"
    INSERT INTO trx_perf_out (
        batch_id, date_id, country_id, operator_id, 
        country_count, operator_count, target_percentage, actual_percentage
    )
    SELECT 
        op.batch_id,
        op.date_id,
        op.country_id,
        op.operator_id,
        co.country_count,
        op.operator_count,
        csp.rate::INT AS target_percentage,
        ROUND(((operator_count::DECIMAL / country_count::DECIMAL) * 100)::NUMERIC, 2) as actual_percentage
    FROM (
        SELECT 
            sro.batch_id,
            rd.date_id,
            sro.country_id,
            sro.operator_id,
            COUNT(*) as operator_count
        FROM stg_roam_out sro
        JOIN ref_dates rd ON rd.date_str = sro.batch_date
        WHERE sro.batch_id = $1
        GROUP BY sro.batch_id, rd.date_id, sro.country_id, sro.operator_id
    ) op
    JOIN (
        SELECT 
            sro.batch_id,
            rd.date_id,
            sro.country_id,
            COUNT(*) as country_count
        FROM stg_roam_out sro
        JOIN ref_dates rd ON rd.date_str = sro.batch_date
        WHERE sro.batch_id = $1
        GROUP BY sro.batch_id, rd.date_id, sro.country_id
    ) co ON op.batch_id = co.batch_id 
        AND op.date_id = co.date_id 
        AND op.country_id = co.country_id
    LEFT JOIN cfg_sor_plan csp ON csp.operator_id = op.operator_id
    ORDER BY 
        op.batch_id,
        op.date_id,
        op.country_id,
        op.operator_id;
"#;

// ============================
// Transformer Entrypoint
// ============================
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
                    process_perfs(pool, source_type, batch_id, id).await?;
                    process_alerts(pool, source_type, batch_id, id).await?;
                    process_clean(pool, source_type, batch_id, id).await?;
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

// ============================
// Process Metrics
// ============================
async fn process_metrics(
    pool: &Pool<Postgres>,
    source_type: &str,
    batch_id: i32,
    corr_id: i32,
) -> Result<(), AppError> {
    Logger::info(&format!(
        "Processing metrics [{}] batch_id={} corr_id={}",
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

    Logger::info(&format!(
        "✅ Metrics completed [{}] batch_id={} corr_id={}",
        source_type, batch_id, corr_id
    ));
    Ok(())
}

// ============================
// Process Alerts
// ============================
async fn process_alerts(
    pool: &Pool<Postgres>,
    source_type: &str,
    batch_id: i32,
    corr_id: i32,
) -> Result<(), AppError> {
    Logger::info(&format!(
        "Processing alerts [{}] batch_id={} corr_id={}",
        source_type, batch_id, corr_id
    ));

    let rows = sqlx::query("SELECT rule_id, name FROM ref_rules WHERE is_valid IS TRUE")
        .fetch_all(pool)
        .await
        .map_err(AppError::Sqlx)?;

    let rules: Vec<Rule> = rows
        .into_iter()
        .map(|row| Rule {
            rule_id: row.get("rule_id"),
            name: row.get("name"),
        })
        .collect();

    for rule in rules {
        if rule.name == "local_vlr_number" {
            Logger::info("Checking rule [local_vlr_number]");

            let select_query = r#"
                SELECT sri.batch_id, rd.date_id, SUM(nsub::INT) AS value
                FROM stg_roam_in sri 
                JOIN ref_dates rd ON rd.date_str = sri.batch_date
                WHERE sri.batch_id = $1
                AND sri.country_id IN (
                    SELECT country_id 
                    FROM cfg_countries 
                    WHERE upper(country_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_COUNTRY')
                )
                AND sri.operator_id IN (
                    SELECT operator_id 
                    FROM cfg_operators 
                    WHERE upper(operator_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_OPERATOR')
                )
                GROUP BY sri.batch_id, rd.date_id
            "#;

            if let Some(row) = sqlx::query(select_query)
                .bind(corr_id)
                .fetch_optional(pool)
                .await
                .map_err(AppError::Sqlx)?
            {
                let alert_batch_id: i32 = row.get("batch_id");
                let date_id: i32 = row.get("date_id");
                let value: i64 = row.get("value");

                Logger::info(&format!(
                    "🚨 ALERT triggered — batch_id={} date_id={} value={}",
                    alert_batch_id, date_id, value
                ));

                let insert_query = r#"
                    INSERT INTO trx_notifications (batch_id, date_id, rule_id, message)
                    VALUES ($1, $2, $3, $4)
                "#;

                let message = format!("ROAM IN: Local VLR number detected = {}", value);
                sqlx::query(insert_query)
                    .bind(alert_batch_id)
                    .bind(date_id)
                    .bind(rule.rule_id)
                    .bind(message)
                    .execute(pool)
                    .await
                    .map_err(AppError::Sqlx)?;
            }
        } else if rule.name == "sor_plan_deviation" {
            let select_query = r#"
                SELECT batch_id, date_id,COUNT(*) AS value
                FROM trx_perf_out 
                WHERE ABS(COALESCE(target_percentage, 0) - actual_percentage) > (
                    SELECT value::INT 
                    FROM ref_global_config 
                    WHERE key = 'deviance_interval'
                )
                AND target_percentage IS NOT NULL
                AND batch_id = $1
                GROUP BY batch_id, date_id
                            "#;

            if let Some(row) = sqlx::query(select_query)
                .bind(corr_id)
                .fetch_optional(pool)
                .await
                .map_err(AppError::Sqlx)?
            {
                let alert_batch_id: i32 = row.get("batch_id");
                let date_id: i32 = row.get("date_id");
                let value: i64 = row.get("value");

                Logger::info(&format!(
                    "🚨 ALERT triggered — batch_id={} date_id={} value={}",
                    alert_batch_id, date_id, value
                ));

                let insert_query = r#"
                    INSERT INTO trx_notifications (batch_id, date_id, rule_id, message)
                    VALUES ($1, $2, $3, $4)
                "#;

                let message = format!("ROAM OUT: SoR deviance detected = {}", value);
                sqlx::query(insert_query)
                    .bind(alert_batch_id)
                    .bind(date_id)
                    .bind(rule.rule_id)
                    .bind(message)
                    .execute(pool)
                    .await
                    .map_err(AppError::Sqlx)?;
            }
        }
    }

    Ok(())
}

// ============================
// Cleanup Stage
// ============================
async fn process_clean(
    pool: &Pool<Postgres>,
    source_type: &str,
    _batch_id: i32,
    corr_id: i32,
) -> Result<(), AppError> {
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

    sqlx::query(delete_query)
        .bind(corr_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;
    Logger::info(&format!(
        "🧹 Cleanup completed [{}] corr_id={}",
        source_type, corr_id
    ));
    Ok(())
}

// ============================
// Performance Metrics
// ============================
pub async fn process_perfs(
    pool: &Pool<Postgres>,
    source_type: &str,
    batch_id: i32,
    corr_id: i32,
) -> Result<(), AppError> {
    Logger::info(&format!(
        "Processing performance metrics for {} with batch_id={} corr_id={}",
        source_type, batch_id, corr_id
    ));

    sqlx::query(INSERT_PERF_OUT_QUERY)
        .bind(corr_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

    Logger::info(&format!(
        "Finished inserting performance metrics for {} with corr_id={}",
        source_type, corr_id
    ));

    Ok(())
}
