use crate::core::errors::AppError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DEFAULT_WINDOW: u32 = 0;
const DEFAULT_SIZE: u32 = 30;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricsRequest {
    pub dimension: String,
    pub direction: Option<String>,
    pub window: Option<u32>,
    pub size: Option<u32>,
    pub aggregation: Option<String>,
    pub period: Option<Period>,
    pub filter: Option<Filter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filter {
    pub operator: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct ValidatedMetricsRequest {
    pub dimension: String,
    pub direction: String,
    pub window: u32,
    pub size: u32,
    pub aggregation: Option<String>,
    pub period: Option<Period>,
    pub filter: Option<Filter>,
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
            return Err(AppError::bad_request("Invalid dimension"));
        }

        let direction = match dimension.as_str() {
            "global" | "country" | "operator" | "subscriber" => {
                let dir = self
                    .direction
                    .ok_or_else(|| AppError::bad_request("Direction is required"))?
                    .to_lowercase();
                if !matches!(dir.as_str(), "in" | "out") {
                    return Err(AppError::bad_request(
                        "Invalid direction: must be IN or OUT",
                    ));
                }
                dir
            }
            _ => self.direction.unwrap_or_default().to_lowercase(),
        };

        let window = match self.window {
            Some(w) => {
                if w <= 0 {
                    return Err(AppError::bad_request("Window must be greater than 0"));
                }
                w
            }
            None => {
                if let Some(p) = self.period.as_ref() {
                    if p.start == p.end { 0 } else { DEFAULT_WINDOW }
                } else {
                    DEFAULT_WINDOW
                }
            }
        };

        let size = self.size.unwrap_or(DEFAULT_SIZE);
        if size <= 0 {
            return Err(AppError::bad_request("Size must be greater than 0"));
        }

        Ok(ValidatedMetricsRequest {
            dimension,
            direction,
            window,
            size,
            aggregation: self.aggregation,
            period: self.period,
            filter: self.filter,
        })
    }
}
