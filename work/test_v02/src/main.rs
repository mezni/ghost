use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
struct CountryCode {
    #[serde(rename = "CallingCode")]
    code: u16,
    #[serde(rename = "CountryName")]
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("countrycodes.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let country_codes: Vec<CountryCode> = reader.deserialize().collect::<Result<_, _>>()?;

    let mut map = BTreeMap::new();
    for country_code in country_codes {
        map.insert(country_code.code, country_code.name);
    }

    // Print the map
    for (code, name) in &map {
        println!("Code: {}, Name: {}", code, name);
    }

    Ok(())
}