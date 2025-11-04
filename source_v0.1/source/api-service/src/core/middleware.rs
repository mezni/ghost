use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::{
    rc::Rc,
    task::{Context, Poll},
};
use tracing::{error, info};

/// ErrorMiddleware handles panics and errors, returning a graceful error response.
pub struct ErrorMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            service.call(req).await.map_err(|e| {
                error!("Error processing request: {}", e);
                e
            })
        })
    }
}

/// RequestLogger logs request details and timing.
pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        let service = self.service.clone();

        Box::pin(async move {
            let result = service.call(req).await;
            let duration = start.elapsed();
            log_request(method, path, &result, duration);
            result
        })
    }
}

fn log_request<B>(
    method: actix_web::http::Method,
    path: String,
    result: &Result<ServiceResponse<B>, Error>,
    duration: std::time::Duration,
) {
    match result {
        Ok(response) => info!(
            "{} {} -> {} ({}ms)",
            method,
            path,
            response.status(),
            duration.as_millis()
        ),
        Err(e) => error!("{} {} -> Error: {}", method, path, e),
    }
}
