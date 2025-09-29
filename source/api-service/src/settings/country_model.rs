use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub country_id: i32,
    pub iso_code: String,
    pub country_name: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCountry {
    pub iso_code: String,
    pub country_name: String,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCountry {
    pub iso_code: Option<String>,
    pub country_name: Option<String>,
    pub updated_by: String,
}
