use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::services::config_reader::Source;
use deadpool_postgres::Pool;

pub async fn process(_pool: Pool, source: Source) -> Result<(), AppError> {
    println!("🟢 File Loader called for directory: {:?}", source);
    println!("OK");
    Ok(())
}
