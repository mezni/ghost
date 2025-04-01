use core::{shared_function, errors::AppError};
use logger::logger::Logger;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let message = shared_function().await?;
    println!("Service says: {}", message);
    Logger::init();

    // Log messages
    Logger::info("This is an info message.");
    Logger::warn("This is a warning message.");
    Logger::error("This is an error message.");
    Ok(())
}

