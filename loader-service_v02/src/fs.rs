
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

pub struct FileManager;

impl FileManager {
    // Read Directory
    pub fn read_directory(path: &str) -> io::Result<Vec<String>> {
        let mut file_list = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            file_list.push(entry.path().display().to_string());
        }
        Ok(file_list)
    }

    // Read File
    pub fn read_file(file_path: &str) -> io::Result<String> {
        fs::read_to_string(file_path)
    }

    // Write to File
    pub fn write_to_file(file_path: &str, content: &str) -> io::Result<()> {
        let mut file = fs::File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    // Delete File
    pub fn delete_file(file_path: &str) -> io::Result<()> {
        if Path::new(file_path).exists() {
            fs::remove_file(file_path)?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }
    }

    // Create Directory
    pub fn create_directory(path: &str) -> io::Result<()> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    // Check if File Exists
    pub fn file_exists(file_path: &str) -> bool {
        Path::new(file_path).exists()
    }
}
