use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_mgr;
use crate::services::config_mgr::Source;
use crate::services::file_mgr;
use chrono::NaiveDateTime;
use chrono::format::ParseError;
use deadpool_postgres::Pool;
use regex::Regex;
use std::path::Path;
use std::path::PathBuf;
const FILE_TO_PROCESS: usize = 5;
pub async fn load(
    pool: &Pool,
    batch_mgr: &batch_mgr::BatchManager,
    source: &Source,
) -> Result<(), AppError> {
    println!("ROAMOUT");

    match file_mgr::get_files(
        &PathBuf::from(&source.source_directory),
        source.file_pattern.as_deref(),
        FILE_TO_PROCESS,
    ) {
        Ok(files) => {
            for file in files {
                println!("{}", file);
            }
        }
        Err(e) => {}
    }

    Ok(())
}
