// services/file_manager.rs
use crate::core::errors::AppError;
use crate::core::logger::Logger;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::task;

/// Check if a given directory exists and is a directory
pub fn check_directory(directory: &Path) -> bool {
    directory.is_dir()
}

/// Delete a file
pub fn delete_file(path: &Path) -> Result<(), AppError> {
    fs::remove_file(path)
        .map_err(|e| AppError::Other(format!("Failed to delete file {:?}: {}", path, e)))
}

/// Ensure a directory exists (create if not)
fn ensure_dir_exists(dir: &Path) -> Result<(), AppError> {
    if !dir.exists() {
        fs::create_dir_all(dir)
            .map_err(|e| AppError::Other(format!("Failed to create directory {:?}: {}", dir, e)))?;
    }
    Ok(())
}

pub async fn archive_file(
    source_path: &Path,
    archive_dir: &Path,
    action: &str,
) -> Result<(), AppError> {
    let source = source_path.to_owned();
    let archive = archive_dir.to_owned();
    let action = action.to_lowercase();

    task::spawn_blocking(move || {
        ensure_dir_exists(&archive)?;

        let file_name = source
            .file_name()
            .ok_or_else(|| AppError::Other(format!("Invalid file name: {}", source.display())))?;

        let destination = archive.join(file_name);

        match action.as_str() {
            "move" => fs::rename(&source, &destination),
            "copy" => fs::copy(&source, &destination).map(|_| ()),
            other => {
                return Err(AppError::Other(format!(
                    "Unsupported archive action: {}",
                    other
                )));
            }
        }
        .map_err(|e| {
            AppError::Other(format!(
                "Failed to {} file {:?} → {:?}: {}",
                action, source, destination, e
            ))
        })?;

        Ok(())
    })
    .await
    .map_err(|e| AppError::Other(format!("Task join error: {}", e)))?
}

pub fn get_files(
    directory: &Path,
    pattern: Option<&str>,
    file_limit: usize,
) -> Result<Vec<String>, AppError> {
    if !check_directory(directory) {
        return Err(AppError::Other(format!(
            "Invalid directory: {}",
            directory.display()
        )));
    }

    let regex = match pattern {
        Some(p) => Some(
            Regex::new(p)
                .map_err(|e| AppError::Other(format!("Invalid regex pattern '{}': {}", p, e)))?,
        ),
        None => None,
    };

    let mut files = Vec::with_capacity(file_limit.min(100));
    for entry in fs::read_dir(directory)
        .map_err(|e| AppError::Other(format!("Failed to read directory: {}", e)))?
    {
        let entry =
            entry.map_err(|e| AppError::Other(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = path
            .file_name()
            .ok_or_else(|| AppError::Other(format!("Invalid file name: {}", path.display())))?
            .to_string_lossy()
            .into_owned();

        if file_name != "status.txt"
            && regex
                .as_ref()
                .map_or(true, |regex| regex.is_match(&file_name))
        {
            files.push(file_name);
            if files.len() >= file_limit {
                break;
            }
        }
    }

    Ok(files)
}

/// Handle a file action: delete, move, or copy
pub async fn handle_file_action(
    file_path: &Path,
    archive_path: &Option<PathBuf>,
    file_action: &str,
) -> Result<(), AppError> {
    match file_action.to_lowercase().as_str() {
        "delete" => {
            delete_file(file_path)?;
            Logger::info(&format!("🗑 Deleted file: {:?}", file_path));
        }
        "move" | "copy" => {
            let archive_dir = archive_path.as_ref().ok_or_else(|| {
                AppError::Other("Archive path is required for move/copy action".into())
            })?;

            archive_file(file_path, archive_dir.as_path(), file_action).await?;
            Logger::info(&format!(
                "📦 {} file: {:?} → {:?}",
                file_action.to_uppercase(),
                file_path,
                archive_dir
            ));
        }
        _ => {
            delete_file(file_path)?;
            Logger::info(&format!(
                "⚠️ Unrecognized action '{}', defaulted to delete: {:?}",
                file_action, file_path
            ));
        }
    }

    Ok(())
}
