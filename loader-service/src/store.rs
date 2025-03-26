use crate::db::DBPool;
use crate::errors::AppError;
use tokio_postgres::Row;

const BATCH_STATUS_START: &str = "Started";

/// Inserts a new batch exec and returns the generated ID
pub async fn insert_batch_exec(db: &DBPool, path_name: &str) -> Result<i32, AppError> {
    let client = db.get_client().await?;
    let query = "INSERT INTO batch_execs (batch_name, start_time, batch_status) VALUES ($1, NOW(), $2) RETURNING id";
    let row = client.query_one(query, &[&path_name, &BATCH_STATUS_START]).await?;
    let id: i32 = row.try_get("id")?;
    Ok(id)
}

pub async fn update_batch_execs(
    db: &DBPool,
    batch_id: i32,
    status: &str,
) -> Result<u64, AppError> {
    let client = db.get_client().await?;
    let query = "UPDATE batch_execs SET batch_status = $1, end_time = NOW() WHERE id = $2";
    let rows_affected = client.execute(query, &[&status, &batch_id]).await?;
    Ok(rows_affected as u64)
}