use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents a row from sor_plan + joined country/operator/routage names
#[derive(Debug, Serialize, Deserialize)]
pub struct SorPlan {
    pub sor_plan_id: i32,
    pub country_id: i32,
    pub country_name: String,
    pub operator_id: i32,
    pub operator_name: String,
    pub routage_type_id: i32,
    pub routage_type_name: String,
    pub barring: Option<bool>, // changed to boolean
    pub rate: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
    pub is_current: bool,
    pub version: i32,
}

/// Payload for creating a new sor_plan (DTO)
#[derive(Debug, Serialize, Deserialize)]
pub struct NewSorPlan {
    pub country_name: String,
    pub operator_name: String,
    pub routage_type_name: String,
    pub barring: Option<bool>, // changed to boolean
    pub rate: Option<String>,
    pub created_by: String,
}

/// Payload for updating an existing sor_plan (DTO)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSorPlan {
    pub country_name: String,
    pub operator_name: String,
    pub routage_type_name: String,
    pub barring: Option<bool>, // changed to boolean
    pub rate: Option<String>,
    pub updated_by: String,
}

/// Response DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct SorPlanResponse {
    pub sor_plan_id: i32,
    pub country_name: String,
    pub operator_name: String,
    pub routage_type_name: String,
    pub barring: Option<bool>, // changed to boolean
    pub rate: Option<String>,
}

impl From<SorPlan> for SorPlanResponse {
    fn from(plan: SorPlan) -> Self {
        SorPlanResponse {
            sor_plan_id: plan.sor_plan_id,
            country_name: plan.country_name,
            operator_name: plan.operator_name,
            routage_type_name: plan.routage_type_name,
            barring: plan.barring,
            rate: plan.rate,
        }
    }
}
