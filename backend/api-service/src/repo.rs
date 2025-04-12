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
