use csv::ReaderBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the path to your CSV file
    let file_path = "example.csv";

    // Create a CSV reader with default settings
    let mut reader = ReaderBuilder::new()
        .flexible(true) // Allows rows with varying numbers of fields
        .from_path(file_path)?;

    // Iterate through each record in the CSV file
    for result in reader.records() {
        // Each record is a Result<StringRecord, Error>
        let record = result?;
        println!("{:?}", record); // Print the record as a vector of strings
    }

    Ok(())
}