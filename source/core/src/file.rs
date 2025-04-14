use crate::config::{AppConfig, Source};
use crate::entities::RoamOutDTO;
use crate::errors::AppError;
use chrono::Local;
use regex::Regex;
use std::path::{Path, PathBuf};

use csv::{ReaderBuilder, Trim};
use std::fs;
use std::{fs::File, io::BufReader};

const PROCESS_DIR_NAME: &str = "PROCESS";
const REJECTED_DIR_NAME: &str = "REJECTED";
const PROCESSED_DIR_NAME: &str = "PROCESSED";

pub struct FileManager {
    config: AppConfig,
    work_base_dir: PathBuf,
}

impl FileManager {
    pub fn new(config: AppConfig) -> Result<Self, AppError> {
        let work_base_dir = PathBuf::from("/app/WORK");

        Ok(FileManager {
            config,
            work_base_dir,
        })
    }

    pub async fn next(&self) -> Result<Option<(PathBuf, String)>, AppError> {
        for source in &self.config.sources {
            let source_dir = self.work_base_dir.join(&source.source_directory);
            //            println!("{:#?}", source_dir);
            let mut files_vec: Vec<_> = match fs::read_dir(&source_dir) {
                Ok(files) => files.flatten().collect(),
                Err(_) => continue,
            };
            files_vec.sort_by_key(|entry| entry.file_name());
            /*
                        for file in &files_vec {
                            println!("File found: {:?}", file.path());
                        }
            */
            let file_pattern_regex = Regex::new(&source.file_pattern)
                .map_err(|err| AppError::Unexpected(format!("Invalid file pattern: {}", err)))?;

            if let Some(file) = files_vec.iter().find(|entry| {
                let file_name_os = entry.file_name();
                let file_name = file_name_os.to_string_lossy();
                file_pattern_regex.is_match(&file_name)
            }) {
                if source.source_type.to_uppercase() == "ROAM_IN" {
                    return Ok(Some((file.path(), "ROAM_IN".to_string())));
                } else if source.source_type.to_uppercase() == "ROAM_OUT" {
                    return Ok(Some((file.path(), "ROAM_OUT".to_string())));
                }
            }
        }

        Ok(None)
    }

    pub async fn read_file_roam_out(
        &self,
        base_dir: PathBuf,
        file_name: String,
    ) -> Result<Vec<RoamOutDTO>, AppError> {
        let full_path = base_dir.join(file_name);
        let file = File::open(&full_path)?;
        let mut reader = ReaderBuilder::new()
            .trim(Trim::All)
            .from_reader(BufReader::new(file));

        let mut records = Vec::new();
        for result in reader.records() {
            let record = result?;

            let imsi = record[0].to_string();
            let msisdn = record[1].to_string();
            let vlr_number = record[2].to_string();

            records.push(RoamOutDTO {
                imsi,
                msisdn,
                vlr_number,
            });
        }

        Ok(records)
    }

    pub fn extract_and_format_date(&self, filename: &str) -> String {
        let re = Regex::new(r"\d{8}").unwrap(); // Match 8-digit date
        if let Some(mat) = re.find(filename) {
            let date_str = mat.as_str();
            return format!(
                "{}-{}-{}",
                &date_str[0..4], // Year
                &date_str[4..6], // Month
                &date_str[6..8]  // Day
            );
        }

        // Return today's date if no match is found
        Local::now().format("%Y-%m-%d").to_string()
    }

    pub fn delete_file(&self, path: &Path) -> Result<(), AppError> {
        if path.exists() {
            fs::remove_file(path).map_err(AppError::IoError)?;
        }
        Ok(())
    }

    pub fn archive_file(&self, source: &Path) -> Result<PathBuf, AppError> {
        let file_name = source
            .file_name()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid source file name")
            })
            .map_err(AppError::IoError)?;

        // Find corresponding source config
        let source_config = self.config.sources.iter().find(|s| {
            let full_input_dir = self.work_base_dir.join(&s.source_directory);
            source.starts_with(&full_input_dir)
        });

        let destination = if let Some(config) = source_config {
            let archive_dir = config
                .archive_directory
                .as_ref()
                .map(|d| self.work_base_dir.join(d))
                .ok_or_else(|| AppError::Unexpected("Missing archive_directory".to_string()))?;

            archive_dir.join(file_name)
        } else {
            return Err(AppError::Unexpected(
                "Could not match source to config".to_string(),
            ));
        };

        std::fs::rename(source, &destination).map_err(AppError::IoError)?;
        Ok(destination)
    }
}
