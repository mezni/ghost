use actix_web::{web, App, HttpServer};
mod config;
mod db;
mod handlers;
mod jwt;
mod models;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let pool = db::init_db().await.expect("DB init failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
