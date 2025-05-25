// main.rs
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use crate::handlers::{login, register, logout};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").route(web::post().to(logout)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}