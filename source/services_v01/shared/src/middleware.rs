use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::header::<String>("authorization").and_then(|token: String| async move {
        if token.starts_with("Bearer ") {
            Ok(token)
        } else {
            Err(warp::reject::custom(crate::errors::AppError::Unauthorized))
        }
    })
}

pub fn cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["authorization", "content-type", "x-requested-with"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
}
