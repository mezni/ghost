use actix_web::{web, App, HttpServer};
use deadpool_postgres::Pool;

mod dtos;
mod errors;
mod handlers;
mod repositories;
mod db;

use crate::errors::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool: Pool = db::create_pool();

    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::init_routes)
    })
    .bind(("127.0.0.1", 8080))
    .map_err(|e| AppError::InternalError(format!("Bind error: {}", e)))?
    .run()
    .await
    .map_err(|e| AppError::InternalError(format!("Server run error: {}", e)))
}
