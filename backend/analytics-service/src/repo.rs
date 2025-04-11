use core::db::DBManager;
use core::entities::RoamOutDB;
use core::errors::AppError;
use tokio_postgres::Client;

const INSERT_SOR_OUT_QUERY: &str = "
INSERT INTO fct_sor_out (date_id, batch_id,country_id, operator_id, country_count, operator_count, percent)
SELECT
    t.date_id,
    t.batch_id,    
    t.country_id,
    t.operator_id,
    COUNT(*) AS count_by_country_operator,
    c.total_by_country,
    ROUND(100.0 * COUNT(*) / c.total_by_country, 2) AS percentage
FROM fct_roam_out t
JOIN (
    SELECT country_id, COUNT(*) AS total_by_country
    FROM fct_roam_out
    WHERE batch_id = $1
    GROUP BY country_id
) c ON t.country_id = c.country_id
WHERE t.batch_id = $1
GROUP BY t.date_id, t.batch_id, t.country_id, t.operator_id,c.total_by_country
ORDER BY t.date_id, t.batch_id, t.country_id, t.operator_id
";

const NEXT_CORR_ID_QUERY: &str = "
SELECT MIN(id)
FROM (
    SELECT id
    FROM batch_execs 
    WHERE batch_name = 'loader-srv'
      AND batch_status = 'Success'

    EXCEPT

    SELECT corr_id AS id
    FROM batch_execs 
    WHERE batch_name = 'analytics-srv'
      AND batch_status = 'Success'
) AS unmatched_ids;
";

pub async fn insert_fct_sor_out_records(client: &Client, corr_id: i32) -> Result<(), AppError> {
    client
        .execute(INSERT_SOR_OUT_QUERY, &[&corr_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn get_next_batch_id(client: &Client) -> Result<Option<i32>, AppError> {
    let row = client
        .query_opt(NEXT_CORR_ID_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(row.map(|r| r.get(0)))
}
