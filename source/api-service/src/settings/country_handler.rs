use crate::core::errors::AppError;
use crate::settings::country_model::{Country, NewCountry, UpdateCountry};
use crate::settings::country_service::CountryService;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use deadpool_postgres::Pool;

/// POST /api/v1/countries
#[post("/countries")]
pub async fn create_country(
    pool: web::Data<Pool>,
    payload: web::Json<NewCountry>,
) -> Result<impl Responder, AppError> {
    let country = CountryService::create(&pool, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(country))
}

/// GET /api/v1/countries
#[get("/countries")]
pub async fn get_all_countries(pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let countries = CountryService::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(countries))
}

/// GET /api/v1/countries/{id}
#[get("/countries/{id}")]
pub async fn get_country(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let country = CountryService::get_by_id(&pool, id).await?;
    Ok(HttpResponse::Ok().json(country))
}

/// PUT /api/v1/countries/{id}
#[put("/countries/{id}")]
pub async fn update_country(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    payload: web::Json<UpdateCountry>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let updated = CountryService::update(&pool, id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/// DELETE /api/v1/countries/{id}
#[delete("/countries/{id}")]
pub async fn delete_country(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    CountryService::delete(&pool, id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "deleted" })))
}

pub fn scope() -> actix_web::Scope {
    web::scope("")
        .service(create_country)
        .service(get_all_countries)
        .service(update_country)
        .service(get_country)
        .service(delete_country)
}
