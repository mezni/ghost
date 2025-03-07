use rusqlite::{params, Connection, Result};
use csv::{ReaderBuilder, StringRecord, Trim};
use std::fs::File;
use std::io::{BufReader, Error as IoError};
use std::error::Error;

// Struct to hold configuration details
struct Config {
    db_path: String,
    csv_path: String,
}

// Function to establish a connection to the SQLite database
fn establish_connection(db_path: &str) -> Result<Connection, rusqlite::Error> {
    Connection::open(db_path)
}

// Function to create the country_codes table if it doesn't exist
fn create_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS country_codes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code INTEGER NOT NULL,
            name TEXT NOT NULL
        );
    ")?;
    Ok(())
}

// Function to read records from the CSV file
fn read_csv_records(csv_path: &str) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let file = File::open(csv_path)?;
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut records = Vec::new();
    for result in reader.records() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => eprintln!("Skipping invalid record: {:?}", e),
        }
    }
    Ok(records)
}

// Function to insert records into the database
fn insert_records(conn: &Connection, records: Vec<StringRecord>) -> Result<(), rusqlite::Error> {
    for record in records {
        if record.len() < 2 {
            eprintln!("Skipping record with missing fields: {:?}", record);
            continue;
        }

        let country_code: i32 = match record[0].parse() {
            Ok(code) => code,
            Err(_) => {
                eprintln!("Invalid country code: {}", record[0].to_string());
                continue;
            }
        };

        let country_name = &record[1];

        conn.execute(
            "INSERT INTO country_codes (code, name) VALUES (?1, ?2)",
            params![country_code, country_name],
        )?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define configuration
    let config = Config {
        db_path: "telco.db".to_string(),
        csv_path: "countrycodes.csv".to_string(),
    };

    // Establish database connection
    let conn = establish_connection(&config.db_path)?;

    // Create table
    create_table(&conn)?;

    // Read CSV records
    let records = read_csv_records(&config.csv_path)?;

    // Insert records into the database
    insert_records(&conn, records)?;

    println!("CSV data successfully imported into SQLite.");
    Ok(())
}
