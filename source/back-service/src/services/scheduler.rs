use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_mgr;
use crate::services::config_mgr::AppConfig;
use crate::services::file_mgr;
use crate::services::roamin_loader;
use crate::services::roamout_loader;
use deadpool_postgres::Pool;
use regex::Regex;
use std::path::PathBuf;
use tokio::time::{Duration, interval};

const TIME_OUT_SECONDS: u64 = 5;

const ROAM_IN: &str = "ROAM_IN";
const ROAM_OUT: &str = "ROAM_OUT";

pub async fn execute(pool: &Pool, config: &AppConfig) -> Result<(), AppError> {
    let batch_mgr = batch_mgr::BatchManager::new(pool.clone());
    for source in &config.sources {
        Logger::debug(&format!("Processing source: {:?}", source));
        match source.source_type.as_str() {
            ROAM_IN => match PathBuf::from(&source.source_directory) {
                directory => {
                    if file_mgr::check_directory(&directory) {
                        roamin_loader::load(&pool, &batch_mgr, source).await?;
                    }
                }
            },
            ROAM_OUT => match PathBuf::from(&source.source_directory) {
                directory => {
                    if file_mgr::check_directory(&directory) {
                        roamout_loader::load(&pool, &batch_mgr, source).await?;
                    }
                }
            },
            _ => {}
        }
    }
    Ok(())
}

pub async fn run(pool: Pool, config: AppConfig) -> Result<(), AppError> {
    let mut interval = interval(Duration::from_secs(TIME_OUT_SECONDS));
    loop {
        if let Err(e) = execute(&pool, &config).await {
            Logger::error(&format!("Error executing task: {}", e));
        }
        interval.tick().await;
    }
    Ok(())
}
