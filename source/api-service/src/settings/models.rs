// settings/models.rs
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub country_id: i32,
    pub iso_code: String,
    pub country_name: String,
    pub is_valid: bool,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCountry {
    pub iso_code: String,
    pub country_name: String,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCountry {
    pub iso_code: Option<String>,
    pub country_name: Option<String>,
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryDTO {
    pub country_id: i32,
    pub iso_code: String,
    pub country_name: String,
}

impl From<Country> for CountryDTO {
    fn from(country: Country) -> Self {
        CountryDTO {
            country_id: country.country_id,
            iso_code: country.iso_code,
            country_name: country.country_name,
        }
    }
}
