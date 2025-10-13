use crate::core::db::Db;
use crate::core::errors::AppError;
use deadpool_postgres::Pool;

/// Simple batch manager
pub struct BatchManager {
    pool: Pool,
}

impl BatchManager {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Insert a new batch
    pub async fn insert_batch(
        &self,
        batch_name: &str,
        source_type: &str,
        source_name: &str,
        status: &str,
    ) -> Result<i32, AppError> {
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                r#"
                INSERT INTO batch_execs (batch_name, source_type, source_name, batch_status, start_time)
                VALUES ($1, $2, $3, $4, NOW())
                RETURNING batch_id
                "#,
                &[&batch_name, &source_type, &source_name, &status],
            )
            .await?;

        Ok(row.get("batch_id"))
    }

    /// Update batch status and end_time
    pub async fn update_status(&self, batch_id: i32, status: &str) -> Result<(), AppError> {
        let client = self.pool.get().await?;
        client
            .execute(
                r#"
                UPDATE batch_execs
                SET batch_status = $1, end_time = NOW()
                WHERE batch_id = $2
                "#,
                &[&status, &batch_id],
            )
            .await?;
        Ok(())
    }
}
