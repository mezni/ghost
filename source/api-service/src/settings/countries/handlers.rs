use actix_web::{web, HttpResponse, Scope};
use deadpool_postgres::Pool;

use crate::core::errors::AppError;
use crate::settings::countries::models::{NewCountry, UpdateCountry};
use crate::settings::countries::services::CountryService;

/// GET /countries
pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let countries = CountryService::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(countries))
}

/// GET /countries/{id}
pub async fn get_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    match CountryService::get_by_id(&pool, id.into_inner()).await? {
        Some(country) => Ok(HttpResponse::Ok().json(country)),
        None => Ok(HttpResponse::NotFound().body("Country not found")),
    }
}

/// POST /countries
pub async fn create(pool: web::Data<Pool>, payload: web::Json<NewCountry>) -> Result<HttpResponse, AppError> {
    let new_country = payload.into_inner();
    let country = CountryService::create(&pool, new_country).await?;
    Ok(HttpResponse::Created().json(country))
}

/// PUT /countries/{id}
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    payload: web::Json<UpdateCountry>,
) -> Result<HttpResponse, AppError> {
    let updated = CountryService::update(&pool, id.into_inner(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/// DELETE /countries/{id}
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let deleted = CountryService::delete(&pool, id.into_inner()).await?;
    if deleted == 0 {
        Ok(HttpResponse::NotFound().body("Country not found"))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

/// Scope for /countries endpoints
pub fn scope() -> Scope {
    web::scope("/countries")
        .route("", web::get().to(get_all))
        .route("", web::post().to(create))
        .route("/{id}", web::get().to(get_by_id))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete))
}
