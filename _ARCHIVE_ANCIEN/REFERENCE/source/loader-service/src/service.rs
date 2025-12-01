use crate::repo;
use crate::repo::Prefixes;
use chrono::NaiveDate;
use core::config;
use core::db;
use core::errors::AppError;
use core::file;
use core::logger::Logger;
use std::collections::HashMap;

const CONFIG_FILE: &str = "config.yaml";
const SERVICE_NAME: &str = "loader-srv";
const BATCH_STATUS_SUCCESS: &str = "Success";
const BATCH_STATUS_FAILURE: &str = "Failure";

pub struct LoadService {
    srv_config: config::ServerConfig,
    app_config: config::AppConfig,
    db_manager: db::DBManager,
    file_manager: file::FileManager,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        let srv_config = config::read_srv_config()?;
        srv_config.validate()?;

        let app_config = config::read_app_config(CONFIG_FILE)?;
        app_config.validate()?;

        let db_manager = db::DBManager::new(srv_config.clone())?;
        let file_manager = file::FileManager::new();

        Ok(LoadService {
            srv_config,
            app_config,
            db_manager,
            file_manager,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        while let Some(file) = self.file_manager.next(self.app_config.clone()).await? {
            Logger::info(&format!("Processing {}", file.file_path.display()));
            let db_client = self.db_manager.get_client().await?;
            let prefix_map = self.prefix_map().await?;
            let file_clone = file.clone();
            let batch_id = repo::insert_batch_exec(
                &db_client,
                SERVICE_NAME,
                &file.file_type,
                &file.file_path.to_string_lossy(),
            )
            .await?;
            if let Some(parsed) = self.file_manager.parse_file(file).await? {
                match parsed {
                    file::ParsedData::RoamIn(data) => {
                        // let batch_date = data.metadata.creation_date;
                        let batch_date = data.metadata.creation_date;

                        let mut db_records = Vec::new();
                        for record in data.records {
                            let hlraddr_parts: Vec<&str> = record.hlraddr.split('-').collect();
                            let hlraddr = hlraddr_parts.get(1).unwrap_or(&"").to_string();

                            let prefix = self.lookup(&prefix_map, hlraddr.clone());

                            let db_record = repo::RoamInDataDBRecord {
                                batch_id,
                                batch_date: batch_date.clone(),
                                hlraddr: hlraddr,
                                nsub: record.nsub,
                                nsuba: record.nsuba,
                                prefix: prefix.prefix,
                                country_id: prefix.country_id,
                                operator_id: prefix.operator_id,
                            };

                            db_records.push(db_record);
                        }

                        repo::insert_roam_in_stg_records(&db_client, db_records).await?;
                        repo::update_batch_status(&db_client, batch_id, BATCH_STATUS_SUCCESS)
                            .await?;
                        Logger::info("Processed with success");
                    }

                    file::ParsedData::RoamOut(data) => {
                        println!("enter ROAMOUT");
                        let batch_date = data.metadata.creation_date[..10].to_string();
                        println!("{}",batch_date.clone());
                        let mut db_records = Vec::new();
                        for record in data.records {
                            let prefix = self.lookup(&prefix_map, record.vlr_number.clone());

                            let db_record = repo::RoamOutDataDBRecord {
                                batch_id,
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

                        repo::insert_roam_out_stg_records(&db_client, db_records).await?;
                        repo::update_batch_status(&db_client, batch_id, BATCH_STATUS_SUCCESS)
                            .await?;
                        Logger::info("Processed with success");
                    }
                }
            }
            self.file_manager.remove_file(&file_clone.file_path).await?;
            Logger::info("File removed");
        }
        Ok(())
    }

    pub async fn prefix_map(
        &self,
    ) -> Result<HashMap<String, (Option<i32>, Option<i32>)>, AppError> {
        let db_client = self.db_manager.get_client().await?;

        let prefixes = repo::select_all_prefixes(&db_client).await?;

        let prefix_map: HashMap<String, (Option<i32>, Option<i32>)> = prefixes
            .into_iter()
            .map(|p| (p.prefix, (p.country_id, p.operator_id)))
            .collect();

        Ok(prefix_map)
    }

    pub fn lookup(
        &self,
        prefix_map: &HashMap<String, (Option<i32>, Option<i32>)>,
        mut s: String,
    ) -> Prefixes {
        while !s.is_empty() {
            if let Some((country_id, operator_id)) = prefix_map.get(&s) {
                return Prefixes {
                    prefix: s.clone(),
                    country_id: *country_id,
                    operator_id: *operator_id,
                };
            }
            s.pop();
        }

        Prefixes {
            prefix: "".to_string(),
            country_id: None,
            operator_id: None,
        }
    }
}
