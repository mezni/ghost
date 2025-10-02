use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricRequest {
    #[serde(rename = "type")]
    pub request_type: String,

    pub dataset: Dataset,

    pub timePeriod: TimePeriod,

    #[serde(default)]
    pub filter: Filter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(default = "default_granularity")]
    pub granularity: String,

    pub aggregation: String,
    pub direction: String,
}

fn default_granularity() -> String {
    "Daily".to_string()
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TimePeriod {
    #[serde(default)]
    pub window: i32,
    pub from: Option<String>,
    pub to: Option<String>,
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
