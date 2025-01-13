use chrono::{Utc, Duration};
use rand::Rng;
use serde::{Serialize, Deserialize};
use csv;
use std::fs;
use std::fs::File;
use std::io::{Write};
use serde_json;
use bincode;

// Define the CDR structure
#[derive(Serialize, Deserialize, Debug)]
struct Cdr {
    call_id: u64,
    calling_number: String,
    called_number: String,
    start_time: String,
    end_time: String,
    duration: u32,
    call_type: String,
}

impl Cdr {
    fn new(call_id: u64) -> Self {
        let mut rng = rand::thread_rng();
        let calling_number = format!("21650{}", rng.gen_range(100000..999999));
        let called_number = format!("216{}", rng.gen_range(10000000..99999999));
        
        let now = Utc::now().naive_utc();
        let random_seconds_ago = rng.gen_range(0..(2 * 60 * 60)); // Random start time within the last 2 hours
        let start_time = now - Duration::seconds(random_seconds_ago as i64);
        
        let duration = rng.gen_range(1..3601); // Random duration between 1 second and 1 hour
        let end_time = start_time + Duration::seconds(duration as i64);

        let call_type = if rng.gen_bool(0.5) { "Incoming" } else { "Outgoing" };

        Cdr {
            call_id,
            calling_number,
            called_number,
            start_time: start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            end_time: end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            duration,
            call_type: call_type.to_string(),
        }
    }
}

// Manually encode CDR struct to DER format
impl Cdr {
    fn to_der(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Encoding each field manually into DER format
        result.extend(encode_integer(self.call_id));   // Encoding the call_id as INTEGER
        result.extend(encode_octet_string(self.calling_number.as_bytes())); // Encoding calling_number as OCTET STRING
        result.extend(encode_octet_string(self.called_number.as_bytes()));  // Encoding called_number as OCTET STRING
        result.extend(encode_octet_string(self.start_time.as_bytes()));     // Encoding start_time as OCTET STRING
        result.extend(encode_octet_string(self.end_time.as_bytes()));       // Encoding end_time as OCTET STRING
        result.extend(encode_integer(self.duration.into()));   // Encoding duration as INTEGER
        result.extend(encode_octet_string(self.call_type.as_bytes())); // Encoding call_type as OCTET STRING

        result
    }
}

// Helper function to encode integer to DER format
fn encode_integer(value: u64) -> Vec<u8> {
    let mut result = Vec::new();
    result.push(0x02); // INTEGER tag
    let bytes = value.to_be_bytes();
    result.push(bytes.len() as u8); // Length of the integer in bytes
    result.extend(&bytes);
    result
}

// Helper function to encode octet string to DER format
fn encode_octet_string(value: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    result.push(0x04); // OCTET STRING tag
    result.push(value.len() as u8); // Length of the octet string
    result.extend(value);
    result
}

// Helper function to create the output directory
fn create_output_directory(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dir)?;
    Ok(())
}

// Generate dynamic file name based on current timestamp
fn generate_file_name(prefix: &str) -> String {
    let now = Utc::now();
    format!("{prefix}{}", now.format("%Y%m%d%H%M%S"))
}

// Write CDRs to CSV
fn write_to_csv(cdrs: &[Cdr], dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    create_output_directory(dir)?;  // Use the helper function with the provided directory

    let file_name = dir.to_string() + &generate_file_name("CSV") + ".csv"; 
    let mut wtr = csv::Writer::from_path(&file_name)?;
    
    for cdr in cdrs {
        wtr.serialize(cdr)?;
    }
    
    wtr.flush()?;
    Ok(())
}

// Write CDRs to TSV
fn write_to_tsv(cdrs: &[Cdr], dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    create_output_directory(dir)?;  // Use the helper function with the provided directory

    let file_name = dir.to_string() + &generate_file_name("TSV") + ".tsv"; 
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')  // Specify tab delimiter
        .from_path(&file_name)?;
    
    for cdr in cdrs {
        wtr.serialize(cdr)?;
    }
    
    wtr.flush()?;
    Ok(())
}

// Write CDRs to JSON
fn write_to_json(cdrs: &[Cdr], dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    create_output_directory(dir)?;  // Use the helper function with the provided directory

    let file_name = dir.to_string() + &generate_file_name("JSON") + ".json"; 
    let json_data = serde_json::to_string(cdrs)?;
    fs::write(file_name, json_data)?;
    Ok(())
}

// Write CDRs to Binary
fn write_to_binary(cdrs: &[Cdr], dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    create_output_directory(dir)?;  // Use the helper function with the provided directory

    let file_name = dir.to_string() + &generate_file_name("BIN") + ".bin"; 
    let serialized_data = bincode::serialize(cdrs)?; // Serialize data to binary format
    fs::write(file_name, serialized_data)?; // Write to binary file
    Ok(())
}

// Write CDRs to ASN.1
fn write_to_asn1(cdrs: &[Cdr], dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(dir)?;

    let file_name = dir.to_string() + &format!("ASN1{}", chrono::Utc::now().format("%Y%m%d%H%M%S")) + ".asn1";
    let mut file = File::create(file_name)?;

    // For each CDR, serialize to DER and write to the file
    for cdr in cdrs {
        let cdr_asn1 = cdr.to_der();
        file.write_all(&cdr_asn1)?;
    }

    Ok(())
}

// Generate a specified number of CDRs
fn generate_cdrs(n: u64) -> Vec<Cdr> {
    (1..=n).map(Cdr::new).collect()
}

// Main function to generate and write CDRs to CSV, TSV, JSON, Binary, and ASN.1
fn main() {
    let cdrs = generate_cdrs(1000);
    let dir = "OUTPUT/";  // Directory path for the output files

    if let Err(e) = write_to_csv(&cdrs, dir) {
        eprintln!("Error writing to CSV: {}", e);
    }
    if let Err(e) = write_to_tsv(&cdrs, dir) {
        eprintln!("Error writing to TSV: {}", e);
    }
    if let Err(e) = write_to_json(&cdrs, dir) {
        eprintln!("Error writing to JSON: {}", e);
    }
    if let Err(e) = write_to_binary(&cdrs, dir) {
        eprintln!("Error writing to Binary: {}", e);
    }
    if let Err(e) = write_to_asn1(&cdrs, dir) {
        eprintln!("Error writing to ASN.1: {}", e);
    }
}
