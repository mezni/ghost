use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenvy::dotenv;
use std::env;

mod db;
mod models;
mod repo;
mod handlers;
mod errors;

use db::create_pool;
use repo::UserRepo;
use handlers::init_routes;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_addr = env::var("SERVICE_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let pool = create_pool(&database_url).await?;
    let repo = web::Data::new(UserRepo::new(pool));

    log::info!("Starting server at {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(repo.clone())
            .configure(init_routes)
    })
    .bind(bind_addr)?
    .run()
    .await?;

    Ok(())
}
