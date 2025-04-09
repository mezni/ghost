use core::errors::AppError;
use core::logger::Logger;

use std::process;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info("Start.");
    Logger::info("Stop.");
    Ok(())
}
