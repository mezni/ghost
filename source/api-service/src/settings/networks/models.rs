use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub network_id: i32,
    pub plmn_code: String,
    pub plmn: String,
    pub mcc: String,
    pub mnc: String,
    pub operator_id: i32,      // internal DB ID
    pub operator_name: String, // human-readable
    pub country_id: i32,       // internal DB ID
    pub country_name: String,  // human-readable
    pub tech_2g: Option<String>,
    pub tech_3g: Option<String>,
    pub tech_lte: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNetwork {
    pub plmn_code: String,
    pub plmn: String,
    pub mcc: String,
    pub mnc: String,
    pub operator_name: String, // input by user
    pub country_name: String,  // input by user
    pub tech_2g: Option<String>,
    pub tech_3g: Option<String>,
    pub tech_lte: Option<String>,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNetwork {
    pub plmn_code: Option<String>,
    pub plmn: Option<String>,
    pub mcc: Option<String>,
    pub mnc: Option<String>,
    pub operator_name: Option<String>,
    pub country_name: Option<String>,
    pub tech_2g: Option<String>,
    pub tech_3g: Option<String>,
    pub tech_lte: Option<String>,
    pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkResponse {
    pub network_id: i32,
    pub plmn_code: String,
    pub plmn: String,
    pub mcc: String,
    pub mnc: String,
    pub operator_name: String,
    pub country_name: String,
    pub tech_2g: Option<String>,
    pub tech_3g: Option<String>,
    pub tech_lte: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

impl From<Network> for NetworkResponse {
    fn from(n: Network) -> Self {
        NetworkResponse {
            network_id: n.network_id,
            plmn_code: n.plmn_code,
            plmn: n.plmn,
            mcc: n.mcc,
            mnc: n.mnc,
            operator_name: n.operator_name,
            country_name: n.country_name,
            tech_2g: n.tech_2g,
            tech_3g: n.tech_3g,
            tech_lte: n.tech_lte,
            created_at: n.created_at,
            created_by: n.created_by,
            updated_at: n.updated_at,
            updated_by: n.updated_by,
        }
    }
}
