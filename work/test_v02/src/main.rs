use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub struct FileSource {
    pub id: u32,
    pub path: String,
}

pub struct FileStat {
    pub size: u64,
}

impl FileSource {
    pub fn new(path: String) -> Self {
        let filename = Path::new(&path).file_name().unwrap().to_str().unwrap();
        let id = Self::hash_filename(filename);
        FileSource { path, id }
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub fn filename(&self) -> Option<String> {
        Path::new(&self.path)
            .file_name()
            .and_then(|f| f.to_str())
            .map(|s| s.to_string())
    }

    fn hash_filename(filename: &str) -> u32 {
        let mut hasher = DefaultHasher::new();
        filename.hash(&mut hasher);
        hasher.finish() as u32
    }

    pub fn metadata(&self) -> std::io::Result<fs::Metadata> {
        fs::metadata(&self.path)
    }
}

fn main() {
    let file_source = FileSource::new("cdr.csv".to_string());

    println!("File path: {}", file_source.path);
    println!("File exists: {}", file_source.exists());
    println!("Filename: {:?}", file_source.filename().unwrap().to_string());
    println!("File ID: {}", file_source.id);

    match file_source.metadata() {
        Ok(metadata) => {
            println!("File size: {} bytes", metadata.len());
            let file_stat = FileStat { size: metadata.len() };
            println!("FileStat size: {}", file_stat.size);
        }
        Err(err) => println!("Error getting metadata: {}", err),
    }
}