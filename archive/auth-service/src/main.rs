mod controller;
mod model;

use crate::controller::{get_info_handler, login_handler};
use axum::Router;
use axum::routing::{get, post};
use tokio;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", post(login_handler))
        .route("/info", post(get_info_handler));

    let listener = tokio::net::Tcplistener::bind("0.0.0.0:3000").await.unwrap;

    println!("Hello, world!");
    axum::serve(listener, app).await.unwrap;
}
