use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_mgr;
use deadpool_postgres::Pool;
use tokio_postgres::Transaction;

const SOURCE_TYPE_IN: &str = "IN";
const SOURCE_TYPE_OUT: &str = "OUT";

const INSERT_GLOBAL_OUT_QUERY: &str = "
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
";

const INSERT_GLOBAL_IN_QUERY: &str = "
    INSERT INTO trx_metrics_global (metric_definition_id, batch_id, date_id, value)
    SELECT md.metric_definition_id, $3, rd.date_id, SUM(nsub::INT) 
    FROM stg_roam_in sri 
    JOIN ref_dates rd ON rd.date_str = sri.batch_date
    JOIN cfg_operators co ON co.operator_id = sri.operator_id
    JOIN cfg_countries cc ON cc.country_id =  co.country_id
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
";

const INSERT_COUNTRY_OUT_QUERY: &str = "
    INSERT INTO trx_metrics_country (metric_definition_id, batch_id, date_id, country_id, value)
    SELECT md.metric_definition_id, $3, rd.date_id, 
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
    SELECT md.metric_definition_id, $3, rd.date_id, 
           sri.country_id as country_id, SUM(nsub::INT) 
    FROM stg_roam_in sri 
    JOIN ref_dates rd ON rd.date_str = sri.batch_date
    JOIN cfg_operators co ON co.operator_id = sri.operator_id
    JOIN cfg_countries cc ON cc.country_id =  co.country_id
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
        SELECT country_id 
        FROM cfg_countries 
        WHERE upper(country_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_COUNTRY')
    )
    AND sri.operator_id NOT IN (
        SELECT operator_id 
        FROM cfg_operators 
        WHERE upper(operator_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_OPERATOR')
    )
    GROUP BY md.metric_definition_id, sri.batch_id, rd.date_id, sri.country_id
";

const INSERT_OPERATOR_OUT_QUERY: &str = "
    INSERT INTO trx_metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
    SELECT md.metric_definition_id, $3, rd.date_id, sro.country_id,
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
    SELECT md.metric_definition_id, $3, rd.date_id, sri.country_id,
           sri.operator_id, SUM(nsub::INT)
    FROM stg_roam_in sri 
    JOIN ref_dates rd ON rd.date_str = sri.batch_date
    JOIN cfg_operators co ON co.operator_id = sri.operator_id
    JOIN cfg_countries cc ON cc.country_id =  co.country_id
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
        SELECT country_id 
        FROM cfg_countries 
        WHERE upper(country_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_COUNTRY')
    )
    AND sri.operator_id NOT IN (
        SELECT operator_id 
        FROM cfg_operators 
        WHERE upper(operator_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_OPERATOR')
    )
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

    // Process metrics and cleanup in a single transaction
    if let Err(e) = execute_transformation(pool, source_type, corr_id, batch_id).await {
        Logger::error(&format!(
            "Transformation failed for {} batch {}: {}",
            source_type, corr_id, e
        ));
        batch_mgr.update_status(batch_id, "FAILED").await?;
        return Err(e);
    }

    batch_mgr.update_status(batch_id, "COMPLETED").await?;
    Logger::debug(&format!(
        "Completed processing {} batch ID: {}",
        source_type, corr_id
    ));
    Ok(())
}

async fn execute_transformation(
    pool: &Pool,
    source_type: &str,
    corr_id: i32,
    batch_id: i32,
) -> Result<(), AppError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    // Process metrics
    process_metrics(&transaction, source_type, corr_id, batch_id).await?;

    // Process metrics
    process_others(&transaction, source_type, corr_id, batch_id).await?;

    // Cleanup staging data
    cleanup(&transaction, source_type, corr_id).await?;

    // Commit everything or rollback on error
    transaction.commit().await?;

    Ok(())
}

async fn process_others(
    transaction: &Transaction<'_>,
    source_type: &str,
    corr_id: i32,
    batch_id: i32,
) -> Result<(), AppError> {
    let query = "SELECT sri.batch_id, rd.date_id, COUNT(*) AS value
                 FROM stg_roam_in sri 
                 JOIN ref_dates rd ON rd.date_str = sri.batch_date
                 JOIN cfg_operators co ON co.operator_id = sri.operator_id
                 JOIN cfg_countries cc ON cc.country_id =  co.country_id
                 WHERE sri.batch_id = $1
                 AND sri.country_id  IN (
                     SELECT country_id 
                     FROM cfg_countries 
                     WHERE upper(country_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_COUNTRY')
                 )
                 AND sri.operator_id  IN (
                     SELECT operator_id 
                     FROM cfg_operators 
                     WHERE upper(operator_name) = (SELECT upper(value) FROM ref_global_config WHERE upper(key) = 'HOME_OPERATOR')
                 ) GROUP BY sri.batch_id, rd.date_id";

    let rows = transaction.query(query, &[&corr_id]).await?;

    let rule_id_query = "SELECT rule_id FROM ref_rules WHERE name = $1";
    let rule_id_row = transaction
        .query_one(rule_id_query, &[&"local_vlr_number"])
        .await?;
    let rule_id: i32 = rule_id_row.get("rule_id");

    for row in rows {
        let batch_id: i32 = row.get("batch_id");
        let date_id: i32 = row.get("date_id");
        let value: i64 = row.get("value");

        if value == 0 {
            let message = format!("ROAM IN contient {} enregistrements local", value);
            let insert_query = "INSERT INTO trx_notifications (batch_id, date_id, rule_id, message) VALUES ($1, $2, $3, $4)";
            transaction
                .execute(insert_query, &[&batch_id, &date_id, &rule_id, &message])
                .await?;
        }
    }

    Logger::debug(&format!("Processed"));

    Ok(())
}

async fn process_metrics(
    transaction: &Transaction<'_>,
    source_type: &str,
    corr_id: i32,
    batch_id: i32,
) -> Result<(), AppError> {
    let queries = match source_type {
        SOURCE_TYPE_IN => (
            INSERT_GLOBAL_IN_QUERY,
            INSERT_COUNTRY_IN_QUERY,
            INSERT_OPERATOR_IN_QUERY,
        ),
        SOURCE_TYPE_OUT => (
            INSERT_GLOBAL_OUT_QUERY,
            INSERT_COUNTRY_OUT_QUERY,
            INSERT_OPERATOR_OUT_QUERY,
        ),
        _ => {
            return Err(AppError::Other(format!(
                "Invalid source type: {}",
                source_type
            )));
        }
    };

    // Execute all metric insertions
    for (query, name) in [
        (queries.0, "global"),
        (queries.1, "country"),
        (queries.2, "operator"),
    ] {
        let rows = transaction
            .execute(query, &[&source_type, &corr_id, &batch_id])
            .await?;

        Logger::debug(&format!(
            "Inserted {} {} rows for batch {}",
            rows, name, corr_id
        ));
    }

    Ok(())
}

async fn cleanup(
    transaction: &Transaction<'_>,
    source_type: &str,
    corr_id: i32,
) -> Result<(), AppError> {
    let delete_query = match source_type {
        SOURCE_TYPE_IN => DELETE_IN_QUERY,
        SOURCE_TYPE_OUT => DELETE_OUT_QUERY,
        _ => {
            return Err(AppError::Other(format!(
                "Invalid source type: {}",
                source_type
            )));
        }
    };

    let deleted_rows = transaction.execute(delete_query, &[&corr_id]).await?;

    Logger::debug(&format!(
        "Deleted {} staging rows for batch {}",
        deleted_rows, corr_id
    ));

    Ok(())
}
