use core::config::{AppConfig, ServerConfig};
use core::db::{DBManager, LogRecord};
use core::errors::AppError;
use core::file::FileManager;
use core::logger::Logger;
use std::collections::HashMap;

pub const SERVICE_NAME: &str = "loader-srv";

pub struct LoadService {
    db_manager: DBManager,
    file_manager: FileManager,
}

impl LoadService {
    pub async fn new(srv_config: ServerConfig, app_config: AppConfig) -> Result<Self, AppError> {
        let db_mgr = match DBManager::new(srv_config) {
            // Remove `.await`
            Ok(sm) => sm,
            Err(e) => {
                return Err(e);
            }
        };

        let file_mgr = match FileManager::new(app_config) {
            Ok(fm) => fm,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(LoadService {
            db_manager: db_mgr,
            file_manager: file_mgr,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let result = self.file_manager.next().await?;

        match result {
            Some((path, source_type)) => {
                Logger::info(&format!(
                    "{} : process : {}, {}",
                    SERVICE_NAME,
                    path.display(),
                    source_type
                ));
                let prefix_map = self.prefix_map().await?;
                println!("{:#?}", prefix_map);
            }
            None => {}
        }

        Ok(())
    }

    pub async fn prefix_map(
        &self,
    ) -> Result<HashMap<Option<String>, (Option<i32>, Option<i32>)>, AppError> {
        let prefixes = self.db_manager.select_all_prefixes().await?;

        let prefix_map: HashMap<Option<String>, (Option<i32>, Option<i32>)> = prefixes
            .into_iter()
            .map(|p| (p.prefix, (p.country_id, p.operator_id)))
            .collect();

        Ok(prefix_map)
    }
}
