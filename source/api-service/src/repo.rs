use core::errors::AppError;
use tokio_postgres::Client;

const SELECT_LAST_DATE_QUERY: &str = "SELECT date_text
FROM dim_time
WHERE id = (
  SELECT MAX(date_id)
  FROM fct_roam_out
)";

const SELECT_LAST_ROAM_OUT_QUERY: &str = "
WITH latest AS (
  SELECT date_id, batch_id
  FROM fct_roam_out
  ORDER BY date_id DESC, batch_id DESC
  LIMIT 1
)
SELECT COUNT(*) AS cnt
FROM fct_roam_out t
JOIN latest l USING (date_id, batch_id)
";

const SELECT_ROAM_OUT_COUNTS_QUERY: &str = "
SELECT
  dt.date_text   AS date,
  COUNT(*)       AS count
FROM fct_roam_out fo
JOIN dim_time dt ON fo.date_id = dt.id
GROUP BY dt.date_text
ORDER BY dt.date_text;
";

pub async fn last_date(client: &Client) -> Result<String, AppError> {
    let row = client
        .query_one(SELECT_LAST_DATE_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result: String = row.get(0);

    Ok(result)
}

pub async fn last_roam_out(client: &Client) -> Result<i64, AppError> {
    let row = client
        .query_one(SELECT_LAST_ROAM_OUT_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result: i64 = row.get(0);

    Ok(result)
}

pub async fn roamout_by_date(client: &Client) -> Result<Vec<(String, i64)>, AppError> {
    let rows = client
        .query(SELECT_ROAM_OUT_COUNTS_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let results = rows
        .into_iter()
        .map(|row| {
            let date: String = row.get("date");
            let count: i64 = row.get("count");
            (date, count)
        })
        .collect();

    Ok(results)
}
