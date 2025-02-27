mod domain;

use fake::Fake;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::FreeEmail;
use serde_json;

use crate::domain::manufacturer::Manufacturer;

fn main() {
    // Generate fake company name and contact email
    let company_name: String = CompanyName().fake();
    let contact: String = FreeEmail().fake();

    // Create a new Manufacturer instance
    let manufacturer = Manufacturer::new(company_name, Some(contact));

    // Print the Manufacturer instance
    println!("Manufacturer: {:?}", manufacturer);

    // Serialize the Manufacturer instance to JSON
    let json = serde_json::to_string_pretty(&manufacturer).unwrap();
    println!("JSON: {}", json);

    // Deserialize the JSON back to a Manufacturer instance
    let deserialized: Manufacturer = serde_json::from_str(&json).unwrap();
    println!("Deserialized Manufacturer: {:?}", deserialized);
}
