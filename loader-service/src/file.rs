use crate::config::Config;
use crate::errors::AppError;
use crate::logger::Logger;
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
        let work_base_dir = &config.work_dir;
        let work_base_dir_path = PathBuf::from(work_base_dir);

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
            config: config,
            work_base_dir: work_base_dir_path,
            work_process_dir: work_process_dir_path,
            work_rejected_dir: work_rejected_dir_path,
            work_processed_dir: work_processed_dir_path,
        })
    }

    pub fn execute(&self) -> Result<(), AppError> {
        // You can add logic here to perform some operations, for example:
        Logger::info("Executing file operations...");
        // Example action:
        // - Check the directories for files to process, move them, etc.

        Ok(())
    }
}
