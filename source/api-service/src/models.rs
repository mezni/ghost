use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub id: i32,
    pub country_name: String,
    pub iso: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCountry {
    pub country_name: String,
    pub iso: String,
}
