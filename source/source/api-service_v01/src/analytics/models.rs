use crate::core::errors::AppError;
use serde::{Deserialize, Serialize};

const DEFAULT_SIZE: u32 = 30;
const DEFAULT_SIZE_TOP: u32 = 5;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsRequest {
    pub dimension: String,
    pub aggregation: Option<String>,
    pub filter: Option<Vec<Filter>>,
    pub size: Option<u32>,
    pub period: Option<Period>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    pub key: String,
    pub value: String,
    pub operator: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Period {
    pub start: String,
    pub end: String,
}

#[derive(Debug)]
pub struct ValidatedMetricsRequest {
    pub dimension: String,
    pub aggregation: String,
    pub filter: Option<Vec<Filter>>,
    pub size: Option<u32>,
    pub period: Option<Period>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalMetric {
    pub date: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryMetric {
    pub date: String,
    pub country: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotifMetric {
    pub date: String,
    pub value: String,
}

impl MetricsRequest {
    pub fn validate(self) -> Result<ValidatedMetricsRequest, AppError> {
        let dimension = self.dimension.to_lowercase();
        if !matches!(
            dimension.as_str(),
            "global"
                | "country"
                | "operator"
                | "subscriber"
                | "performance"
                | "alerts"
                | "notification"
        ) {
            return Err(AppError::BadRequest("Invalid dimension".to_string()));
        }

        let aggregation = if let Some(agg) = &self.aggregation {
            match agg.to_lowercase().as_str() {
                "top" | "latest" | "history" | "summary" | "detail" => agg.to_lowercase(),
                _ => "history".to_string(),
            }
        } else {
            "history".to_string()
        };

        let size = self.size.or_else(|| {
            Some(match aggregation.as_str() {
                "top" => DEFAULT_SIZE_TOP,
                _ => DEFAULT_SIZE,
            })
        });
        Ok(ValidatedMetricsRequest {
            dimension,
            aggregation,
            filter: self.filter,
            size: size,
            period: self.period,
        })
    }
}
