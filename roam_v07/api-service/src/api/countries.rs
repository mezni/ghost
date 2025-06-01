use actix_web::{web, HttpResponse, Responder};
use crate::application::country_service::{CountryService, CreateCountryCommand, UpdateCountryCommand};
use crate::infra::postgres::countries::PgCountryRepository;

#[derive(Debug, serde::Deserialize)]
pub struct CreateCountryRequest {
    pub name: String,
    pub code: String,
    #[serde(default)]
    pub created_by: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateCountryRequest {
    pub name: Option<String>,
    pub code: Option<String>,
    pub updated_by: String,
}

pub async fn create_country(
    req: web::Json<CreateCountryRequest>,
    country_service: web::Data<CountryService<PgCountryRepository>>,
) -> Result<HttpResponse, crate::infra::error::AppError> {
    let command = CreateCountryCommand {
        name: req.name.clone(),
        code: req.code.clone(),
        created_by: req.created_by.clone(),
    };
    let country = country_service.create_country(command).await?;
    Ok(HttpResponse::Created().json(country))
}

pub async fn get_country_by_id(
    path: web::Path<i32>,
    country_service: web::Data<CountryService<PgCountryRepository>>,
) -> Result<HttpResponse, crate::infra::error::AppError> {
    let id = path.into_inner();
    let country = country_service.get_country(id).await?;
    Ok(HttpResponse::Ok().json(country))
}

pub async fn get_all_countries(
    country_service: web::Data<CountryService<PgCountryRepository>>,
) -> Result<HttpResponse, crate::infra::error::AppError> {
    let countries = country_service.get_all_countries().await?;
    Ok(HttpResponse::Ok().json(countries))
}

pub async fn update_country(
    path: web::Path<i32>,
    req: web::Json<UpdateCountryRequest>,
    country_service: web::Data<CountryService<PgCountryRepository>>,
) -> Result<HttpResponse, crate::infra::error::AppError> {
    let id = path.into_inner();
    let command = UpdateCountryCommand {
        id,
        name: req.name.clone(),
        code: req.code.clone(),
        updated_by: req.updated_by.clone(),
    };
    let country = country_service.update_country(command).await?;
    Ok(HttpResponse::Ok().json(country))
}

pub async fn delete_country(
    path: web::Path<i32>,
    country_service: web::Data<CountryService<PgCountryRepository>>,
) -> Result<HttpResponse, crate::infra::error::AppError> {
    let id = path.into_inner();
    country_service.delete_country(id).await?;
    Ok(HttpResponse::NoContent().finish())
}