use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalMetric {
    pub date: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryMetric {
    pub date: String,
    pub country: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricRequest {
    pub r#type: String,      // e.g., "metric"
    pub direction: String,   // "IN" or "OUT"
    pub aggregation: String, // "GLOBAL", "COUNTRY", etc.
    pub granularity: String, // "DAILY", "MONTHLY"
    pub window: i32,         // e.g., 0
}
