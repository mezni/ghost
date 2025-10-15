use crate::core::errors::AppError;
use crate::core::logger::Logger;

use std::fs;

use std::path::Path;

use csv::ReaderBuilder;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug)]
pub struct RoamOutRecord {
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
}

pub fn dir_exists(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn delete_file(path: &str) -> Result<(), AppError> {
    println!("{}", path);
    fs::remove_file(path).map_err(AppError::from)
}

pub fn move_file(source: &str, destination: &str) -> Result<(), AppError> {
    fs::rename(source, destination).map_err(AppError::from)
}

pub fn archive_file(source: &str, destination: &str) -> Result<(), AppError> {
    fs::copy(source, destination).map_err(AppError::from)?;
    fs::remove_file(source).map_err(AppError::from)
}

pub fn get_first_n_files(dir_path: &str, n: usize) -> Result<Vec<String>, std::io::Error> {
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

pub struct RoamOutFileReader {}

impl RoamOutFileReader {
    pub fn read(file_path: &str) -> Result<Vec<RoamOutRecord>, io::Error> {
        let file = File::open(file_path)?;
        let mut reader = ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(true)
            .from_reader(BufReader::new(file));

        let mut records = Vec::new();
        for result in reader.records() {
            let record = result?;
            if record.len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid record length",
                ));
            }

            let imsi = record.get(0).unwrap().trim().to_string();
            let msisdn = record.get(1).unwrap().trim().to_string();
            let vlr_number = record.get(2).unwrap().trim().to_string();

            records.push(RoamOutRecord {
                imsi,
                msisdn,
                vlr_number,
            });
        }

        Ok(records)
    }
}
