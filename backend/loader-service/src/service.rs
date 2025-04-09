use core::config::{AppConfig, ServerConfig};
use core::db::DBManager;
use core::errors::AppError;
use core::logger::Logger;

const SERVICE_NAME: &str = "loader-srv";

pub struct LoadService {
    db_manager: DBManager,
}

impl LoadService {
    pub async fn new(srv_config: ServerConfig) -> Result<Self, AppError> {
        Logger::info(&format!("{} : init.", SERVICE_NAME));

        let db_mgr = match DBManager::new(srv_config) {
            Ok(sm) => {
                Logger::info("Store - init");
                sm
            }
            Err(e) => {
                Logger::error(&format!("Store - failed: {:?}", e));
                return Err(e);
            }
        };

        Ok(LoadService { db_manager: db_mgr })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        Logger::info(&format!("{} : start.", SERVICE_NAME));
        Ok(())
    }
}
