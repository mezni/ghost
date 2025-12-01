use core::errors::AppError;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

pub async fn get_next_batch_id(db_client: &Client) -> Result<Option<(i32, String)>, AppError> {
    let query = "
        SELECT batch_id, source_type
        FROM batch_execs
        WHERE batch_id = (
            SELECT MIN(batch_id)
            FROM (
                SELECT batch_id
                FROM batch_execs 
                WHERE batch_name = 'loader-srv'
                AND batch_status = 'Success'
                EXCEPT
                SELECT corr_id AS batch_id
                FROM batch_execs 
                WHERE batch_name = 'analytics-srv'
                AND batch_status = 'Success'
            ) AS unmatched_ids
        )
    ";

    let row = db_client
        .query_opt(query, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result = row.map(|r| {
        let id: i32 = r.get(0);
        let source_type: String = r.get(1);
        (id, source_type)
    });

    Ok(result)
}

pub async fn insert_batch_exec(
    db_client: &Client,
    batch_name: &str,
    source_type: &str,
    corr_id: i32,
) -> Result<i32, AppError> {
    let batch_status = "Started";

    let row = db_client
        .query_one(
            "INSERT INTO batch_execs (batch_name, source_type, corr_id, start_time, batch_status)
             VALUES ($1, $2, $3, NOW(), $4)
             RETURNING batch_id",
            &[&batch_name, &source_type, &corr_id, &batch_status],
        )
        .await?;

    let batch_id: i32 = row.get("batch_id");
    Ok(batch_id)
}

pub async fn update_batch_status(
    db_client: &Client,
    batch_id: i32,
    batch_status: &str,
) -> Result<(), AppError> {
    db_client
        .execute(
            "UPDATE batch_execs
             SET batch_status = $1, end_time = NOW()
             WHERE batch_id = $2",
            &[&batch_status, &batch_id],
        )
        .await?;

    Ok(())
}

pub async fn insert_roam_in_metrics(db_client: &Client, corr_id: i32) -> Result<(), AppError> {
    let query_sub_global = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_subscribers_in'),
            stg.batch_id,
            dat.date_id,
            SUM(nsub) AS value
        FROM stg_roam_in stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id
    ";

    db_client
        .execute(query_sub_global, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    let query_sub_country = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, country_id , value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_subscribers_in_by_country'),
            stg.batch_id,
            dat.date_id,
            stg.country_id, 
            SUM(nsub) AS value
        FROM stg_roam_in stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id, stg.country_id
    ";

    db_client
        .execute(query_sub_country, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    let query_sub_operator = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, country_id, operator_id , value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_subscribers_in_by_operator'),
            stg.batch_id,
            dat.date_id,
            stg.country_id, 
            stg.operator_id,
            SUM(nsub) AS value
        FROM stg_roam_in stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        LEFT JOIN operators ope ON stg.operator_id = ope.operator_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id, stg.country_id, stg.operator_id
        ";

    db_client
        .execute(query_sub_operator, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    let query_act_global = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_active_subscribers_in'),
            stg.batch_id,
            dat.date_id,
            SUM(nsuba) AS value
        FROM stg_roam_in stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id
    ";

    db_client
        .execute(query_act_global, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    let query_act_country = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, country_id , value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_active_subscribers_in_by_country'),
            stg.batch_id,
            dat.date_id,
            stg.country_id, 
            SUM(nsuba) AS value
        FROM stg_roam_in stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id, stg.country_id
    ";

    db_client
        .execute(query_act_country, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    let query_act_operator = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, country_id, operator_id , value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_active_subscribers_in_by_operator'),
            stg.batch_id,
            dat.date_id,
            stg.country_id, 
            stg.operator_id,
            SUM(nsuba) AS value
        FROM stg_roam_in stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        LEFT JOIN operators ope ON stg.operator_id = ope.operator_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id, stg.country_id, stg.operator_id
        ";

    db_client
        .execute(query_act_operator, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn insert_roam_out_metrics(db_client: &Client, corr_id: i32) -> Result<(), AppError> {
    let query_global = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_subscribers_out'),
            stg.batch_id,
            dat.date_id,
            COUNT(*) AS value
        FROM stg_roam_out stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id
    ";

    db_client
        .execute(query_global, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    let query_country = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, country_id , value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_subscribers_out_by_country'),
            stg.batch_id,
            dat.date_id,
            stg.country_id, 
            COUNT(*) AS value
        FROM stg_roam_out stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id, stg.country_id
    ";

    db_client
        .execute(query_country, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    let query_operator = "
        INSERT INTO metrics (metric_definition_id, batch_id , date_id, country_id, operator_id , value)
        SELECT 
            (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_subscribers_out_by_operator'),
            stg.batch_id,
            dat.date_id,
            stg.country_id, 
            stg.operator_id,
            COUNT(*) AS value
        FROM stg_roam_out stg
        LEFT JOIN countries cnt ON stg.country_id = cnt.country_id
        LEFT JOIN operators ope ON stg.operator_id = ope.operator_id
        JOIN dates dat ON stg.batch_date = dat.date_str 
        WHERE stg.operator_id IS DISTINCT FROM (
            SELECT operator_id FROM operators opr 
            JOIN countries cnt ON opr.country_id = cnt.country_id
            WHERE cnt.common_name = (SELECT value FROM global_config WHERE key='home_country')
            AND opr.operator = (SELECT value FROM global_config WHERE key='home_operator')  
        )
        AND stg.batch_id = $1
        GROUP BY stg.batch_id, dat.date_id, stg.country_id, stg.operator_id
        ";

    db_client
        .execute(query_operator, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn insert_roam_out_perfs(db_client: &Client, corr_id: i32) -> Result<(), AppError> {
    let query_global = "
        INSERT INTO roam_out_perf (date_id, batch_id, country_id, operator_id, country_count, operator_count, percent)
        SELECT
            d.date_id,
            t.batch_id,    
            t.country_id,
            t.operator_id,
            COUNT(*) AS count_by_country_operator,
            c.total_by_country,
            ROUND(100.0 * COUNT(*) / c.total_by_country, 2) AS percentage
        FROM stg_roam_out t
        JOIN (
            SELECT country_id, COUNT(*) AS total_by_country
            FROM stg_roam_out
            WHERE batch_id = $1
            GROUP BY country_id
        ) c ON t.country_id = c.country_id
        JOIN dates d ON t.batch_date = d.date_str
        WHERE t.batch_id = $1
        GROUP BY  d.date_id,t.batch_id, t.country_id, t.operator_id, c.total_by_country
        ORDER BY  d.date_id,t.batch_id, t.country_id, t.operator_id
    ";

    db_client
        .execute(query_global, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    let query_notif = "
            INSERT INTO notifications (date_id, batch_id, rule_id, ref_id, message) 
            SELECT date_id, batch_id, rule_id, ref_id, 
                '- ' || operator || ' ('|| common_name ||') config=' || perct_configure || ' reel=' || perct_reel 
            FROM v_roam_out_perf agg
            WHERE agg.batch_id = $1
            AND agg.perct_reel NOT BETWEEN COALESCE(perct_configure::float, 0) - 2 
                                    AND COALESCE(perct_configure::float, 0) + 2
            ORDER by agg.common_name, agg.operator 
    ";

    db_client
        .execute(query_notif, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn cleanup_roam_out_stg(db_client: &Client, corr_id: i32) -> Result<(), AppError> {
    let query = "
            DELETE FROM stg_roam_out WHERE batch_id = $1 
    ";

    db_client
        .execute(query, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn cleanup_roam_in_stg(db_client: &Client, corr_id: i32) -> Result<(), AppError> {
    let query = "
        DELETE FROM stg_roam_in WHERE batch_id = $1 
";

    db_client
        .execute(query, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}
