use actix_web::{HttpResponse, Scope, web};
use deadpool_postgres::Pool;

use crate::catalog::countries::models::{CountryResponse, NewCountry, UpdateCountry};
use crate::catalog::countries::services::CountryService;
use crate::core::errors::AppError;

/// GET /countries
pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let countries = CountryService::get_all(&pool).await?;
    let resp: Vec<CountryResponse> = CountryService::to_response_vec(countries);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /countries/{id}
pub async fn get_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if let Some(country) = CountryService::get_by_id(&pool, id.into_inner()).await? {
        let resp: CountryResponse = CountryService::to_response(country);
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body("Country not found"))
    }
}

/// GET /countries/by_name/{name}
pub async fn get_by_name(
    pool: web::Data<Pool>,
    name: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    if let Some(country) = CountryService::get_by_name(&pool, &name.into_inner()).await? {
        let resp: CountryResponse = CountryService::to_response(country);
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body("Country not found"))
    }
}

/// POST /countries
pub async fn create(
    pool: web::Data<Pool>,
    payload: web::Json<NewCountry>,
) -> Result<HttpResponse, AppError> {
    let new_country = payload.into_inner();
    let country = CountryService::create(&pool, new_country).await?;
    let resp: CountryResponse = CountryService::to_response(country);
    Ok(HttpResponse::Created().json(resp))
}

/// PUT /countries/{id}
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    payload: web::Json<UpdateCountry>,
) -> Result<HttpResponse, AppError> {
    let updated = CountryService::update(&pool, id.into_inner(), payload.into_inner()).await?;
    let resp: CountryResponse = CountryService::to_response(updated);
    Ok(HttpResponse::Ok().json(resp))
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
        .route("/by_name/{name}", web::get().to(get_by_name))
}
