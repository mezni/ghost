mod config;
mod routes;
mod handlers;
mod models;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use actix_cors::Cors;
use config::get_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let pool = get_db_pool().expect("Failed to create PostgreSQL pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .configure(routes::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
