use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// =====================
// Country Models
// =====================
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
        Self {
            country_id: country.country_id,
            iso_code: country.iso_code,
            country_name: country.country_name,
        }
    }
}

// =====================
// Operator Models
// =====================
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOperator {
    pub operator_name: String,
    pub brand_name: Option<String>,
    pub country_name: String,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOperator {
    pub operator_id: i32,
    pub operator_name: Option<String>,
    pub brand_name: Option<String>,
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorDTO {
    pub operator_id: i32,
    pub operator_name: String,
    pub country_name: String,
}

impl From<Operator> for OperatorDTO {
    fn from(op: Operator) -> Self {
        Self {
            operator_id: op.operator_id,
            operator_name: op.operator_name,
            country_name: op.country_name,
        }
    }
}


// =====================
// Network Models
// =====================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub network_id: i32,
    pub plmn_code: String,
    pub plmn: String,
    pub mcc: String,
    pub mnc: String,
    pub operator_id: i32,
    pub country_name: String, 
    pub operator_name: String,    
    pub tech_2g: bool,
    pub tech_3g: bool,
    pub tech_lte: bool,
    pub is_valid: bool,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNetwork {
    pub plmn_code: String,
    pub plmn: String,
    pub mcc: String,
    pub mnc: String,
    pub country_name: String, 
    pub operator_name: String,  
    pub tech_2g: Option<bool>,
    pub tech_3g: Option<bool>,
    pub tech_lte: Option<bool>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNetwork {
    pub network_id: i32,
    pub plmn_code: Option<String>,
    pub plmn: Option<String>,
    pub mcc: Option<String>,
    pub mnc: Option<String>,
    pub tech_2g: Option<bool>,
    pub tech_3g: Option<bool>,
    pub tech_lte: Option<bool>,
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDTO {
    pub network_id: i32,
    pub plmn_code: String,
    pub plmn: String,
    pub mcc: String,
    pub mnc: String,
    pub country_name: String,
    pub operator_name: String,
    pub tech_2g: bool,
    pub tech_3g: bool,
    pub tech_lte: bool,
}

impl From<Network> for NetworkDTO {
    fn from(net: Network) -> Self {
        Self {
            network_id: net.network_id,
            plmn_code: net.plmn_code,
            plmn: net.plmn,
            mcc: net.mcc,
            mnc: net.mnc,
            country_name: net.country_name,
            operator_name: net.operator_name,
            tech_2g: net.tech_2g,
            tech_3g: net.tech_3g,
            tech_lte: net.tech_lte,
        }
    }
}

