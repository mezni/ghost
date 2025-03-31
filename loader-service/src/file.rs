use crate::config::AppConfig;
use crate::entities::RoamOutDAO;
use crate::errors::AppError;
use crate::logger::Logger;
use regex::Regex;
use std::path::{Path, PathBuf};

use csv::{ReaderBuilder, Trim};
use std::fs;
use std::{
    fs::File,
    io::{self, BufReader},
};

const PROCESS_DIR_NAME: &str = "PROCESS";
const REJECTED_DIR_NAME: &str = "REJECTED";
const PROCESSED_DIR_NAME: &str = "PROCESSED";

pub struct FileManager {
    config: AppConfig,
    work_base_dir: PathBuf,
    work_process_dir: PathBuf,
    work_rejected_dir: PathBuf,
    work_processed_dir: PathBuf,
}

impl FileManager {
    pub fn new(config: AppConfig) -> Result<Self, AppError> {
        let work_base_dir_path = PathBuf::from(&config.work_dir);

        if !work_base_dir_path.exists() || !work_base_dir_path.is_dir() {
            return Err(AppError::Unexpected(format!(
                "Directory does not exist: {}",
                work_base_dir_path.display()
            )));
        }

        let work_process_dir_path = work_base_dir_path.join(PROCESS_DIR_NAME);
        let work_rejected_dir_path = work_base_dir_path.join(REJECTED_DIR_NAME);
        let work_processed_dir_path = work_base_dir_path.join(PROCESSED_DIR_NAME);

        for dir in [
            &work_process_dir_path,
            &work_rejected_dir_path,
            &work_processed_dir_path,
        ] {
            fs::create_dir_all(dir).map_err(AppError::IoError)?;
        }

        Ok(FileManager {
            config: config,
            work_base_dir: work_base_dir_path,
            work_process_dir: work_process_dir_path,
            work_rejected_dir: work_rejected_dir_path,
            work_processed_dir: work_processed_dir_path,
        })
    }

    pub async fn next(&self) -> Result<Option<(PathBuf, String)>, AppError> {
        for source in &self.config.sources {
            let files_vec: Vec<_> = match fs::read_dir(&source.source_directory) {
                Ok(files) => files.flatten().collect(),
                Err(err) => {
                    continue;
                }
            };
            let file_pattern_regex = Regex::new(&source.file_pattern)
                .map_err(|err| AppError::Unexpected(format!("Invalid file pattern: {}", err)))?;

            if let Some(file) = files_vec.iter().find(|file| {
                let file_name_str = file.file_name().to_string_lossy().to_string(); // Convert to owned String
                file_pattern_regex.is_match(&file_name_str)
            }) {
                if source.source_type.to_uppercase() == "ROAM_IN" {
                    return Ok(Some((file.path(), "ROAM_IN".to_string())));
                } else if source.source_type.to_uppercase() == "ROAM_OUT" {
                    return Ok(Some((file.path(), "ROAM_OUT".to_string())));
                }
            }
        }

        Ok(None)
    }

    pub async fn process_roam_in(
        &self,
        base_dir: std::path::PathBuf,
        file_name: String,
    ) -> Result<(), AppError> {
        println!("process_roam_in");
        Ok(())
    }

    pub async fn process_roam_out(
        &self,
        base_dir: std::path::PathBuf,
        file_name: String,
    ) -> Result<Vec<RoamOutDAO>, AppError> {
        println!("process_roam_out");
        let full_path = base_dir.join(file_name);

        let file = File::open(full_path)?;
        let mut reader = ReaderBuilder::new()
            .trim(Trim::All)
            .from_reader(BufReader::new(file));

        let mut records = Vec::new();
        for result in reader.records() {
            let record = result?;
            let imsi = record[0].to_string();
            let msisdn = record[1].to_string();
            let vlr_number = record[2].to_string();
            records.push(RoamOutDAO {
                imsi: imsi,
                msisdn: msisdn,
                vlr_number: vlr_number,
            });
        }

        Ok(records)
    }
}
