use crate::core::errors::AppError;
use crate::core::logger::Logger;
use crate::settings::models::{CreateCountry, CreateOperator, CreateNetwork, UpdateCountry, UpdateOperator, UpdateNetwork};
use crate::settings::services::{CountryService, OperatorService, NetworkService};
use actix_web::{HttpResponse, Scope, delete, get, post, put, web};
use deadpool_postgres::Pool;

pub fn scope() -> Scope {
    web::scope("/settings") // Base scope
        .service(test_settings)
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
        // Networks
        .service(get_all_networks)
        .service(get_network_by_id)
        .service(create_network)
        .service(update_network)
        .service(delete_network)
}

#[get("/test")]
async fn test_settings(_pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    Logger::info("Testing settings endpoint");

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Settings test endpoint is working!",
        "status": "success",
    })))
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
    let mut update_data = input.into_inner();
    update_data.operator_id = id.into_inner();
    let updated = OperatorService::update(&pool, update_data).await?;

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

// -------------------------
// Networks Handlers
// -------------------------

#[get("/networks")]
async fn get_all_networks(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let networks = NetworkService::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(networks))
}

#[get("/networks/{id}")]
async fn get_network_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let network = NetworkService::get_by_id(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(network))
}

#[post("/networks")]
async fn create_network(
    pool: web::Data<Pool>,
    input: web::Json<CreateNetwork>,
) -> Result<HttpResponse, AppError> {
    let network = NetworkService::create(&pool, input.into_inner()).await?;
    Ok(HttpResponse::Created().json(network))
}

#[put("/networks/{id}")]
async fn update_network(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    input: web::Json<UpdateNetwork>,
) -> Result<HttpResponse, AppError> {
    let mut update_data = input.into_inner();
    update_data.network_id = id.into_inner();
    let updated = NetworkService::update(&pool, update_data).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/networks/{id}")]
async fn delete_network(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    NetworkService::delete(&pool, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
