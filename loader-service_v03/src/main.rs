mod config;
mod errors;
mod file;
mod logger;
mod service;

use config::read_config;
use errors::AppError;
use file::FileManager;
use logger::Logger;
use service::LoadService;

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
    let service = match LoadService::new(config).await {
        Ok(srv) => {
            Logger::info("LoadService initialized successfully.");
            srv
        }
        Err(err) => {
            Logger::error(&format!("Failed to initialize Service: {}", err));
            Logger::error("Stop Service");
            std::process::exit(1); // Exit with an error status code
        }
    };

    /*
        // Initialize FileManager
        let file_manager = match FileManager::new(config) {
            Ok(manager) => {
                Logger::info("FileManager initialized successfully.");
                manager
            }
            Err(err) => {
                Logger::error(&format!("Failed to initialize FileManager: {}", err));
                Logger::error("Stop Service");
                std::process::exit(1); // Exit with an error status code
            }
        };


        // Execute file operations
        if let Err(err) = file_manager.execute().await {
            Logger::error(&format!("Error during execution: {}", err));
            std::process::exit(1);
        }
    */
    Logger::info("Stop Service");
    Ok(())
}
