use core::errors::AppError;
use tokio_postgres::Client;

/// Simple query to count rows from the `fct_roam_out` table.
const SELECT_TEST_QUERY: &str = "
    SELECT count(*) FROM fct_roam_out;
";

/// Executes the query and returns the row count.
pub async fn test(client: &Client) -> Result<i64, AppError> {
    let row = client
        .query_one(SELECT_TEST_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let count: i64 = row.get(0);
    println!("{}", count.clone());
    Ok(count)
}
