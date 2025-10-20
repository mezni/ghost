use crate::core::metrics::{API_CALL_COUNTER, API_CALL_FAILURE_COUNTER};
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures::future::{LocalBoxFuture, Ready, ok};
use std::task::{Context, Poll};

/// Middleware for tracking API calls (total + failed) with normalized endpoints
pub struct ApiCallTracker;

impl<S, B> Transform<S, ServiceRequest> for ApiCallTracker
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiCallTrackerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiCallTrackerMiddleware { service })
    }
}

pub struct ApiCallTrackerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ApiCallTrackerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Normalize the endpoint path
        let normalized_path = normalize_path(req.path());

        // Increment total API call counter
        API_CALL_COUNTER
            .with_label_values(&[&normalized_path])
            .inc();

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            // Increment failed counter for non-2xx status codes
            if !res.status().is_success() {
                API_CALL_FAILURE_COUNTER
                    .with_label_values(&[&normalized_path])
                    .inc();
            }
            Ok(res)
        })
    }
}

/// Normalize paths: replace numeric segments with `{id}`
/// e.g., /api/v1/countries/1 -> /api/v1/countries/{id}
fn normalize_path(path: &str) -> String {
    path.split('/')
        .map(|p| {
            if p.chars().all(|c| c.is_ascii_digit()) {
                "{id}"
            } else {
                p
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}
