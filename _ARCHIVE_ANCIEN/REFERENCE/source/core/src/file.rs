use crate::config::AppConfig;
use crate::errors::AppError;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use csv::ReaderBuilder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use tokio::fs as tokio_fs;

pub struct FileManager;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileProcessed {
    pub file_path: PathBuf,
    pub file_type: String,
    pub file_action: String,
    pub archive_path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct RoamInDataRecord {
    pub hlraddr: String,
    pub nsub: i32,
    pub nsuba: i32,
}

#[derive(Debug, Serialize)]
pub struct RoamOutDataRecord {
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
}

#[derive(Debug, Serialize)]
struct SummaryRecord {
    totnsub: u64,
    totnsuba: u64,
    nsubpr: u64,
    nsubxp: u64,
    nsubpxou: u64,
    nsubsgs: u64,
    nsubgs: u64,
}

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub creation_date: String,
}

#[derive(Debug, Serialize)]
pub struct RoamInData {
    pub metadata: Metadata,
    pub records: Vec<RoamInDataRecord>,
}

#[derive(Debug, Serialize)]
pub struct RoamOutData {
    pub metadata: Metadata,
    pub records: Vec<RoamOutDataRecord>,
}

#[derive(Debug)]
pub enum ParsedData {
    RoamIn(RoamInData),
    RoamOut(RoamOutData),
}

impl FileManager {
    pub fn new() -> Self {
        FileManager
    }

    pub async fn next(&self, app_config: AppConfig) -> Result<Option<FileProcessed>, AppError> {
        println!("file enter");
        for source in app_config.sources {
            let source_type = source.source_type.to_uppercase();
            let path = Path::new(&source.source_directory);
   
            if source_type == "ROAM_IN" || source_type == "ROAM_OUT" {
                if path.exists() {
                    let mut files_vec: Vec<_> = fs::read_dir(&path)
                        .ok()
                        .into_iter()
                        .flat_map(|read_dir| read_dir.flatten())
                        .collect();
                    
                    if let Some(pattern) = &source.file_pattern {
                        if let Ok(regex) = Regex::new(pattern) {
                            files_vec.retain(|entry| {
                                entry
                                    .path()
                                    .file_name()
                                    .and_then(OsStr::to_str)
                                    .map(|name| regex.is_match(name))
                                    .unwrap_or(false)
                            });
                        }
                    }

                    files_vec.sort_by_key(|entry| entry.file_name());
                    
                    if let Some(first_file) = files_vec.first() {
                        let post_action_upper = source
                            .post_action
                            .as_ref()
                            .map(|a| a.to_uppercase())
                            .unwrap_or_else(|| "DELETE".to_string());

                        let file_action = if post_action_upper == "ARCHIVE" {
                            "ARCHIVE".to_string()
                        } else {
                            "DELETE".to_string()
                        };

                        let archive_path = source.archive_directory.as_ref().map(PathBuf::from);

                        let file_processed = FileProcessed {
                            file_path: first_file.path(),
                            file_type: source_type,
                            file_action,
                            archive_path,
                        };
                        println! ("file {:?}", file_processed);
                        return Ok(Some(file_processed));
                    }
                }
            }
        }

        Ok(None)
    }

    pub async fn remove_file<P: AsRef<Path>>(&self, path: P) -> Result<(), AppError> {
        tokio_fs::remove_file(&path).await?;
        Ok(())
    }

    pub async fn parse_file(&self, file: FileProcessed) -> Result<Option<ParsedData>, AppError> {
        match file.file_type.as_str() {
            "ROAM_IN" => {
                let result = self.roam_in_parser(file).await?;
                Ok(Some(ParsedData::RoamIn(result)))
            }
            "ROAM_OUT" => {
                let result = self.roam_out_parser(file).await?;
                Ok(Some(ParsedData::RoamOut(result)))
            }
            _ => Ok(None),
        }
    }

    pub async fn roam_in_parser(&self, file: FileProcessed) -> Result<RoamInData, AppError> {
        let file = std::fs::File::open(file.file_path)?;
        let reader = BufReader::new(file);

        let re_row = Regex::new(r"(4-\d+)\s+(\d+)\s+(\d+)")?;
        let re_summary = Regex::new(r"([A-Z]+)\s+(\d+)")?;

        let mut creation_date: Option<String> = None;
        let mut in_data_section = false;
        let mut records = Vec::new();
        let mut summary = SummaryRecord {
            totnsub: 0,
            totnsuba: 0,
            nsubpr: 0,
            nsubxp: 0,
            nsubpxou: 0,
            nsubsgs: 0,
            nsubgs: 0,
        };

        for line in reader.lines() {
            let line = line?;

            if line.starts_with("ACT") && line.contains("TIME") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                creation_date = Some(if let Some(date_str) = parts.get(4) {
                    match NaiveDate::parse_from_str(date_str, "%Y%m%d") {
                        Ok(date) => date.format("%Y-%m-%d").to_string(),
                        Err(_) => Local::now().format("%Y-%m-%d").to_string(),
                    }
                } else {
                    Local::now().format("%Y-%m-%d").to_string()
                });
                continue;
            }

            if line.contains("MT MOBILE SUBSCRIBER SURVEY RESULT") {
                in_data_section = true;
                continue;
            }

            if in_data_section {
                for caps in re_row.captures_iter(&line) {
                    records.push(RoamInDataRecord {
                        hlraddr: caps[1].to_string(),
                        nsub: caps[2].parse()?,
                        nsuba: caps[3].parse()?,
                    });
                }

                if let Some(caps) = re_summary.captures(&line) {
                    let key = &caps[1];
                    let value: u64 = caps[2].parse()?;
                    match key {
                        "TOTNSUB" => summary.totnsub = value,
                        "TOTNSUBA" => summary.totnsuba = value,
                        "NSUBPR" => summary.nsubpr = value,
                        "NSUBXP" => summary.nsubxp = value,
                        "NSUBPXOU" => summary.nsubpxou = value,
                        "NSUBSGS" => summary.nsubsgs = value,
                        "NSUBGS" => summary.nsubgs = value,
                        _ => {}
                    }
                }
            }
        }

        let metadata = Metadata {
            creation_date: creation_date
                .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string()),
        };
        Ok(RoamInData { metadata, records })
    }

    pub async fn roam_out_parser(&self, file: FileProcessed) -> Result<RoamOutData, AppError> {
        let f = std::fs::File::open(&file.file_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(f);

        let mut records = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let imsi = record.get(0).unwrap_or("").to_string();
            let msisdn = record.get(1).unwrap_or("").to_string();
            let vlr_number = record.get(2).unwrap_or("").to_string();

            records.push(RoamOutDataRecord {
                imsi,
                msisdn,
                vlr_number,
            });
        }

        let filename = file
            .file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        let creation_date = filename
            .split('_')
            .last()
            .and_then(|part| part.strip_suffix(".txt"))
            .and_then(|datetime_str| {
                NaiveDateTime::parse_from_str(datetime_str, "%Y%m%d%H%M%S")
                    .ok()
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            })
            .unwrap_or_else(|| Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        let metadata = Metadata { creation_date };

        Ok(RoamOutData { metadata, records })
    }
}
