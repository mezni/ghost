use actix_web::{HttpResponse, Responder, get};
use lazy_static::lazy_static;
use prometheus::{
    Encoder, IntCounter, IntCounterVec, register_int_counter, register_int_counter_vec,
};

lazy_static! {
    pub static ref REGISTRATION_COUNTER: IntCounter = register_int_counter!(
        "api_user_registration_total",
        "Total number of successful user registrations"
    ).unwrap();

    pub static ref REGISTRATION_FAILURE_COUNTER: IntCounter = register_int_counter!(
        "api_user_registration_failure_total",
        "Total number of failed user registration attempts"
    ).unwrap();

    pub static ref LOGIN_COUNTER: IntCounter = register_int_counter!(
        "api_user_login_total",
        "Total number of successful logins"
    ).unwrap();

    pub static ref LOGIN_FAILURE_COUNTER: IntCounter = register_int_counter!(
        "api_user_login_failure_total",
        "Total number of failed login attempts"
    ).unwrap();

    pub static ref API_CALL_COUNTER: IntCounterVec = register_int_counter_vec!(
        "api_calls_total",
        "Total number of API calls",
        &["endpoint"]
    ).unwrap();

    // ✅ Failed API calls
    pub static ref API_CALL_FAILURE_COUNTER: IntCounterVec = register_int_counter_vec!(
        "api_calls_failed_total",
        "Total number of failed API calls (non-2xx)",
        &["endpoint"]
    ).unwrap();
}

#[get("/metrics")]
pub async fn metrics() -> impl Responder {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer)
}
