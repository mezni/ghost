use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

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
        Ok(FileSource { path })
    }

    async fn exists(path: &str) -> bool {
        tokio::task::spawn_blocking(move || Path::new(path).exists())
            .await
            .unwrap()
    }

    async fn read(&self, options: ReadOptions) -> Result<String, std::io::Error> {
        let mut file = File::open(&self.path).await?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();

        loop {
            let bytes_read = reader.read_line(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            if buffer.len() >= options.chunk_size {
                break;
            }
        }

        Ok(buffer)
    }
}

#[tokio::main]
async fn main() {
    let file_path = "example.txt";
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