mod common;
mod api;
mod application;
mod domain;
mod infra;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

use crate::infra::db::establish_connection_pool;
use crate::infra::postgres::countries::PgCountryRepository;
use crate::infra::postgres::operators::PgOperatorRepository;
use crate::application::country_service::CountryService;
use crate::application::operator_service::OperatorService;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok(); // Load .env file
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file or environment");

    let pool: PgPool = establish_connection_pool(&database_url)
        .await
        .expect("Failed to create PostgreSQL connection pool");

    log::info!("Successfully connected to PostgreSQL");

    // Initialize repositories
    let country_repo = PgCountryRepository::new(pool.clone());
    let operator_repo = PgOperatorRepository::new(pool.clone());

    // Initialize application services
    let country_service = CountryService::new(country_repo);
    let operator_service = OperatorService::new(operator_repo);

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    log::info!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(country_service.clone())) // Share services across workers
            .app_data(web::Data::new(operator_service.clone())) // Share services across workers
            .service(
                web::scope("/api/v1/countries")
                    .route("", web::post().to(api::country_handlers::create_country))
                    .route("", web::get().to(api::country_handlers::get_all_countries))
                    .route("/{id}", web::get().to(api::country_handlers::get_country_by_id))
                    .route("/{id}", web::put().to(api::country_handlers::update_country))
                    .route("/{id}", web::delete().to(api::country_handlers::delete_country)),
            )
            .service(
                web::scope("/api/v1/operators")
                    .route("", web::post().to(api::operator_handlers::create_operator))
                    .route("/{id}", web::get().to(api::operator_handlers::get_operator_by_id))
            )
    })
    .bind(&server_address)?
    .run()
    .await?;

    Ok(())
}