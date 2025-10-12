mod core;

use core::errors::AppError;
use core::logger::Logger;
use std::fs;
use std::path::Path;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();    
    Logger::info("Starting directory listing process");
    let pool = core::db::Db::create_pool();

    Logger::info(&format!("Reading directory: {}", dir_path));
    
    let file_loader = services::loader::FileLoader::new(db);

    let dir_path = "../../WORK/INPUT/RIN/";
    
    let summary = file_loader.process_directory(dir_path).await?;

    Logger::info("Process completed successfully");
    Ok(())
}