use core::config::{AppConfig, ServerConfig};
use core::db::DBManager;
use core::errors::AppError;
use core::logger::Logger;

const SERVICE_NAME: &str = "analytics-srv";

pub struct AnalyticsService {
    db_manager: DBManager,
}

impl AnalyticsService {
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

        Ok(AnalyticsService { db_manager: db_mgr })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        Logger::info(&format!("{} : start.", SERVICE_NAME));
        Ok(())
    }
}
