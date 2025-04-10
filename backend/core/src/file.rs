use crate::config::AppConfig;
use crate::errors::AppError;
use crate::logger::Logger;
use chrono::Local;
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
                let file_name_str = file.file_name().to_string_lossy().to_string();
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
}
