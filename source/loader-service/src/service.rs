use core::config::{AppConfig, ServerConfig};
use core::errors::AppError;
use core::file::FileManager;
use core::logger::Logger;
use core::store::StoreManager;

const SERVICE_NAME: &str = "loader-srv";

const SOURCE_TYPE_ROAM_IN: &str = "ROAM_IN";
const SOURCE_TYPE_ROAM_OUT: &str = "ROAM_OUT";

const BATCH_STATUS_SUCCESS: &str = "Success";
const BATCH_STATUS_FAILURE: &str = "Failure";

pub struct LoadService {
    store_manager: StoreManager,
    file_manager: FileManager,
}

impl LoadService {
    pub async fn new(srv_config: ServerConfig, app_config: AppConfig) -> Result<Self, AppError> {
        Logger::info(&format!("{} : init.", SERVICE_NAME));

        let store_mgr = match StoreManager::new(srv_config) {
            Ok(sm) => {
                Logger::info("Store - init");
                sm
            }
            Err(e) => {
                Logger::error(&format!("Store - failed: {:?}", e));
                return Err(e);
            }
        };

        let file_mgr = match FileManager::new(app_config) {
            Ok(fm) => {
                Logger::info("File - init");
                fm
            }
            Err(e) => {
                Logger::error(&format!("File - failed: {:?}", e));
                return Err(e);
            }
        };

        Ok(LoadService {
            store_manager: store_mgr,
            file_manager: file_mgr,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        Logger::info(&format!("{} : start.", SERVICE_NAME));

        let result = self.file_manager.next().await?;

        if let Some((_path, source_type)) = result {
            if source_type == SOURCE_TYPE_ROAM_IN {
                self.process_roam_in().await?;
            } else if source_type == SOURCE_TYPE_ROAM_OUT {
                self.process_roam_out().await?;
            }
        }

        Ok(())
    }

    pub async fn process_roam_in(&self) -> Result<(), AppError> {
        println!("roam_in");
        let batch_id = self
            .start_batch(SERVICE_NAME, SOURCE_TYPE_ROAM_IN, "TEST")
            .await?;
        self.update_batch(batch_id, BATCH_STATUS_SUCCESS).await?;
        Ok(())
    }

    pub async fn process_roam_out(&self) -> Result<(), AppError> {
        println!("roam_out");
        let batch_id = self
            .start_batch(SERVICE_NAME, SOURCE_TYPE_ROAM_OUT, "TEST")
            .await?;
        self.update_batch(batch_id, BATCH_STATUS_SUCCESS).await?;
        Ok(())
    }

    pub async fn start_batch(
        &self,
        path_name: &str,
        source_type: &str,
        action: &str,
    ) -> Result<i32, AppError> {
        let result = self
            .store_manager
            .insert_batch_exec(SERVICE_NAME, source_type, path_name)
            .await;

        match &result {
            Ok(batch_id) => Logger::info(&format!("Batch started: ID={}", batch_id)),
            Err(e) => Logger::error(&format!("Batch failed: {:?}", e)),
        }
        result
    }

    pub async fn update_batch(&self, batch_id: i32, batch_status: &str) -> Result<u64, AppError> {
        let result = self
            .store_manager
            .update_batch_exec(batch_id, Some(batch_status))
            .await;

        match &result {
            Ok(rows) => Logger::info(&format!(
                "Batch updated: ID={}, Status={}, Rows affected={}",
                batch_id, batch_status, rows
            )),
            Err(e) => Logger::error(&format!(
                "Batch update failed: ID={}, Error={:?}",
                batch_id, e
            )),
        }

        result
    }
}
