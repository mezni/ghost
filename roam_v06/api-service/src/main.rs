mod errors;
mod infra;

use errors::AppError;
use infra::config::load_config;
use infra::logger::Logger;

fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("Starting service...");

    let config = load_config()?;

    Ok(())
}
