use csv_async::{AsyncReaderBuilder, Trim};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("cdr.csv").await?;
    let reader = BufReader::new(file);

    let mut csv_reader = AsyncReaderBuilder::new()
        .has_headers(true)
        .trim(Trim::All)
        .create_deserializer(reader);

    let mut records_stream = csv_reader.deserialize::<HashMap<String, String>>();

    // Process records in chunks
    let chunk_size = 10; // Number of records per chunk
    let mut chunk = Vec::with_capacity(chunk_size);

    while let Some(record) = records_stream.next().await {
        let record = record?;

        // Add the record to the chunk
        chunk.push(record);

        // If the chunk is full, process it
        if chunk.len() == chunk_size {
            println!("Processing a chunk of {} records", chunk.len());
            for r in &chunk {
                println!("Dynamic Record: {:?}", r);
            }
            chunk.clear(); // Clear the chunk after processing
        }
    }

    // Process any remaining records in the last chunk
    if !chunk.is_empty() {
        println!("Processing remaining records");
        for r in &chunk {
            println!("Dynamic Record: {:?}", r);
        }
    }

    Ok(())
}
