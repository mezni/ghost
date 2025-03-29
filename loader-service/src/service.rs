use crate::config::{AppConfig, ServerConfig};
use crate::errors::AppError;
use crate::file::FileManager;
use crate::store::{StoreManager, insert_batch_exec};
use crate::logger::Logger;

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

    pub async fn start_batch(&self, path_name: &str) -> Result<i32, AppError> {
        Logger::info(&format!("Starting batch with name: {}", path_name));
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
}
