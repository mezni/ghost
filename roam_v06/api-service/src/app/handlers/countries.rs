// api/handlers/country_handler.rs
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::app::services::country_service::CountryService;
use crate::domain::entities::country::Country;
use crate::errors::AppError;

#[get("/countries")]
async fn get_countries<R: CountryRepository + Send + Sync>(
    service: web::Data<CountryService<R>>,
) -> Result<impl Responder, AppError> {
    let countries = service.list_countries().await?;
    Ok(HttpResponse::Ok().json(countries))
}
