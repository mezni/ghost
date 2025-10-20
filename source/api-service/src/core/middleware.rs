use crate::core::logger::Logger;
use actix_web::{
    Error, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::{FutureExt, LocalBoxFuture, Ready, ok};
use std::rc::Rc;
use std::time::Instant;
use tracing::error;

// ─────────────────────────────
// 🧰 Panic & error catching middleware
// ─────────────────────────────
pub struct ErrorMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ErrorMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ErrorMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct ErrorMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ErrorMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            // Split request into parts to safely reuse in panic case
            let (req_parts, payload) = req.into_parts();
            let req_for_service = ServiceRequest::from_parts(req_parts.clone(), payload);

            let res = std::panic::AssertUnwindSafe(srv.call(req_for_service))
                .catch_unwind()
                .await;

            match res {
                Ok(Ok(response)) => Ok(response.map_into_left_body()),
                Ok(Err(e)) => {
                    error!("❌ Request error: {:?}", e);
                    Err(e)
                }
                Err(panic) => {
                    error!("💥 Panic caught: {:?}", panic);

                    let response = HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Internal Server Error",
                        "message": "An unexpected error occurred"
                    }));

                    Ok(ServiceResponse::new(
                        req_parts,
                        response.map_into_right_body(),
                    ))
                }
            }
        })
    }
}

// ─────────────────────────────
// 🪵 Request logger middleware
// ─────────────────────────────
pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerService {
            service: Rc::new(service),
        })
    }
}

pub struct RequestLoggerService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        let srv = self.service.clone();

        Box::pin(async move {
            let result = srv.call(req).await;
            let duration = start.elapsed();

            match &result {
                Ok(res) => {
                    let status = res.status().as_u16();
                    if status >= 500 {
                        Logger::error(&format!(
                            "❌ {} {} -> {} ({} ms)",
                            method,
                            path,
                            status,
                            duration.as_millis()
                        ));
                    } else if status >= 400 {
                        Logger::warn(&format!(
                            "⚠️ {} {} -> {} ({} ms)",
                            method,
                            path,
                            status,
                            duration.as_millis()
                        ));
                    } else {
                        Logger::info(&format!(
                            "✅ {} {} -> {} ({} ms)",
                            method,
                            path,
                            status,
                            duration.as_millis()
                        ));
                    }
                }
                Err(e) => {
                    Logger::error(&format!(
                        "❌ {} {} -> error: {:?} ({} ms)",
                        method,
                        path,
                        e,
                        duration.as_millis()
                    ));
                }
            }

            // convert response to EitherBody
            match result {
                Ok(res) => Ok(res.map_into_left_body()),
                Err(e) => Err(e),
            }
        })
    }
}
