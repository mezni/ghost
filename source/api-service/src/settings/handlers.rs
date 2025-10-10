// settings/handlers.rs
use crate::core::errors::AppError;
use crate::settings::models::{CreateCountry, UpdateCountry};
use crate::settings::services::CountryService;
use actix_web::{HttpResponse, Scope, delete, get, post, put, web};
use deadpool_postgres::Pool;

pub fn scope() -> Scope {
    web::scope("/countries")
        .service(get_all_countries)
        .service(get_country_by_id)
        .service(create_country)
        .service(update_country)
        .service(delete_country)
}

/// Get all countries
#[get("")]
async fn get_all_countries(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let countries = CountryService::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(countries))
}

/// Get a country by ID
#[get("/{id}")]
async fn get_country_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let country = CountryService::get_by_id(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(country))
}

/// Create a new country
#[post("")]
async fn create_country(
    pool: web::Data<Pool>,
    input: web::Json<CreateCountry>,
) -> Result<HttpResponse, AppError> {
    let country = CountryService::create(&pool, input.into_inner()).await?;
    Ok(HttpResponse::Created().json(country))
}

/// Update an existing country
#[put("/{id}")]
async fn update_country(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    input: web::Json<UpdateCountry>,
) -> Result<HttpResponse, AppError> {
    let updated = CountryService::update(&pool, id.into_inner(), input.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/// Delete a country
#[delete("/{id}")]
async fn delete_country(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    CountryService::delete(&pool, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
