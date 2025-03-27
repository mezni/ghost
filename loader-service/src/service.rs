use crate::errors::AppError;
use crate::logger::Logger;
use crate::store::{DBPool, insert_batch_exec, update_batch_execs};

pub struct LoadService {
    pool: DBPool,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        let pool = DBPool::new()?;
        Logger::info("Database connection pool initialized.");
        Ok(LoadService { pool })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let path_name = "TEST";
        let status = "TEST";

        let batch_id = self.start_batch(path_name).await?;
        self.update_batch_status(batch_id, status).await?;
        Ok(())
    }

    async fn start_batch(&self, path_name: &str) -> Result<i32, AppError> {
        Logger::info(&format!("Starting batch with name: {}", path_name));
        match insert_batch_exec(&self.pool, path_name).await {
            Ok(id) => {
                Logger::info(&format!("Batch started with ID: {}", id));
                Ok(id)
            }
            Err(e) => {
                Logger::error(&format!("Failed to start batch: {}", e));
                Err(e)
            }
        }
    }

    async fn update_batch_status(&self, batch_id: i32, status: &str) -> Result<(), AppError> {
        Logger::info(&format!(
            "Ending batch {} with status: {}",
            batch_id, status
        ));
        match update_batch_execs(&self.pool, batch_id, status).await {
            Ok(_) => {
                Logger::info(&format!(
                    "Batch {} updated with status: {}",
                    batch_id, status
                ));
                Ok(())
            }
            Err(e) => {
                Logger::error(&format!("Failed to update batch {}: {}", batch_id, e));
                Err(e)
            }
        }
    }
}
