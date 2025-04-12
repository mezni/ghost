use crate::repo;
use core::db::DBManager;
use core::errors::AppError;

pub async fn test_service(pool: &DBManager) -> Result<i64, AppError> {
    let client = pool.get_client().await?;
    repo::test(&client).await
}
