mod app;
mod domain;
mod infra;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::sync::Arc;

use app::countries::CountryService;
use dotenvy::dotenv;
use infra::store::countries::PgCountryRepository;
use sqlx::postgres::PgPoolOptions;
use std::env;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let repo = Arc::new(PgCountryRepository::new(Arc::new(pool)));
    let service = web::Data::new(CountryService::new(repo.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(service.clone())
            .route("/health", web::get().to(health))
        // TODO: add country endpoints
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
