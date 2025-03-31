use crate::config::{AppConfig, ServerConfig};
use crate::entities::{Prefixes, RoamOutDAO, RoamOutDB};
use crate::errors::AppError;
use crate::file::FileManager;
use crate::logger::Logger;
use crate::store::{
    StoreManager, insert_batch_exec, insert_roam_out_stg, select_all_prefixes, update_batch_execs,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

    pub async fn prefix_map(&self) -> Result<HashMap<String, (String, String)>, AppError> {
        // Call select_all_prefixes to get data
        let prefixes = select_all_prefixes(&self.store_manager).await?;

        // Construct the HashMap
        let prefix_map: HashMap<String, (String, String)> = prefixes
            .into_iter()
            .map(|p| (p.prefix, (p.carrier_name, p.country_name)))
            .collect();

        Ok(prefix_map)
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let result = self.file_manager.next().await?;

        match result {
            Some((path, source_type)) => {
                let prefix_map = self.prefix_map().await?;

                if source_type == ROAM_IN {
                    match self.process_roam_in(path, &prefix_map).await {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    }
                } else if source_type == ROAM_OUT {
                    match self.process_roam_out(path, &prefix_map).await {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    }
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

    async fn process_roam_in(
        &self,
        path: PathBuf,
        prefix_map: &HashMap<String, (String, String)>,
    ) -> Result<(), AppError> {
        Logger::info(&format!("-> Processing ROAM_IN: {}", path.display()));

        // Start batch with actual filename
        let file_name = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("Unknown File");

        let batch_id = self.start_batch(file_name).await?;

        Ok(())
    }

    async fn process_roam_out(
        &self,
        path: std::path::PathBuf,
        prefix_map: &std::collections::HashMap<String, (String, String)>,
    ) -> Result<(), AppError> {
        Logger::info(&format!("-> Process ROAM_OUT: {}", path.display()));

        let (dir_name, file_name) = match extract_dir_and_file_name(&path) {
            Ok((dir, file)) => (dir, file),
            Err(e) => return Err(e),
        };

        let batch_id = self.start_batch(&file_name).await?;

        let records = self
            .file_manager
            .process_roam_out(dir_name, file_name.to_string())
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
            let prefix = lookup(&prefix_map, record.vlr_number.clone()); // Pass the prefix_map and vlr_number

            // Create a RoamOutDB entry and assign values from the prefix lookup
            let db_record = RoamOutDB {
                batch_id: batch_id,
                batch_date: "2025-03-29".to_string(),
                imsi: record.imsi,
                msisdn: record.msisdn,
                vlr_number: record.vlr_number,
                carrier_name: prefix.carrier_name, // Get carrier_name from the lookup result
                country_name: prefix.country_name, // Get country_name from the lookup result
            };

            db_records.push(db_record);
        }

        insert_roam_out_stg(&self.store_manager, db_records).await?;

        // Update the batch status
        if let Err(e) = self.update_batch(batch_id, BATCH_STATUS_SUCCESS).await {
            return Err(e);
        }

        Ok(())
    }
}

pub fn lookup(prefix_map: &HashMap<String, (String, String)>, mut s: String) -> Prefixes {
    while !s.is_empty() {
        if let Some((carrier_name, country_name)) = prefix_map.get(&s) {
            return Prefixes {
                prefix: s.clone(),
                carrier_name: carrier_name.clone(),
                country_name: country_name.clone(),
            };
        }
        s.pop(); // Remove the last character for prefix matching
    }

    // Return empty if no match is found
    Prefixes {
        prefix: "".to_string(),
        carrier_name: "".to_string(),
        country_name: "".to_string(),
    }
}

fn extract_dir_and_file_name(path: &Path) -> Result<(PathBuf, String), AppError> {
    // Extract the parent directory (dir_name)
    let dir_name = match path.parent() {
        Some(d) => d.to_path_buf(), // Keep it as PathBuf
        None => {
            Logger::warn("No parent directory found for the given path.");
            return Err(AppError::Unexpected(
                "No parent directory found for the given path.".to_string(),
            ));
        }
    };

    // Extract the file name
    let file_name = match path.file_name() {
        Some(f) => f.to_string_lossy().to_string(), // Convert to string for file name
        None => {
            Logger::warn("No file name found in the given path.");
            return Err(AppError::Unexpected(
                "No file name found in the given path.".to_string(),
            ));
        }
    };

    Ok((dir_name, file_name))
}
