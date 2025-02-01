use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use csv_async::{AsyncReaderBuilder, Trim};
use datafusion::{
    arrow::{array::{StringArray, ArrayRef}, datatypes::{DataType, Field, Schema}, record_batch::RecordBatch},
    dataframe::DataFrameWriteOptions,
    execution::context::SessionContext,
    prelude::*,
};
use futures::stream::StreamExt;
use log::{error, info};
use tokio::fs::File;
use tokio::io::BufReader;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

    fn hash_filename(filename: &str) -> u32 {
        let mut hasher = DefaultHasher::new();
        filename.hash(&mut hasher);
        hasher.finish() as u32
    }


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let file_name = "cdr.csv";
    let file = File::open(file_name).await?;
    let reader = BufReader::new(file);
    let file_id = hash_filename(file_name);
    let mut csv_reader = AsyncReaderBuilder::new()
        .has_headers(true)
        .trim(Trim::All)
        .create_deserializer(reader);

    let mut records_stream = csv_reader.deserialize::<HashMap<String, String>>();

    let chunk_size = 10;
    let mut chunk = Vec::with_capacity(chunk_size);

    while let Some(record) = records_stream.next().await {
        match record {
            Ok(mut record) => {
                // Generate a UUID and add it to the record
                record.insert("_uuid".to_string(), Uuid::new_v4().to_string());
                record.insert("_file_id".to_string(), file_id.to_string());
                chunk.push(record);
            }
            Err(err) => error!("Error deserializing record: {}", err),
        }

        if chunk.len() == chunk_size {
            info!("Processing a chunk of {} records", chunk.len());
            process_chunk(&chunk).await?;
            chunk.clear();
        }
    }

    if !chunk.is_empty() {
        info!("Processing remaining records");
        process_chunk(&chunk).await?;
    }

    Ok(())
}

async fn process_chunk(chunk: &[HashMap<String, String>]) -> Result<(), Box<dyn Error>> {
    let mut columns: HashMap<String, Vec<String>> = HashMap::new();

    for record in chunk {
        for (key, value) in record {
            columns.entry(key.clone()).or_default().push(value.clone());
        }
    }

    let mut string_arrays: Vec<ArrayRef> = Vec::new();
    let mut fields = Vec::new();

    for (column_name, values) in &columns {
        fields.push(Field::new(column_name, DataType::Utf8, false));
        string_arrays.push(Arc::new(StringArray::from(values.clone())) as ArrayRef);
    }

    let schema = Arc::new(Schema::new(fields));
    let batch = RecordBatch::try_new(schema.clone(), string_arrays)?;

    info!("Schema:\n{:?}", schema);

    let table = datafusion::datasource::memory::MemTable::try_new(schema, vec![vec![batch]])?;

    let ctx = SessionContext::new();
    ctx.register_table("my_table", Arc::new(table))?;

    let df = ctx.sql("SELECT * FROM my_table").await?;
    df.clone().show().await?;

    let target_path = "data.parquet";
    df.write_parquet(
        target_path,
        DataFrameWriteOptions::new(),
        None,
    ).await?;

    Ok(())
}
