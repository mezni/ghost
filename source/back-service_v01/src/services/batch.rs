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

    pub async fn update_corr_id(&self, batch_id: i32, corr_id: i32) -> Result<(), AppError> {
        let client = self.pool.get().await?;
        client
            .execute(
                r#"
            UPDATE batch_execs
            SET corr_id = $1
            WHERE batch_id = $2
            "#,
                &[&corr_id, &batch_id], // Pass both parameters
            )
            .await?;
        Ok(())
    }

    pub async fn get_corr_id(&self) -> Result<Option<i32>, AppError> {
        let client = self.pool.get().await.map_err(AppError::from)?;
        let query = "SELECT min(be.batch_id) 
            FROM batch_execs be 
            LEFT JOIN batch_execs bec 
                ON be.batch_id = bec.corr_id AND bec.batch_status = 'COMPLETED'
            WHERE bec.corr_id IS NULL;";
        let row = client.query_opt(query, &[]).await.map_err(AppError::from)?;
        let corr_id = row.map(|r| r.get::<_, i32>(0));
        Ok(corr_id)
    }
}
