use crate::AppError;
use core::config::{AppConfig, ServerConfig};
use core::logger::Logger;

const SERVICE_NAME: &str = "Loader-srv";

pub struct LoadService {
    // Add fields if necessary
}

impl LoadService {
    pub async fn new(srv_config: ServerConfig, app_config: AppConfig) -> Result<Self, AppError> {
        Logger::info(&format!("{} : init.", SERVICE_NAME));

        Ok(LoadService {})
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        Logger::info(&format!("{} : start.", SERVICE_NAME));
        Ok(())
    }
}
