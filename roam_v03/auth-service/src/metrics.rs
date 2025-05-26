use actix_web_prom::PrometheusMetrics;
use actix_web_prom::PrometheusMetricsBuilder;
use once_cell::sync::Lazy;
use prometheus::{Counter, register_counter};

// Initialize Prometheus middleware for Actix-Web
pub fn init_metrics() -> PrometheusMetrics {
    PrometheusMetricsBuilder::new("auth_service")
        .endpoint("/api/v1/metrics")
        .build()
        .expect("Failed to create PrometheusMetrics")
}

// Custom application metrics
pub static LOGIN_COUNTER: Lazy<Counter> = Lazy::new(|| {
    register_counter!("login_requests_total", "Total number of login requests")
        .expect("Failed to register login_requests_total counter")
});

pub static REGISTER_COUNTER: Lazy<Counter> = Lazy::new(|| {
    register_counter!(
        "register_requests_total",
        "Total number of register requests"
    )
    .expect("Failed to register register_requests_total counter")
});
