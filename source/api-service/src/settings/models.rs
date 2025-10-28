use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCountry {
    pub iso_code: String,
    pub country_name: String,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCountry {
    pub iso_code: Option<String>,
    pub country_name: Option<String>,
    pub is_valid: Option<bool>,
    pub updated_by: String,
}
