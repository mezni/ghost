mod core;
mod settings;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use core::errors::AppError;
use core::logger::Logger;

const SERVER_IP: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info(&format!(
        "Starting API server on {}:{}",
        SERVER_IP, SERVER_PORT
    ));

    let pool = core::db::Db::create_pool();

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(|origin, _| {
                origin.as_bytes().starts_with(b"http://localhost")
                    || origin.as_bytes().starts_with(b"http://127.0.0.1")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1")
                    .service(core::health::health)
                    .service(settings::countries::handlers::scope())
                    .service(settings::operators::handlers::scope()),
            )
    })
    .bind((SERVER_IP, SERVER_PORT))?
    .run()
    .await?;

    Ok(())
}
