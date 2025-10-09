use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimePeriod {
    pub start: Option<String>, // start as string
    pub end: Option<String>,   // end as string
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Aggregation {
    pub mesure: Option<String>,
    pub size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsRequest {
    pub metric: String,    // Mandatory
    pub dimension: String, // Mandatory
    pub direction: String, // Mandatory
    pub timeWindow: Option<u32>,
    pub timePeriod: Option<TimePeriod>,   // Optional
    pub filter: Option<Vec<Filter>>,      // Optional, can be an empty list
    pub aggregation: Option<Aggregation>, // Optional
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
