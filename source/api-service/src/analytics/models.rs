use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricRequest {
    #[serde(rename = "type")]
    pub request_type: String,

    pub dataset: Dataset,

    #[serde(default)]
    pub timePeriod: TimePeriod,

    #[serde(default)]
    pub filter: Filter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(default = "default_granularity")]
    pub granularity: String, // optional, default "Daily"

    pub aggregation: String,
    pub direction: String,
}

fn default_granularity() -> String {
    "Daily".to_string()
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TimePeriod {
    #[serde(default)]
    pub window: i32, // default 0
    #[serde(default)]
    pub from: Option<String>, // default null
    #[serde(default)]
    pub to: Option<String>, // default null
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Filter {
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub subscriber: Option<String>,
}

// Metrics returned by the API

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalMetric {
    pub date: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryMetric {
    pub date: String,
    pub country: String,
    pub operator: String,
    pub value: i32,
}
