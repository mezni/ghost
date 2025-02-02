use datafusion::prelude::*;
use datafusion::execution::context::SessionContext;
use uuid::Uuid;
use datafusion::arrow::{
    array::{StringArray, ArrayRef},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use std::sync::Arc;

// Function to generate UUIDs as an Arrow Array
fn generate_uuids(row_count: usize) -> ArrayRef {
    let uuids_vec: Vec<String> = (0..row_count).map(|_| Uuid::new_v4().to_string()).collect();
    Arc::new(StringArray::from(uuids_vec))
}

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let file_name = "cdr.csv";

    // Create a new DataFusion session context
    let ctx = SessionContext::new();

    // Read CSV into DataFrame
    let df = ctx.read_csv(file_name, CsvReadOptions::new()).await?;

    // Show original DataFrame
    df.clone().show().await?;

    // Get DataFrame Schema
    let schema = df.schema();
    let column_names: Vec<String> = schema.fields().iter().map(|f| f.name().clone()).collect();
    println!("Columns: {:?}", column_names);

    // Select specific columns
    let df = df.select_columns(&[
        "call_id", "timestamp", "caller", "receiver", 
        "duration", "call_type", "status", "cost"
    ])?;

    // Show modified DataFrame
    df.clone().show().await?;

    // Get row count
    let row_count = df.clone().count().await?;
    println!("Number of rows: {}", row_count);

    // Generate UUIDs
    let uuids = generate_uuids(row_count);

    // Collect DataFrame into RecordBatches
    let batches = df.collect().await?;
    let mut new_batches = Vec::new();

    for batch in batches {
        let mut columns = batch.columns().to_vec();
        columns.push(uuids.clone()); // Append the UUID column

        let mut fields = batch.schema().fields().clone().to_vec(); // Convert to Vec<Field>
        fields.push(Field::new("uuid", DataType::Utf8, false).into());

        let new_schema = Arc::new(Schema::new(fields));
        let new_batch = RecordBatch::try_new(new_schema.clone(), columns)?;
        new_batches.push(new_batch);
    }

    // Convert RecordBatches back to a DataFrame using read_batches()
    let df = ctx.read_batches(new_batches)?;

    // Show final DataFrame
    df.show().await?;

    Ok(())
}
