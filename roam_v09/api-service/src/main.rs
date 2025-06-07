use std::sync::Arc;

use chrono::Utc;
use sqlx::postgres::PgPoolOptions;

mod domain;

mod infra;

use domain::countries::{Country, CountryRepository};
use infra::store::countries::PgCountryRepository;

const DATABASE_URL: &str = "postgres://user:pass@localhost:5432/roam";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to database at {}", DATABASE_URL);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    let pool = Arc::new(pool);

    let repo = PgCountryRepository::new(pool);

    // Build a new Country entity
    let country = Country::builder("canada", "ca").created_by("admin").build();

    // Insert country into DB
    let inserted = repo.insert(country).await?;
    println!("Inserted country: {:?}", inserted);

    // Fetch by ID
    if let Some(fetched) = repo.get_by_id(inserted.id.unwrap()).await? {
        println!("Fetched country by ID: {:?}", fetched);
    } else {
        println!("Country not found");
    }

    Ok(())
}
