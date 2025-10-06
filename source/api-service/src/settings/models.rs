use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub country_id: i32,
    pub iso_code: String,
    pub country_name: String,
    pub created_at: Option<NaiveDateTime>,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}


