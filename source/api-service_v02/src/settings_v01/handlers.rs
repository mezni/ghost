use crate::core::errors::AppError;
use crate::settings::models::{
    CreateCountry, UpdateCountry,
    CreateOperator, UpdateOperator,
};
use crate::settings::services::{CountryService, OperatorService};
use actix_web::{HttpResponse, Scope, delete, get, post, put, web};
use deadpool_postgres::Pool;

pub fn scope() -> Scope {
    web::scope("")
        // Countries
        .service(get_all_countries)
        .service(get_country_by_id)
        .service(create_country)
        .service(update_country)
        .service(delete_country)
        // Operators
        .service(get_all_operators)
        .service(get_operator_by_id)
        .service(create_operator)
        .service(update_operator)
        .service(delete_operator)
}

// -------------------------
// Countries Handlers
// -------------------------

#[get("/countries")]
async fn get_all_countries(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let countries = CountryService::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(countries))
}

#[get("/countries/{id}")]
async fn get_country_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let country = CountryService::get_by_id(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(country))
}

#[post("/countries")]
async fn create_country(
    pool: web::Data<Pool>,
    input: web::Json<CreateCountry>,
) -> Result<HttpResponse, AppError> {
    let country = CountryService::create(&pool, input.into_inner()).await?;
    Ok(HttpResponse::Created().json(country))
}

#[put("/countries/{id}")]
async fn update_country(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    input: web::Json<UpdateCountry>,
) -> Result<HttpResponse, AppError> {
    let updated = CountryService::update(&pool, id.into_inner(), input.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/countries/{id}")]
async fn delete_country(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    CountryService::delete(&pool, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

// -------------------------
// Operators Handlers
// -------------------------

#[get("/operators")]
async fn get_all_operators(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let operators = OperatorService::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(operators))
}

#[get("/operators/{id}")]
async fn get_operator_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let operator = OperatorService::get_by_id(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(operator))
}

#[post("/operators")]
async fn create_operator(
    pool: web::Data<Pool>,
    input: web::Json<CreateOperator>,
) -> Result<HttpResponse, AppError> {
    let operator = OperatorService::create(&pool, input.into_inner()).await?;
    Ok(HttpResponse::Created().json(operator))
}

#[put("/operators/{id}")]
async fn update_operator(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    input: web::Json<UpdateOperator>,
) -> Result<HttpResponse, AppError> {
    let updated = OperatorService::update(&pool, id.into_inner(), input.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/operators/{id}")]
async fn delete_operator(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    OperatorService::delete(&pool, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
