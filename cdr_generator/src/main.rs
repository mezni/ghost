use datafusion::prelude::*;
use arrow::util::pretty;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a session context
    let ctx = SessionContext::new(); // Removed `mut`

    // Read CSV file
    let df = ctx.read_csv("input.csv", CsvReadOptions::new()).await?;

    // Print schema
    println!("Schema:");
    println!("{:#?}", df.schema());

    // Collect data for printing
    let batches = df.clone().collect().await?; // Clone `df` to retain ownership for later use
    println!("Data:");
    pretty::print_batches(&batches)?;

    // Write to Parquet file
    df.write_parquet("output.parquet", None).await?;

    Ok(())
}
