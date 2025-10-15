use crate::core::errors::AppError;
use crate::core::logger::Logger;
use std::fs;

use std::path::Path;
use std::path::PathBuf;

use regex::Regex;

pub fn check_directory(directory: &PathBuf) -> bool {
    directory.is_dir()
}

pub fn get_first_n_files(dir_path: &str, n: usize) -> Result<Vec<String>, AppError> {
    let mut files = Vec::new();
    let dir = fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path.file_name().unwrap().to_string_lossy().into_owned());
            if files.len() >= n {
                break;
            }
        }
    }

    Ok(files)
}

pub fn get_files(
    directory: &PathBuf,
    pattern: Option<&str>,
    file_to_process: usize,
) -> Result<Vec<String>, AppError> {
    // Validate directory first
    if !check_directory(directory) {
        return Err(AppError::InvalidDirectory(
            directory.to_string_lossy().into_owned(),
        ));
    }

    // Compile regex pattern if provided
    let regex = if let Some(pattern) = pattern {
        Some(
            Regex::new(pattern)
                .map_err(|e| AppError::InvalidPattern(pattern.to_string(), e.to_string()))?,
        )
    } else {
        None
    };

    let mut files = Vec::with_capacity(file_to_process.min(100)); // Reasonable initial capacity

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path
            .file_name()
            .ok_or_else(|| AppError::InvalidFileName(path.to_string_lossy().into_owned()))?
            .to_string_lossy();

        // Apply pattern filter if provided
        if let Some(regex) = &regex {
            if !regex.is_match(&file_name) {
                continue;
            }
        }

        files.push(file_name.into_owned());

        // Early exit if we have enough files
        if files.len() >= file_to_process {
            break;
        }
    }

    Ok(files)
}
