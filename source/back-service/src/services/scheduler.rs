use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch;
use crate::services::config::{AppConfig, Source};
use crate::services::file;
use crate::services::loader;
use crate::services::transformer;
use deadpool_postgres::Pool;
use regex::Regex;
use std::path::PathBuf;
use tokio::time::{Duration, interval};

const TIME_OUT_SECONDS: u64 = 5;

pub async fn run(pool: Pool, config: AppConfig) -> Result<(), AppError> {
    Logger::info("Run");
    let batch_mgr = batch::BatchManager::new(pool.clone());
    let mut interval = interval(Duration::from_secs(TIME_OUT_SECONDS));

    loop {
        interval.tick().await;

        for source in &config.sources {
            println!("{:?}", source);
            if file::dir_exists(&source.source_directory) {
                loader::load(&pool, &batch_mgr, source).await?;
            } else {
                Logger::warn("Directory does not exist");
            }
        }
        transformer::transform(&pool, &batch_mgr).await?;
    }
}
