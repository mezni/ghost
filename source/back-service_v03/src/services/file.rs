use csv::ReaderBuilder;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug)]
pub struct RoamOutRecord {
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
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
