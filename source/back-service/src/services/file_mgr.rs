use crate::core::errors::AppError;
use crate::core::logger::Logger;
use std::fs;

use std::path::Path;
use std::path::PathBuf;

use regex::Regex;
use tokio::task;

pub fn check_directory(directory: &PathBuf) -> bool {
    directory.is_dir()
}

pub fn delete_file(path: &PathBuf) -> Result<(), AppError> {
    fs::remove_file(path).map_err(AppError::from)
}

pub async fn archive_file(
    source_path: &PathBuf,
    archive_path: &PathBuf,
    action: &str,
) -> Result<(), AppError> {
    let source = source_path.clone();
    let archive = archive_path.clone();
    let action_str = action.to_string();

    // Use tokio::task::spawn_blocking for file operations since they are blocking
    task::spawn_blocking(move || {
        match action_str.to_lowercase().as_str() {
            "move" => {
                // Create archive directory if it doesn't exist
                if !archive.exists() {
                    fs::create_dir_all(&archive).map_err(|e| {
                        AppError::new(format!("Failed to create archive directory: {}", e))
                    })?;
                }

                let file_name = source
                    .file_name()
                    .ok_or_else(|| AppError::invalid_file_name(source.to_string_lossy()))?;

                let destination = archive.join(file_name);

                // Move the file
                fs::rename(&source, &destination)
                    .map_err(|e| AppError::new(format!("Failed to move file: {}", e)))?;

                Ok(())
            }
            "copy" => {
                // Create archive directory if it doesn't exist
                if !archive.exists() {
                    fs::create_dir_all(&archive).map_err(|e| {
                        AppError::new(format!("Failed to create archive directory: {}", e))
                    })?;
                }

                let file_name = source
                    .file_name()
                    .ok_or_else(|| AppError::invalid_file_name(source.to_string_lossy()))?;

                let destination = archive.join(file_name);

                // Copy the file
                fs::copy(&source, &destination)
                    .map_err(|e| AppError::new(format!("Failed to copy file: {}", e)))?;

                Ok(())
            }
            _ => Err(AppError::new(format!(
                "Unsupported archive action: {}",
                action_str
            ))),
        }
    })
    .await
    .map_err(|e| AppError::new(format!("Task join error: {}", e)))?
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

pub async fn handle_file_action(
    file_path: &PathBuf,
    archive_path: &Option<PathBuf>,
    file_action: &str,
) -> Result<(), AppError> {
    match file_action.to_lowercase().as_str() {
        "delete" => {
            delete_file(file_path)?;
            Logger::info(&format!("Successfully deleted file: {:?}", file_path));
        }
        "move" => {
            if let Some(archive_path) = archive_path {
                archive_file(file_path, archive_path, "move").await?;
                Logger::info(&format!(
                    "Successfully moved file: {:?} to {:?}",
                    file_path, archive_path
                ));
            } else {
                return Err(AppError::new("Archive path is required for move action"));
            }
        }
        "copy" => {
            if let Some(archive_path) = archive_path {
                archive_file(file_path, archive_path, "copy").await?;
                Logger::info(&format!(
                    "Successfully copied file: {:?} to {:?}",
                    file_path, archive_path
                ));
            } else {
                return Err(AppError::new("Archive path is required for copy action"));
            }
        }
        _ => {
            return Err(AppError::new(format!(
                "Unknown file action: {}",
                file_action
            )));
        }
    }
    Ok(())
}
