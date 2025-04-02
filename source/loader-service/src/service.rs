use core::config::{AppConfig, ServerConfig};
use core::entities::{Prefixes, RoamOutDB};
use core::errors::AppError;
use core::file::FileManager;
use core::logger::Logger;
use core::store::StoreManager;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

    pub async fn prefix_map(&self) -> Result<HashMap<String, (String, String, String)>, AppError> {
        let prefixes = self.store_manager.select_all_prefixes().await?;

        let prefix_map: HashMap<String, (String, String, String)> = prefixes
            .into_iter()
            .map(|p| (p.prefix, (p.carrier_name, p.country_alpha2, p.country_name)))
            .collect();

        Ok(prefix_map)
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        Logger::info(&format!("{} : start.", SERVICE_NAME));

        let result = self.file_manager.next().await?;

        match result {
            Some((path, source_type)) => {
                let prefix_map = self.prefix_map().await?;

                if source_type == SOURCE_TYPE_ROAM_IN {
                    match self.process_roam_in(path, &prefix_map).await {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    }
                } else if source_type == SOURCE_TYPE_ROAM_OUT {
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

    pub async fn process_roam_in(
        &self,
        path: PathBuf,
        prefix_map: &HashMap<String, (String, String, String)>,
    ) -> Result<(), AppError> {
        println!("roam_in");
        let batch_id = self.start_batch(SOURCE_TYPE_ROAM_IN, "TEST").await?;
        self.update_batch(batch_id, BATCH_STATUS_SUCCESS).await?;
        Ok(())
    }

    pub async fn process_roam_out(
        &self,
        path: PathBuf,
        prefix_map: &HashMap<String, (String, String, String)>,
    ) -> Result<(), AppError> {
        println!("roam_out");

        let (dir_name, file_name) = match extract_dir_and_file_name(&path) {
            Ok((dir, file)) => (dir, file),
            Err(e) => return Err(e),
        };

        let batch_id = self.start_batch(SOURCE_TYPE_ROAM_OUT, &file_name).await?;

        let records = self
            .file_manager
            .read_file_roam_out(dir_name, file_name.to_string())
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
        let batch_date = self.file_manager.extract_and_format_date(&file_name);
        for record in records {
            let prefix = lookup(&prefix_map, record.vlr_number.clone());

            let db_record = RoamOutDB {
                batch_id: batch_id,
                batch_date: batch_date.clone(),
                imsi: record.imsi,
                msisdn: record.msisdn,
                vlr_number: record.vlr_number,
                carrier_name: prefix.carrier_name,
                country_name: prefix.country_name,
                country_alpha2: prefix.country_alpha2,
            };

            db_records.push(db_record);
        }

        self.store_manager.insert_roam_out_stg(db_records).await?;
        self.store_manager.insert_dim_carriers().await?;
        self.store_manager.insert_dim_imsi().await?;
        self.store_manager.insert_dim_msisdn().await?;
        self.store_manager.insert_fct_roam_out().await?;
        self.update_batch(batch_id, BATCH_STATUS_SUCCESS).await?;
        Ok(())
    }

    pub async fn start_batch(&self, source_type: &str, source_name: &str) -> Result<i32, AppError> {
        let result = self
            .store_manager
            .insert_batch_exec(SERVICE_NAME, source_type, source_name)
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

fn extract_dir_and_file_name(path: &Path) -> Result<(PathBuf, String), AppError> {
    let dir_name = match path.parent() {
        Some(d) => d.to_path_buf(),
        None => {
            Logger::warn("No parent directory found for the given path.");
            return Err(AppError::Unexpected(
                "No parent directory found for the given path.".to_string(),
            ));
        }
    };

    let file_name = match path.file_name() {
        Some(f) => f.to_string_lossy().to_string(),
        None => {
            Logger::warn("No file name found in the given path.");
            return Err(AppError::Unexpected(
                "No file name found in the given path.".to_string(),
            ));
        }
    };

    Ok((dir_name, file_name))
}

pub fn lookup(prefix_map: &HashMap<String, (String, String, String)>, mut s: String) -> Prefixes {
    while !s.is_empty() {
        if let Some((carrier_name, country_alpha2, country_name)) = prefix_map.get(&s) {
            return Prefixes {
                prefix: s.clone(),
                carrier_name: carrier_name.clone(),
                country_alpha2: country_alpha2.clone(),
                country_name: country_name.clone(),
            };
        }
        s.pop(); // Remove the last character for prefix matching
    }

    // Return empty if no match is found
    Prefixes {
        prefix: "".to_string(),
        carrier_name: "".to_string(),
        country_alpha2: "".to_string(),
        country_name: "".to_string(),
    }
}
