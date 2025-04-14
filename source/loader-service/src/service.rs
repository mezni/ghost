use crate::repo::{
    delete_stg_roam_out_records, insert_fct_roam_out_records, insert_imsi_records,
    insert_msisdn_records, insert_stg_roam_out_records, insert_vlr_records,
};
use core::config::{AppConfig, ServerConfig};
use core::db::{DBManager, LogRecord};
use core::entities::{Prefixes, RoamOutDB, RoamOutDTO};
use core::errors::AppError;
use core::file::FileManager;
use core::logger::Logger;
use core::utils::{lookup, prefix_map};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const SERVICE_NAME: &str = "loader-srv";

const SOURCE_TYPE_ROAM_IN: &str = "ROAM_IN";
const SOURCE_TYPE_ROAM_OUT: &str = "ROAM_OUT";

const BATCH_STATUS_SUCCESS: &str = "Success";
const BATCH_STATUS_FAILURE: &str = "Failure";

pub struct LoadService {
    db_manager: DBManager,
    file_manager: FileManager,
}

impl LoadService {
    pub async fn new(srv_config: ServerConfig, app_config: AppConfig) -> Result<Self, AppError> {
        let db_mgr = match DBManager::new(srv_config) {
            Ok(sm) => sm,
            Err(e) => return Err(e),
        };

        let file_mgr = match FileManager::new(app_config) {
            Ok(fm) => fm,
            Err(e) => return Err(e),
        };

        Ok(LoadService {
            db_manager: db_mgr,
            file_manager: file_mgr,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let result = self.file_manager.next().await?;

        if let Some((path, source_type)) = result {
            Logger::info(&format!(
                "{} : process : {}, {}",
                SERVICE_NAME,
                path.display(),
                source_type
            ));

            // Get the prefix map
            let prefix_map = prefix_map(&self.db_manager).await?;

            // Convert source_type to &str and match against constants
            match source_type.as_str() {
                SOURCE_TYPE_ROAM_IN => self.process_roam_in(path, &prefix_map).await?,
                SOURCE_TYPE_ROAM_OUT => self.process_roam_out(path, &prefix_map).await?,
                _ => {
                    Logger::warn(&format!("Unknown source type: {}", source_type));
                    return Err(AppError::Unexpected(format!(
                        "Unknown source type: {}",
                        source_type
                    )));
                }
            }
        }
        Ok(())
    }

    pub async fn process_roam_in(
        &self,
        path: PathBuf,
        prefix_map: &HashMap<String, (Option<i32>, Option<i32>)>,
    ) -> Result<(), AppError> {
        println!("roam_in");
        let mut log_record = LogRecord {
            batch_id: None,
            batch_name: Some(SERVICE_NAME.to_string()),
            source_type: Some(SOURCE_TYPE_ROAM_IN.to_string()),
            source_name: Some(path.to_string_lossy().to_string()),
            corr_id: None,
            batch_status: None,
        };

        let batch_id = self.db_manager.insert_batch(&log_record).await?;
        log_record.batch_id = Some(batch_id);

        log_record.batch_status = Some(BATCH_STATUS_SUCCESS.to_string());

        self.db_manager.update_batch(&log_record).await?;

        Ok(())
    }

    pub async fn process_roam_out(
        &self,
        path: PathBuf,
        prefix_map: &HashMap<String, (Option<i32>, Option<i32>)>,
    ) -> Result<(), AppError> {
        println!("roam_out");
        let mut log_record = LogRecord {
            batch_id: None,
            batch_name: Some(SERVICE_NAME.to_string()),
            source_type: Some(SOURCE_TYPE_ROAM_OUT.to_string()),
            source_name: Some(path.to_string_lossy().to_string()),
            corr_id: None,
            batch_status: None,
        };

        let batch_id = self.db_manager.insert_batch(&log_record).await?;
        log_record.batch_id = Some(batch_id);

        let (dir_name, file_name) = extract_dir_and_file_name(&path)?;

        let records = self
            .file_manager
            .read_file_roam_out(dir_name, file_name.to_string())
            .await
            .map_err(|e| {
                let err_msg = format!("Failed to process ROAM_OUT for {}: {}", path.display(), e);
                Logger::error(&err_msg);
                AppError::Unexpected(err_msg)
            })?;

        let mut db_records = Vec::new();
        let batch_date = self.file_manager.extract_and_format_date(&file_name);
        for record in records {
            let prefix = lookup(&prefix_map, record.vlr_number.clone());

            let db_record = RoamOutDB {
                batch_id: batch_id.clone(),
                batch_date: batch_date.clone(),
                imsi: record.imsi,
                msisdn: record.msisdn,
                vlr_number: record.vlr_number,
                prefix: prefix.prefix,
                country_id: prefix.country_id,
                operator_id: prefix.operator_id,
            };

            db_records.push(db_record);
        }

        let client = self.db_manager.get_client().await?;
        insert_stg_roam_out_records(&client, db_records).await?;
        // Insert IMSI records
        insert_imsi_records(&client, batch_id.clone()).await?;

        // Insert MSISDN records
        insert_msisdn_records(&client, batch_id.clone()).await?;

        insert_vlr_records(&client, batch_id.clone()).await?;

        insert_fct_roam_out_records(&client, batch_id.clone()).await?;

        delete_stg_roam_out_records(&client, batch_id.clone()).await?;

        log_record.batch_status = Some(BATCH_STATUS_SUCCESS.to_string());

        self.db_manager.update_batch(&log_record).await?;
        //        self.file_manager.archive_file(&path)?;
        self.file_manager.delete_file(&path);
        Ok(())
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
