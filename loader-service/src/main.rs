mod config;
mod errors;
mod file;
mod logger;

use config::read_config;
use errors::AppError;
use file::FileManager;
use logger::Logger;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize the logger
    Logger::init();
    Logger::info("Start Service");

    // Read the configuration file
    let config_file = "config.yaml";
    let config = match read_config(config_file) {
        Ok(config) => config,
        Err(err) => {
            Logger::error(&format!(
                "Failed to read config file '{}': {}",
                config_file, err
            ));
            Logger::error("Stop Service");
            std::process::exit(1); // Exit with an error status code
        }
    };

    match FileManager::new(config) {
        Ok(_) => Logger::info("FileManager initialized successfully."),
        Err(err) => {
            Logger::error(&format!("Failed to initialize FileManager: {}", err));
            Logger::error("Stop Service");
            std::process::exit(1); // Exit with an error status code
        }
    }

    Logger::info("Stop Service");
    Ok(())
}
