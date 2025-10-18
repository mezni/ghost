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

    pub async fn update_corr_id(&self, batch_id: i32, corr_id: i32) -> Result<(), AppError> {
        let client = self.pool.get().await?;
        client
            .execute(
                r#"
                UPDATE batch_execs
                SET corr_id = $1
                WHERE batch_id = $2
                "#,
                &[&corr_id, &batch_id],
            )
            .await?;
        Ok(())
    }

    pub async fn get_corr_id(&self, batch_source: String) -> Result<Option<i32>, AppError> {
        let client = self.pool.get().await?;
        let query = "
            SELECT MIN(batch_id) as min_batch_id
            FROM batch_execs bec
            WHERE bec.batch_status = 'COMPLETED'
            AND bec.batch_name = 'LOADER'
            AND bec.source_type = $1
            AND batch_id NOT IN (SELECT corr_id FROM batch_execs be WHERE corr_id IS NOT NULL)
        ";

        let row = client.query_opt(query, &[&batch_source]).await?;

        // Handle the case where MIN returns NULL (no rows found)
        match row {
            Some(row) => {
                // Get the value as Option<i32> to handle NULL properly
                let min_batch_id: Option<i32> = row.get("min_batch_id");
                Ok(min_batch_id)
            }
            None => Ok(None),
        }
    }
}
