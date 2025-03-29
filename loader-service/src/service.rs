use crate::config::{AppConfig, ServerConfig};
use crate::entities::{RoamOutDAO, RoamOutDB};
use crate::errors::AppError;
use crate::file::FileManager;
use crate::logger::Logger;
use crate::store::{StoreManager, insert_batch_exec, update_batch_execs};

const BATCH_STATUS_SUCCESS: &str = "Success";
const BATCH_STATUS_FAILURE: &str = "Failure";
const ROAM_IN: &str = "ROAM_IN";
const ROAM_OUT: &str = "ROAM_OUT";

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
        let result = self.file_manager.next().await?;

        match result {
            Some((path, source_type)) => {
                if source_type == ROAM_IN {
                    self.process_roam_in(path).await?;
                } else if source_type == ROAM_OUT {
                    self.process_roam_out(path).await?;
                }
            }
            None => {}
        }

        Ok(())
    }

    pub async fn start_batch(&self, path_name: &str) -> Result<i32, AppError> {
        match insert_batch_exec(&self.store_manager, path_name).await {
            Ok(id) => {
                Logger::info(&format!("Batch ID: {} started", id));
                Ok(id)
            }
            Err(e) => {
                Logger::error(&format!("Batch start Failed: {}", e));
                Err(e)
            }
        }
    }

    pub async fn update_batch(&self, batch_id: i32, status: &str) -> Result<(), AppError> {
        match update_batch_execs(&self.store_manager, batch_id, status).await {
            Ok(_) => {
                Logger::info(&format!(
                    "Batch ID: {} updated with status: {}",
                    batch_id, status
                ));
                Ok(())
            }
            Err(e) => {
                Logger::error(&format!("Batch ID: {} failed to update : {}", batch_id, e));
                Err(e)
            }
        }
    }

    // Define your process_roam_in and process_roam_out methods
    async fn process_roam_in(&self, path: std::path::PathBuf) -> Result<(), AppError> {
        Logger::info(&format!("-> Process ROAM_IN: {}", path.display()));
        let batch_id = match self.start_batch("TEST").await {
            Ok(id) => id,
            Err(e) => {
                return Err(e);
            }
        };

        if let Err(e) = self.update_batch(batch_id, BATCH_STATUS_SUCCESS).await {
            return Err(e);
        }
        //        self.file_manager.process_roam_in(path).await;

        Ok(())
    }

    async fn process_roam_out(&self, path: std::path::PathBuf) -> Result<(), AppError> {
        Logger::info(&format!("-> Process ROAM_OUT: {}", path.display()));

        // Handle parent directory extraction as PathBuf
        let dir_name = match path.parent() {
            Some(d) => d.to_path_buf(), // Keep it as PathBuf
            None => {
                Logger::warn("No parent directory found for the given path.");
                return Err(AppError::Unexpected(
                    "No parent directory found for the given path.".to_string(),
                ));
            }
        };

        // Handle file name extraction
        let file_name = match path.file_name() {
            Some(f) => f.to_string_lossy().to_string(), // Convert to string for file name
            None => {
                Logger::warn("No file name found in the given path.");
                return Err(AppError::Unexpected(
                    "No file name found in the given path.".to_string(),
                ));
            }
        };

        let batch_id = self.start_batch(&file_name).await?;

        let records = self
            .file_manager
            .process_roam_out(dir_name, file_name)
            .await
            .map_err(|e| {
                Logger::error(&format!(
                    "Failed to process ROAM_OUT for {}: {}",
                    path.display(),
                    e
                ));
                AppError::Unexpected(format!("Failed to process ROAM_OUT: {}", e))
            })?;

        let mut db_records = Vec::new();
        for record in records {
            //            println!("{:?}", record);
            let db_record = RoamOutDB {
                batch_id: batch_id,
                batch_date: "2025-03-29".to_string(),
                imsi: record.imsi,
                msisdn: record.msisdn,
                vlr_number: record.vlr_number,
                carrier_name: "".to_string(),
                country_name: "".to_string(),
            };
            db_records.push(db_record);
        }

        // Update the batch status
        if let Err(e) = self.update_batch(batch_id, BATCH_STATUS_SUCCESS).await {
            return Err(e);
        }

        Ok(())
    }
}
