use crate::repo::{
    get_next_batch_id, insert_anomalie_imsi, insert_anomalie_msisdn, insert_anomalie_sor_deviation,
    insert_fct_sor_out_records,
};
use core::config::ServerConfig;
use core::db::{DBManager, LogRecord};
use core::errors::AppError;
use core::logger::Logger;

pub const SERVICE_NAME: &str = "analytics-srv";
const BATCH_STATUS_SUCCESS: &str = "Success";
const BATCH_STATUS_FAILURE: &str = "Failure";

pub struct AnalyticsService {
    db_manager: DBManager,
}

impl AnalyticsService {
    pub async fn new(srv_config: ServerConfig) -> Result<Self, AppError> {
        let db_mgr = match DBManager::new(srv_config) {
            Ok(sm) => sm,
            Err(e) => return Err(e),
        };

        Ok(AnalyticsService { db_manager: db_mgr })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let client = self.db_manager.get_client().await?;

        while let Some(corr_id) = get_next_batch_id(&client).await? {
            let mut log_record = LogRecord {
                batch_id: None,
                batch_name: Some(SERVICE_NAME.to_string()),
                source_type: None,
                source_name: None,
                corr_id: Some(corr_id),
                batch_status: None,
            };

            let batch_id = self.db_manager.insert_batch(&log_record).await?;
            log_record.batch_id = Some(batch_id);

            let result = async {
                insert_fct_sor_out_records(&client, corr_id).await?;
                insert_anomalie_imsi(&client, corr_id).await?;
                insert_anomalie_msisdn(&client, corr_id).await?;
                insert_anomalie_sor_deviation(&client, corr_id).await?;
                Ok::<(), AppError>(())
            }
            .await;

            match result {
                Ok(_) => {
                    log_record.batch_status = Some(BATCH_STATUS_SUCCESS.to_string());
                }
                Err(e) => {
                    log_record.batch_status = Some(BATCH_STATUS_FAILURE.to_string());
                }
            }

            self.db_manager.update_batch(&log_record).await?;
        }

        Ok(())
    }
}
