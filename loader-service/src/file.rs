use crate::config::Config;
use crate::errors::AppError;
use crate::logger::Logger;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

const PROCESS_DIR_NAME: &str = "PROCESS";
const REJECTED_DIR_NAME: &str = "REJECTED";
const PROCESSED_DIR_NAME: &str = "PROCESSED";

pub struct FileManager {
    config: Config,
    work_base_dir: PathBuf,
    work_process_dir: PathBuf,
    work_rejected_dir: PathBuf,
    work_processed_dir: PathBuf,
}

impl FileManager {
    pub fn new(config: Config) -> Result<Self, AppError> {
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

        // Ensure all directories exist
        for dir in [
            &work_process_dir_path,
            &work_rejected_dir_path,
            &work_processed_dir_path,
        ] {
            fs::create_dir_all(dir).map_err(AppError::IoError)?;
        }

        Ok(FileManager {
            config,
            work_base_dir: work_base_dir_path,
            work_process_dir: work_process_dir_path,
            work_rejected_dir: work_rejected_dir_path,
            work_processed_dir: work_processed_dir_path,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        for source in &self.config.sources {
            let files_vec: Vec<_> = match fs::read_dir(&source.source_directory) {
                Ok(files) => files.flatten().collect(),
                Err(err) => {
                    Logger::error(&format!(
                        "Failed to read directory '{}': {}",
                        source.source_directory, err
                    ));
                    continue;
                }
            };

            let file_pattern_regex = Regex::new(&source.file_pattern)
                .map_err(|err| AppError::Unexpected(format!("Invalid file pattern: {}", err)))?;

            if let Some(file) = files_vec.iter().find(|file| {
                let file_name_str = file.file_name().to_string_lossy().to_string(); // Convert to owned String
                file_pattern_regex.is_match(&file_name_str)
            }) {
                Logger::info(&format!("Processing file: {}", file.path().display()));
                if source.source_type == "ROAM_IN" {
                    self.process_roam_in_file(&file.path()).await?;
                } else if source.source_type == "ROAM_OUT" {
                    self.process_roam_out_file(&file.path()).await?;
                } else {
                    Logger::error(&format!("Unknown source type: {}", source.source_type));
                }
            }
        }
        Ok(())
    }

    pub async fn process_roam_in_file(&self, file_path: &PathBuf) -> Result<(), AppError> {
        println!("Processing ROAM_IN file: {}", file_path.display());
        Ok(())
    }

    pub async fn process_roam_out_file(&self, file_path: &PathBuf) -> Result<(), AppError> {
        println!("Processing ROAM_OUT file: {}", file_path.display());
        Ok(())
    }
}
