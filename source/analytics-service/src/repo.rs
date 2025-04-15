use core::db::DBManager;
use core::entities::RoamOutDB;
use core::errors::AppError;
use tokio_postgres::Client;

const INSERT_SOR_OUT_QUERY: &str = "
INSERT INTO fct_sor_out (date_id, batch_id, country_id, operator_id, country_count, operator_count, percent)
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
GROUP BY t.date_id, t.batch_id, t.country_id, t.operator_id, c.total_by_country
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

const INSERT_ANOMALIE_IMSI_QUERY: &str = "
INSERT INTO notifications (date_id, batch_id, rule_id, ref_id, message)
SELECT 
    fct.date_id, 
    fct.batch_id, 
    (SELECT id FROM rules WHERE name = 'imsi_is_not_local') AS rule_id,
    ims.id, 
    ims.imsi
FROM dim_imsi ims 
JOIN fct_roam_out fct ON fct.imsi_id = ims.id
JOIN dim_roam_type typ ON ims.roam_type_id = typ.id
WHERE typ.roam_type = 'OUT'
  AND ims.imsi IS NOT NULL
  AND fct.batch_id = $1
  AND ims.imsi NOT LIKE '60501%';
";

const INSERT_ANOMALIE_MSISDN_QUERY: &str = "
INSERT INTO notifications (date_id, batch_id, rule_id, ref_id, message)
SELECT 
    fct.date_id, 
    fct.batch_id, 
    (SELECT id FROM rules WHERE name = 'local_vlr_number') AS rule_id,
    msi.id, 
    msi.msisdn
FROM dim_msisdn msi 
JOIN fct_roam_out fct ON fct.msisdn_id = msi.id
JOIN dim_roam_type typ ON msi.roam_type_id = typ.id
WHERE typ.roam_type = 'OUT'
  AND msi.msisdn IS NOT NULL
  AND fct.batch_id = $1
  AND msi.msisdn NOT LIKE '216%';
";

const INSERT_ANOMALIE_SOR_DEVIATION_QUERY: &str = "
INSERT INTO notifications (date_id, batch_id, rule_id, ref_id, message) 
SELECT date_id, batch_id, rule_id, ref_id, 
       'operateur: ' || operator || ' config: ' || rate || ' reel: ' || percent 
FROM (
    SELECT 
        agg.date_id, 
        agg.batch_id, 
        (SELECT id FROM rules WHERE name = 'sor_percent_deviation') AS rule_id, 
        999999999 AS ref_id,
        pln.rate,
        agg.percent,
        ope.operator
    FROM (
        SELECT *
        FROM fct_sor_out fct
        WHERE batch_id = $1
          AND country_id IN (SELECT country_id FROM sor_plan)
    ) agg
    LEFT JOIN sor_plan pln 
        ON agg.country_id = pln.country_id 
       AND agg.operator_id = pln.operator_id
    JOIN dim_operators ope 
        ON pln.operator_id = ope.id
    WHERE agg.percent NOT BETWEEN COALESCE(pln.rate::float, 0) - 2 
                          AND COALESCE(pln.rate::float, 0) + 2
) deviations;
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

    Ok(row.and_then(|r| r.get(0)))
}

pub async fn insert_anomalie_imsi(client: &Client, batch_id: i32) -> Result<(), AppError> {
    client
        .execute(INSERT_ANOMALIE_IMSI_QUERY, &[&batch_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn insert_anomalie_msisdn(client: &Client, batch_id: i32) -> Result<(), AppError> {
    client
        .execute(INSERT_ANOMALIE_MSISDN_QUERY, &[&batch_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn insert_anomalie_sor_deviation(client: &Client, batch_id: i32) -> Result<(), AppError> {
    client
        .execute(INSERT_ANOMALIE_SOR_DEVIATION_QUERY, &[&batch_id])
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(())
}
