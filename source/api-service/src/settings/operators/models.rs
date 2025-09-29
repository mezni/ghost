use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Operator {
    pub operator_id: i32,
    pub operator_name: String,
    pub brand_name: Option<String>,
    pub country_id: i32,
    pub country_name: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOperator {
    pub operator_name: String,
    pub brand_name: Option<String>,
    pub country_name: String,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOperator {
    pub operator_name: Option<String>,
    pub brand_name: Option<String>,
    pub country_name: Option<String>,
    pub updated_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorResponse {
    pub operator_id: i32,
    pub operator_name: String,
    pub brand_name: Option<String>,
    pub country_name: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

impl From<Operator> for OperatorResponse {
    fn from(op: Operator) -> Self {
        OperatorResponse {
            operator_id: op.operator_id,
            operator_name: op.operator_name,
            brand_name: op.brand_name,
            country_name: op.country_name,
            created_at: op.created_at,
            created_by: op.created_by,
            updated_at: op.updated_at,
            updated_by: op.updated_by,
        }
    }
}
