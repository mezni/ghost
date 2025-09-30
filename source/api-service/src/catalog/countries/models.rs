use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents a row in `dim_countries`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Country {
    pub country_id: i32,
    pub iso_code: String,
    pub country_name: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

/// Used when creating a new country
#[derive(Debug, Serialize, Deserialize)]
pub struct NewCountry {
    pub iso_code: String,
    pub country_name: String,
    pub created_by: String,
}

/// Used when updating a country
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCountry {
    pub iso_code: String,
    pub country_name: String,
    pub updated_by: String,
}

/// Response DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryResponse {
    pub country_id: i32,
    pub iso_code: String,
    pub country_name: String,
}

impl From<Country> for CountryResponse {
    fn from(country: Country) -> Self {
        CountryResponse {
            country_id: country.country_id,
            iso_code: country.iso_code,
            country_name: country.country_name,
        }
    }
}
