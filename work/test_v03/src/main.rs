use csv::ReaderBuilder;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use uuid::Uuid;

const DEFAULT_CHUNK_SIZE: usize = 10;
const DEFAULT_BUFFER_SIZE: usize = 256 * 1024; // 256K

struct ReadOptions {
    chunk_size: usize,
    buffer_size: usize,
}

impl ReadOptions {
    fn new() -> Self {
        ReadOptions {
            chunk_size: DEFAULT_CHUNK_SIZE,
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }

    fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }
}

struct FileSource {
    file_id: u32,
    path: String,
}

impl FileSource {
    async fn new(path: String) -> Result<Self, std::io::Error> {
        if !Self::exists(&path).await {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File '{}' not found", path),
            ));
        }
        let filename = Self::get_filename(&path);
        let file_id = Self::hash_filename(&filename.unwrap());
        Ok(FileSource { file_id, path })
    }

    fn get_filename(path: &str) -> Option<&str> {
        Path::new(path).file_name().and_then(|name| name.to_str())
    }

    async fn exists(path: &str) -> bool {
        tokio::task::spawn_blocking(move || Path::new(path).exists())
            .await
            .unwrap()
    }

    fn hash_filename(filename: &str) -> u32 {
        let mut hasher = DefaultHasher::new();
        filename.hash(&mut hasher);
        hasher.finish() as u32
    }

    async fn read(&self, options: ReadOptions) -> Result<String, std::io::Error> {
        let file = File::open(&self.path).await?;
        let reader = BufReader::new(file);
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .trim(true)
            .from_reader(reader);
        let mut records_stream = csv_reader.deserialize::<HashMap<String, String>>();

        let chunk_size = ReadOptions.chunk_size;
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
                process_chunk(&chunk).await?;
                chunk.clear();
            }
        }

        if !chunk.is_empty() {
            process_chunk(&chunk).await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let file_path = "example.csv";
    let options = ReadOptions::new().with_chunk_size(100);

    match FileSource::new(file_path.to_string()).await {
        Ok(file_source) => {
            println!("File '{}' found", file_source.path);
            match file_source.read(options).await {
                Ok(content) => println!("File content: {}", content),
                Err(error) => println!("Error reading file: {}", error),
            }
        }
        Err(error) => println!("Error: {}", error),
    }

    println!("File exists: {}", FileSource::exists(file_path).await);
}
