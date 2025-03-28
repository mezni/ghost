mod config;
mod errors;
mod logger;

use config::{read_app_config, read_srv_config};
use errors::AppError;
use logger::Logger;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize the logger
    Logger::init();
    Logger::info("Start Service");

    // Read the server configuration
    match read_srv_config() {
        Ok(srv_config) => {
            Logger::info("Server Config: loaded successfully.");
        }
        Err(err) => {
            Logger::error(&format!("Server Config: {}", err));
            Logger::info("Stop Service");
            std::process::exit(1);
        }
    }
    let config_file = "config.yaml";
    match read_app_config(config_file) {
        Ok(srv_config) => {
            Logger::info("Server Config: loaded successfully.");
        }
        Err(err) => {
            Logger::error(&format!("Server Config: {}", err));
            Logger::info("Stop Service");
            std::process::exit(1);
        }
    }

    Logger::info("Stop Service");
    Ok(())
}
