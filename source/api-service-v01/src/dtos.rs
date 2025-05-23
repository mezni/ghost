use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateCountryDto {
    pub country_name: String,
    pub iso: String,
}

#[derive(Debug, Serialize)]
pub struct CountryDto {
    pub id: i32,
    pub country_name: String,
    pub iso: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOperatorDto {
    pub operator_name: String,
    pub country_id: i32,
}

#[derive(Debug, Serialize)]
pub struct OperatorDto {
    pub id: i32,
    pub operator_name: String,
    pub country_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanDto {
    pub country_id: i32,
    pub operator_id: i32,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct PlanDto {
    pub id: i32,
    pub country_id: i32,
    pub operator_id: i32,
    pub percentage: f64,
}
