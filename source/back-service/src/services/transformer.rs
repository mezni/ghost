use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::batch_manager::BatchManager;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, QueryBuilder};

const BATCH_NAME: &str = "TRANSFORMER";
const BATCH_INSERT_SIZE: usize = 500;

pub async fn run(pool: &Pool<Postgres>, batch_mgr: &BatchManager) -> Result<(), AppError> {
    Logger::info("HERE");
    Ok(())
}
