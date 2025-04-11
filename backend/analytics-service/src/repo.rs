use core::db::DBManager;
use core::entities::RoamOutDB;
use core::errors::AppError;
use tokio_postgres::Client;

pub async fn test(client: &Client) -> Result<(), AppError> {
    Ok(())
}
