use std::collections::HashMap;
use std::error::Error;

use std::sync::Arc;
//use async_trait::async_trait;
use csv_async::{AsyncReaderBuilder, Trim};
use datafusion::{
    arrow::{array::StringArray, datatypes::DataType, record_batch::RecordBatch},
    dataframe::DataFrame,
    execution::context::SessionContext,
    prelude::*,
};
use datafusion::dataframe::DataFrameWriteOptions;
use futures::stream::StreamExt;
use log::{error, info};
use tokio::fs::File;
use tokio::io::BufReader;

struct CDRProcessor {
    file_path: String,
    chunk_size: usize,
    schema: Option<Arc<datafusion::arrow::datatypes::Schema>>,
}

impl CDRProcessor {
    async fn new(file_path: String) -> Self {
        CDRProcessor {
            file_path,
            chunk_size: 10,
            schema: None,
        }
    }

    async fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.file_path).await?;
        let reader = BufReader::new(file);

        let mut csv_reader = AsyncReaderBuilder::new()
            .has_headers(true)
            .trim(Trim::All)
            .create_deserializer(reader);

        let mut records_stream = csv_reader.deserialize::<HashMap<String, String>>();

        let mut chunk = Vec::with_capacity(self.chunk_size);

        while let Some(record) = records_stream.next().await {
            match record {
                Ok(record) => chunk.push(record),
                Err(err) => error!("Error deserializing record: {}", err),
            }

            if chunk.len() == self.chunk_size {
                info!("Processing a chunk of {} records", chunk.len());
                self.process_chunk(&chunk).await?;
                chunk.clear();
            }
        }

        if !chunk.is_empty() {
            info!("Processing remaining records");
            self.process_chunk(&chunk).await?;
        }

        Ok(())
    }

    async fn process_chunk(&mut self, chunk: &[HashMap<String, String>]) -> Result<(), Box<dyn Error>> {
        if self.schema.is_none() {
            self.schema = Some(self.read_schema(chunk)?);
        }

        let batch = self.create_batch(chunk)?;
        let table = datafusion::datasource::memory::MemTable::try_new(self.schema.clone().unwrap(), vec![vec![batch]])?;

        let ctx = SessionContext::new();
        ctx.register_table("my_table", Arc::new(table))?;

        let df = ctx.sql("SELECT * FROM my_table").await?;
        df.clone().show().await?;

        let target_path = "data.parquet";
        df.write_parquet(
            target_path,
            DataFrameWriteOptions::new(),
            None, 
        ).await;

        Ok(())
    }

    fn read_schema(&self, chunk: &[HashMap<String, String>]) -> Result<Arc<datafusion::arrow::datatypes::Schema>, Box<dyn Error>> {
        let mut columns: HashMap<String, Vec<String>> = HashMap::new();

        for record in chunk {
            for (key, value) in record {
                columns.entry(key.clone()).or_default().push(value.clone());
            }
        }

        let mut fields = Vec::new();

        for (column_name, _) in &columns {
            fields.push(datafusion::arrow::datatypes::Field::new(
                column_name,
                DataType::Utf8,
                false,
            ));
        }

        let schema = Arc::new(datafusion::arrow::datatypes::Schema::new(fields));
        Ok(schema)
    }

    fn create_batch(&self, chunk: &[HashMap<String, String>]) -> Result<RecordBatch, Box<dyn Error>> {
        let mut string_arrays: Vec<datafusion::arrow::array::ArrayRef> = Vec::new();

        for (column_name, _) in self.schema.clone().unwrap().fields().iter().enumerate() {
            let mut values = Vec::new();

            for record in chunk {
                if let Some(value) = record.get(self.schema.clone().unwrap().fields()[column_name].name()) {
                    values.push(value.clone());
                } else {
                    values.push("".to_string());
                }
            }

            string_arrays.push(Arc::new(StringArray::from(values)) as datafusion::arrow::array::ArrayRef);
        }

        let batch = RecordBatch::try_new(self.schema.clone().unwrap(), string_arrays)?;
        Ok(batch)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut processor = CDRProcessor::new("cdr.csv".to_string()).await;
    processor.process().await?;

    Ok(())
}