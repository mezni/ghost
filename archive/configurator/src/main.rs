use reqwest;
use scraper::{Html, Selector};
use csv::Writer;
use std::error::Error;

#[derive(Debug)]
struct Country {
    name_en: String,
    name_fr: String,
    dial_code: String,
    iso_alpha2: String,
    iso_alpha3: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Fetch Wikipedia's list of country calling codes
    let url = "https://en.wikipedia.org/wiki/List_of_country_calling_codes";
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);

    // Select the correct table (Wikipedia may have multiple tables)
    let table_selector = Selector::parse("table.wikitable").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td, th").unwrap(); // Include <th> for headers

    let mut countries = Vec::new();

    for table in document.select(&table_selector) {
        for row in table.select(&row_selector).skip(1) { // Skip header row
            let cells: Vec<_> = row.select(&cell_selector).collect();

            // Ensure we have enough columns (adjust index as needed)
            if cells.len() >= 4 {
                let name_en = cells[0].text().collect::<String>().trim().to_string();
                let dial_code = cells[1].text().collect::<String>().trim().to_string();
                
                // ISO codes (adjust indices based on Wikipedia's current structure)
                let iso_alpha2 = cells.get(2)
                    .map_or("".to_string(), |c| c.text().collect::<String>().trim().to_string());
                
                let iso_alpha3 = cells.get(3)
                    .map_or("".to_string(), |c| c.text().collect::<String>().trim().to_string());

                // For now, duplicate English name as French name
                let name_fr = name_en.clone();

                countries.push(Country {
                    name_en,
                    name_fr,
                    dial_code,
                    iso_alpha2,
                    iso_alpha3,
                });
            }
        }
    }

    // Debug: Print number of countries scraped
    println!("Scraped {} countries.", countries.len());

    // Write to CSV
    let mut writer = Writer::from_path("countries.csv")?;
    writer.write_record(&["English Name", "French Name", "Dial Code", "ISO Alpha-2", "ISO Alpha-3"])?;

    for country in countries {
        writer.write_record(&[
            country.name_en,
            country.name_fr,
            country.dial_code,
            country.iso_alpha2,
            country.iso_alpha3,
        ])?;
    }

    writer.flush()?;
    println!("Data saved to countries.csv");
    Ok(())
}