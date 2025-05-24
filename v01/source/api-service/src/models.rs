use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub country_id: i32,
    pub country_name: String,
    pub iso: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCountry {
    pub country_name: String,
    pub iso: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operator {
    pub operator_id: i32,
    pub operator_name: String,
    pub country_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOperator {
    pub operator_name: String,
    pub country_id: i32,
    pub created_by: Option<String>,
}
