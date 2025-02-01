use std::sync::Arc;
use datafusion::arrow::{
    array::{Float64Array, StringArray},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::prelude::*;
use datafusion::datasource::memory::MemTable;
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::context::SessionContext;
use datafusion::dataframe::DataFrameWriteOptions;
use tokio;

/// Creates a schema for the DataFrame
fn create_schema() -> Arc<Schema> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("value", DataType::Float64, false),
    ]));

    schema
}

/// Creates a sample RecordBatch
fn create_sample_record_batch(schema: &Arc<Schema>) -> Result<RecordBatch> {
    let names = StringArray::from(vec!["Alice", "Bob", "Charlie"]);
    let values = Float64Array::from(vec![10.0, 20.5, 30.8]);
    let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(names), Arc::new(values)])
        .map_err(|e| DataFusionError::ArrowError(e, None))?;

    Ok(batch)
}

/// Creates a DataFrame from a RecordBatch
async fn create_data_frame(ctx: &SessionContext, batch: RecordBatch) -> Result<DataFrame> {
    ctx.read_batch(batch)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create a DataFusion session context
    let ctx = SessionContext::new();

    // Create a schema
    let schema = create_schema();

    // Create a sample RecordBatch
    let batch = create_sample_record_batch(&schema)?;

    // Create a DataFrame from the RecordBatch
    let df = create_data_frame(&ctx, batch).await?;

    // Show the DataFrame
    df.show().await?;

    Ok(())
}