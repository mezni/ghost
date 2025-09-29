// src/settings/countries/models.rs

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

/// Payload for creating a new country
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCountry {
    pub iso_code: String,
    pub country_name: String,
    pub created_by: String,
}

/// Payload for updating an existing country
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCountry {
    pub country_name: Option<String>,
    pub updated_by: String,
}
