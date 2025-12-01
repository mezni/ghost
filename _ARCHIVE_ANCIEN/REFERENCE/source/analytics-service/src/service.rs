use crate::repo;
use core::config;
use core::db;
use core::errors::AppError;
use core::logger::Logger;

const SERVICE_NAME: &str = "analytics-srv";
const BATCH_STATUS_SUCCESS: &str = "Success";
const BATCH_STATUS_FAILURE: &str = "Failure";

pub struct AnalyticsService {
    srv_config: config::ServerConfig,
    db_manager: db::DBManager,
}

impl AnalyticsService {
    pub async fn new() -> Result<Self, AppError> {
        let srv_config = config::read_srv_config()?;
        srv_config.validate()?;

        let db_manager = db::DBManager::new(srv_config.clone())?;

        Ok(AnalyticsService {
            srv_config,
            db_manager,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let db_client = self.db_manager.get_client().await?;

        while let Some((corr_id, source_type)) = repo::get_next_batch_id(&db_client).await? {
            // ROAM_OUT ROAM_IN
            if source_type == "ROAM_IN" {
                let batch_id =
                    repo::insert_batch_exec(&db_client, SERVICE_NAME, &source_type, corr_id)
                        .await?;
                repo::insert_roam_in_metrics(&db_client, corr_id).await?;
                repo::cleanup_roam_in_stg(&db_client, corr_id).await?;
                repo::update_batch_status(&db_client, batch_id, BATCH_STATUS_SUCCESS).await?;
            } else if source_type == "ROAM_OUT" {
                let batch_id =
                    repo::insert_batch_exec(&db_client, SERVICE_NAME, &source_type, corr_id)
                        .await?;
                repo::insert_roam_out_metrics(&db_client, corr_id).await?;
                repo::insert_roam_out_perfs(&db_client, corr_id).await?;
                repo::cleanup_roam_out_stg(&db_client, corr_id).await?;
                repo::update_batch_status(&db_client, batch_id, BATCH_STATUS_SUCCESS).await?;
            } else {
                continue;
            }
        }

        Ok(())
    }
}
