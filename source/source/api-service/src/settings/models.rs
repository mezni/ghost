use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// =========================
// Country Models
// =========================

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

// =========================
// Operator Models
// =========================

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Operator {
    pub operator_id: i32,
    pub operator_name: String,
    pub brand_name: Option<String>,
    pub country_id: i32,
    pub country_name: String,
    pub is_valid: bool,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOperator {
    pub operator_name: String,
    pub brand_name: Option<String>,
    pub country_id: i32,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateOperator {
    pub operator_name: Option<String>,
    pub brand_name: Option<String>,
    pub country_id: Option<i32>,
    pub is_valid: Option<bool>,
    pub updated_by: String,
}

// =========================
// Network Models
// =========================

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Network {
    pub network_id: i32,
    pub plmn_code: String,
    pub plmn: String,
    pub mcc: String,
    pub mnc: String,
    pub operator_id: i32,
    pub operator_name: String,
    pub country_name: String,
    pub tech_2g: bool,
    pub tech_3g: bool,
    pub tech_lte: bool,
    pub is_valid: bool,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateNetwork {
    pub plmn_code: String,
    pub plmn: String,
    pub mcc: String,
    pub mnc: String,
    pub operator_id: i32,
    pub tech_2g: bool,
    pub tech_3g: bool,
    pub tech_lte: bool,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateNetwork {
    pub plmn_code: Option<String>,
    pub plmn: Option<String>,
    pub mcc: Option<String>,
    pub mnc: Option<String>,
    pub operator_id: Option<i32>,
    pub tech_2g: Option<bool>,
    pub tech_3g: Option<bool>,
    pub tech_lte: Option<bool>,
    pub is_valid: Option<bool>,
    pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SorPlan {
    pub sor_plan_id: i32,
    pub operator_id: i32,
    pub operator_name: String,
    pub country_name: String,
    pub routage_type_id: Option<i32>,
    pub routage_type_name: Option<String>,
    pub barring: bool,
    pub rate: String,
    pub is_current: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub created_by: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<String>, // Change to Option<String>
}

#[derive(Debug, Deserialize)]
pub struct CreateSorPlan {
    pub operator_id: i32,
    pub routage_type_id: Option<i32>,
    pub barring: Option<bool>,
    pub rate: String,
    pub created_by: String,
    pub is_current: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSorPlan {
    pub operator_id: Option<i32>,
    pub routage_type_id: Option<i32>,
    pub barring: Option<bool>,
    pub rate: Option<String>,
    pub is_current: Option<bool>,
    pub updated_by: String, // This can remain non-optional since it's provided in the request
}

// DB model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prefix {
    pub prefix_id: i32,
    pub country_id: i32,
    pub operator_id: Option<i32>, // <--- nullable
    pub prefix: String,
    pub is_valid: bool,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<String>,
    pub country_name: Option<String>,
    pub operator_name: Option<String>,
}

// DTO for creating/updating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrefix {
    pub country_id: i32,
    pub operator_id: i32,
    pub prefix: String,
    pub is_valid: Option<bool>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePrefix {
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
    pub prefix: Option<String>,
    pub is_valid: Option<bool>,
    pub updated_by: String,
}
