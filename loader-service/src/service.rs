use crate::config::{AppConfig, ServerConfig};
use crate::errors::AppError;
use crate::file::FileManager;
use crate::logger::Logger;
use crate::store::{StoreManager, insert_batch_exec, update_batch_execs};

const BATCH_STATUS_SUCCESS: &str = "Success";
const BATCH_STATUS_FAILURE: &str = "Failure";

pub struct LoadService {
    store_manager: StoreManager,
    file_manager: FileManager,
}

impl LoadService {
    pub async fn new(srv_config: ServerConfig, app_config: AppConfig) -> Result<Self, AppError> {
        let store_mgr = StoreManager::new(srv_config)?;
        let file_mgr = FileManager::new(app_config)?;

        Ok(LoadService {
            store_manager: store_mgr,
            file_manager: file_mgr,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        Logger::info(&format!("New exec "));

        let batch_id = match self.start_batch("TEST").await {
            Ok(id) => id,
            Err(e) => {
                return Err(e);
            }
        };

        if let Err(e) = self.update_batch(batch_id, BATCH_STATUS_SUCCESS).await {
            return Err(e);
        }

        Ok(())
    }

    pub async fn start_batch(&self, path_name: &str) -> Result<i32, AppError> {
        match insert_batch_exec(&self.store_manager, path_name).await {
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

    pub async fn update_batch(&self, batch_id: i32, status: &str) -> Result<(), AppError> {
        match update_batch_execs(&self.store_manager, batch_id, status).await {
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
