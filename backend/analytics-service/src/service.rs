use core::config::ServerConfig;
use core::db::{DBManager, LogRecord};
use core::errors::AppError;
use core::logger::Logger;

pub const SERVICE_NAME: &str = "analytics-srv";

pub struct AnalyticsService {
    db_manager: DBManager,
}

impl AnalyticsService {
    pub async fn new(srv_config: ServerConfig) -> Result<Self, AppError> {
        let db_mgr = match DBManager::new(srv_config) {
            // Remove `.await`
            Ok(sm) => sm,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(AnalyticsService { db_manager: db_mgr })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let mut log_record = LogRecord {
            batch_id: None,
            batch_name: Some(SERVICE_NAME.to_string()),
            source_type: Some("SOURCE_TYPE".to_string()),
            source_name: Some("SOURCE_NAME".to_string()),
            corr_id: None,
            batch_status: None,
        };

        match self.db_manager.insert_batch(&log_record).await {
            Ok(batch_id) => {
                log_record.batch_id = Some(batch_id);
            }
            Err(e) => {
                return Err(e);
            }
        }

        log_record.batch_status = Some("Success".to_string());

        match self.db_manager.update_batch(&log_record).await {
            Ok(()) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }
}
