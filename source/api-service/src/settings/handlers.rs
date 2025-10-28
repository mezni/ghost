use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::settings::models::{Country, CreateCountry, UpdateCountry};
use crate::settings::services::CountryService;
use crate::core::errors::AppError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/countries")
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_by_id))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete))
    );
}

async fn get_all(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let countries = CountryService::get_all(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(countries))
}

async fn get_by_id(pool: web::Data<PgPool>, path: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let country = CountryService::get_by_id(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(country))
}

async fn create(pool: web::Data<PgPool>, body: web::Json<CreateCountry>) -> Result<HttpResponse, AppError> {
    let country = CountryService::create(pool.get_ref(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(country))
}

async fn update(pool: web::Data<PgPool>, path: web::Path<i32>, body: web::Json<UpdateCountry>) -> Result<HttpResponse, AppError> {
    let country = CountryService::update(pool.get_ref(), path.into_inner(), body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(country))
}

async fn delete(pool: web::Data<PgPool>, path: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let deleted = CountryService::delete(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(deleted))
}
