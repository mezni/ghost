mod config;
mod errors;
mod file;
mod logger;
mod service;
mod store;

use config::{read_app_config, read_srv_config};
use errors::AppError;
use logger::Logger;
use service::LoadService;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize the logger
    Logger::init();
    Logger::info("Start Service");

    let srv_config = match read_srv_config() {
        Ok(srv_config) => {
            Logger::info("Server Config: loaded successfully.");
            srv_config
        }
        Err(err) => {
            Logger::error(&format!("Server Config: {}", err));
            Logger::info("Stop Service");
            std::process::exit(1);
        }
    };

    let config_file = "config.yaml";
    let app_config = match read_app_config(config_file) {
        Ok(app_config) => {
            Logger::info("App Config: loaded successfully.");
            app_config
        }
        Err(err) => {
            Logger::error(&format!("App Config: {}", err));
            Logger::info("Stop Service");
            std::process::exit(1);
        }
    };

    let service = LoadService::new(srv_config, app_config).await?;

    Logger::info("Stop Service");
    Ok(())
}
