use regex::Regex;
use std::fs;

use crate::config::Config;
use crate::errors::AppError;
use crate::logger::Logger;
use crate::store::{DBPool, insert_batch_exec, update_batch_execs};

pub struct LoadService {
    pool: DBPool,
    config: Config,
}

impl LoadService {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        let pool = DBPool::new()?;
        Logger::info("Database connection pool initialized.");
        Ok(LoadService { pool, config })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        for source in &self.config.sources {
            let mut files_vec = Vec::new();

            match fs::read_dir(&source.source_directory) {
                Ok(files) => {
                    files_vec.extend(files.flatten());
                }
                Err(err) => {
                    Logger::error(&format!(
                        "Failed to read directory '{}': {}",
                        source.source_directory, err
                    ));
                    continue;
                }
            }

            let file_pattern_regex = Regex::new(&source.file_pattern)
                .map_err(|err| AppError::Unexpected(format!("Invalid file pattern: {}", err)))?;

            // Find the first matching file
            if let Some(file) = files_vec.iter().find(|file| {
                let file_name = file.file_name();
                let file_name_str = file_name.to_string_lossy();
                file_pattern_regex.is_match(&file_name_str)
            }) {
                Logger::info(&format!("Processing file: {}", file.path().display()));
            }
        }
        Ok(())
    }

    pub async fn execute2(&self) -> Result<(), AppError> {
        let path_name = "TEST";
        let status = "TEST";

        let batch_id = self.start_batch(path_name).await?;
        self.update_batch_status(batch_id, status).await?;
        Ok(())
    }

    async fn start_batch(&self, path_name: &str) -> Result<i32, AppError> {
        Logger::info(&format!("Starting batch with name: {}", path_name));
        match insert_batch_exec(&self.pool, path_name).await {
            Ok(id) => {
                Logger::info(&format!("Batch started with ID: {}", id));
                Ok(id)
            }
            Err(e) => {
                Logger::error(&format!("Failed to start batch: {}", e));
                Err(e)
            }
        }
    }

    async fn update_batch_status(&self, batch_id: i32, status: &str) -> Result<(), AppError> {
        Logger::info(&format!(
            "Ending batch {} with status: {}",
            batch_id, status
        ));
        match update_batch_execs(&self.pool, batch_id, status).await {
            Ok(_) => {
                Logger::info(&format!(
                    "Batch {} updated with status: {}",
                    batch_id, status
                ));
                Ok(())
            }
            Err(e) => {
                Logger::error(&format!("Failed to update batch {}: {}", batch_id, e));
                Err(e)
            }
        }
    }
}
