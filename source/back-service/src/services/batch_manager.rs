// services/batch_manager.rs
use crate::core::db::Db;
use crate::core::errors::AppError;
use sqlx::{PgPool, Row}; // Row needed for try_get
use std::sync::Arc;

#[derive(Clone)]
pub struct BatchManager {
    pool: Arc<PgPool>,
}

impl BatchManager {
    /// Create a new BatchManager with a shared pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Insert a new batch execution and return the generated batch_id
    pub async fn insert_batch_exec(
        &self,
        batch_name: &str,
        source_type: &str,
        source_name: &str,
        batch_status: &str,
    ) -> Result<i32, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO batch_execs (batch_name, source_type, source_name, batch_status, start_time)
            VALUES ($1, $2, $3, $4, NOW())
            RETURNING batch_id
            "#,
        )
        .bind(batch_name)
        .bind(source_type)
        .bind(source_name)
        .bind(batch_status)
        .fetch_one(&*self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        let batch_id: i32 = row.try_get("batch_id")?;
        Ok(batch_id)
    }

    /// Update the status and set end_time = NOW()
    pub async fn update_batch_exec(
        &self,
        batch_id: i32,
        batch_status: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE batch_execs SET batch_status = $1, end_time = NOW() WHERE batch_id = $2",
        )
        .bind(batch_status)
        .bind(batch_id)
        .execute(&*self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(())
    }

    /// Retrieve a batch execution by ID
    pub async fn get_batch_exec(&self, batch_id: i32) -> Result<BatchExec, AppError> {
        let row = sqlx::query(
            "SELECT batch_id, batch_name, source_type, source_name, batch_status
             FROM batch_execs
             WHERE batch_id = $1",
        )
        .bind(batch_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(BatchExec {
            batch_id: row.try_get("batch_id")?,
            batch_name: row.try_get("batch_name")?,
            source_type: row.try_get("source_type")?,
            source_name: row.try_get("source_name")?,
            batch_status: row.try_get("batch_status")?,
        })
    }

    /// Convenience method to create a manager using the global Db pool
    pub async fn from_global() -> Result<Self, AppError> {
        let pool = Db::pool().await?;
        Ok(Self {
            pool: Arc::new(pool.clone()),
        })
    }

    // ===========================
    // Helper functions
    // ===========================

    /// Start a new batch
    pub async fn batch_start(
        &self,
        batch_name: &str,
        source_type: &str,
        source_name: &str,
    ) -> Result<i32, AppError> {
        self.insert_batch_exec(batch_name, source_type, source_name, "START")
            .await
    }

    /// Mark batch as FAILED
    pub async fn batch_failed(&self, batch_id: i32) -> Result<(), AppError> {
        self.update_batch_exec(batch_id, "FAILED").await
    }

    /// Mark batch as SUCCESS
    pub async fn batch_succeeded(&self, batch_id: i32) -> Result<(), AppError> {
        self.update_batch_exec(batch_id, "SUCCESS").await
    }

    // ===========================
    // Correlation ID functions
    // ===========================

    /// Update corr_id of a batch
    pub async fn update_corr_id(&self, batch_id: i32, corr_id: i32) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE batch_execs
            SET corr_id = $1
            WHERE batch_id = $2
            "#,
        )
        .bind(corr_id)
        .bind(batch_id)
        .execute(&*self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(())
    }

    /// Get the MIN(batch_id) for a completed LOADER batch of a given source_type not already used as corr_id
    pub async fn get_corr_id(
        &self,
        batch_source: String,
        last_batch_id: i32,
    ) -> Result<Option<i32>, AppError> {
        let row = sqlx::query(
            r#"
        SELECT MIN(batch_id) as min_batch_id
        FROM batch_execs bec
        WHERE bec.batch_status = 'SUCCESS'
        AND bec.batch_name = 'LOADER'
        AND bec.source_type = $1
        AND batch_id > $2
        AND batch_id NOT IN (
            SELECT corr_id FROM batch_execs be WHERE corr_id IS NOT NULL
        )
        "#,
        )
        .bind(batch_source)
        .bind(last_batch_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        if let Some(row) = row {
            let min_batch_id: Option<i32> = row.try_get("min_batch_id")?;
            Ok(min_batch_id)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatchExec {
    pub batch_id: i32,
    pub batch_name: String,
    pub source_type: String,
    pub source_name: String,
    pub batch_status: String,
}
