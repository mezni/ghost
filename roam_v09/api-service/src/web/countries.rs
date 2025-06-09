use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::app::countries::CountryService;
use crate::domain::countries::Country;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
struct CreateCountryDTO {
    name: String,
    code: String,
    created_by: String,
}

#[post("/countries")]
async fn create_country(
    service: web::Data<CountryService<impl crate::domain::countries::CountryRepository>>,
    payload: web::Json<CreateCountryDTO>,
) -> impl Responder {
    let result = service
        .create_country(
            payload.name.clone(),
            payload.code.clone(),
            payload.created_by.clone(),
        )
        .await;

    match result {
        Ok(country) => HttpResponse::Created().json(country),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}


