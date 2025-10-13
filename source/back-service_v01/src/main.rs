mod core;
mod services;

use crate::core::db::Db;
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::{batch_manager::BatchManager, file_loader::RoamOutLoader};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("Starting batch manager demo");

    let pool = Db::create_pool(); // deadpool_postgres::Pool
    let batch_manager = BatchManager::new(pool.clone());
    let loader = RoamOutLoader::new(pool.clone());

    // --- Insert batch ---
    let batch_id = batch_manager
        .insert_batch("LOADER", "OUT", "roam_out.csv", "STARTED")
        .await?;
    println!("INFO Created batch with ID: {}", batch_id);

    // --- Load CSV ---
    let inserted = loader
        .load_csv(
            "../../WORK/INPUT/ROUT/HSS9860_1549_20250912000000.txt",
            batch_id,
            "2025-10-12",
        )
        .await?;
    println!("INFO Inserted {} rows from CSV", inserted);

    // --- Update batch status ---
    batch_manager.update_status(batch_id, "COMPLETED").await?;
    println!("INFO Batch {} marked as COMPLETED", batch_id);

    Ok(())
}
