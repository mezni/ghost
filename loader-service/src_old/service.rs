use crate::config::Config;
use crate::errors::AppError;
use crate::file::FileManager;
use crate::logger::Logger;

pub struct LoadService {
    config: Config,
    file_manager: FileManager,
}

impl LoadService {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        let file_manager = match FileManager::new(config.clone()) {
            Ok(manager) => {
                Logger::info("FileManager initialized successfully.");
                manager
            }
            Err(err) => {
                Logger::error(&format!("Failed to initialize FileManager: {}", err));
                Logger::error("Stop Service");
                std::process::exit(1);
            }
        };

        Ok(LoadService {
            config,
            file_manager,
        })
    }

    pub async fn new(config: Config) -> Result<Self, AppError> {
        Logger::error("Inside");
        Ok(())
    }
}
