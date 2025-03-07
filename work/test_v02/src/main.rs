use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct CountryCode {
    #[serde(rename = "CallingCode")]
    code: u16,
    #[serde(rename = "CountryName")]
    name: String,
}

fn get_country_code(map: &BTreeMap<u16, String>, number: &str) -> Option<String> {
    let first_four = &number[..4];
    for (code, name) in map {
        if first_four.starts_with(&code.to_string()) {
            return Some(name.clone());
        }
    }
    None
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

    let vlr_number = "16473840003";
    let country_name = get_country_code(&map, vlr_number);
    match country_name {
        Some(name) => println!("Country Name: {}", name),
        None => println!("Country Name not found"),
    }

    Ok(())
}