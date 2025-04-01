use core::config::{AppConfig, ServerConfig};
use core::errors::AppError;
use core::logger::Logger;
use core::store::StoreManager;
use core::file::FileManager;

const SERVICE_NAME: &str = "loader-srv";

pub struct LoadService {
    store_manager: StoreManager,
    file_manager: FileManager,    
}

impl LoadService {
    pub async fn new(srv_config: ServerConfig, app_config: AppConfig) -> Result<Self, AppError> {
        Logger::info(&format!("{} : init.", SERVICE_NAME));
        let store_mgr = match StoreManager::new(srv_config) {
            Ok(sm) => {
                Logger::info("Store - init");
                sm
            }
            Err(e) => {
                Logger::error(&format!("Store - failed: {:?}", e));
                return Err(e);
            }
        };

        let file_mgr = match FileManager::new(app_config) {
            Ok(fm) => {
                Logger::info("File - init");
                fm
            }
            Err(e) => {
                Logger::error(&format!("File - failed: {:?}", e));
                return Err(e);
            }
        };        

        Ok(LoadService {
            store_manager: store_mgr,
            file_manager: file_mgr,    
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        Logger::info(&format!("{} : start.", SERVICE_NAME));

        let result = self
            .store_manager
            .insert_batch_exec(SERVICE_NAME, "ROAM_OUT", "TEST")
            .await;

        match result {
            Ok(batch_id) => Logger::info(&format!("Batch started: ID={}", batch_id)),
            Err(e) => Logger::error(&format!("Batch failed: {:?}", e)),
        }

        Ok(())
    }
}
