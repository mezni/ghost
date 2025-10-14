use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch;
use crate::services::config::{AppConfig, Source};
use crate::services::file;
use crate::services::loader;
use deadpool_postgres::Pool;
use regex::Regex;
use std::path::PathBuf;

pub async fn run(pool: Pool, config: AppConfig) -> Result<(), AppError> {
    Logger::info("Run");
    let batch_mgr = batch::BatchManager::new(pool.clone());

    for source in &config.sources {
        println!("{:?}", source);
        if file::dir_exists(&source.source_directory) {
            loader::load(&pool, &batch_mgr, source).await?;
        } else {
            Logger::warn("Directory does not exist");
        }
    }

    Ok(())
}
