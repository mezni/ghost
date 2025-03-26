mod errors;
mod logger;
mod db;

use errors::AppError;
use db::DBPool;
use logger::Logger;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize the logger
    Logger::init();
    Logger::info("Start Service");

    // Create the DB pool and handle potential error
    let db = DBPool::new()?; 
    
    // Get a client from the DB pool
    let client = db.get_client().await?;

    // Log the stop of the service
    Logger::info("Stop Service");
    
    Ok(())
}
