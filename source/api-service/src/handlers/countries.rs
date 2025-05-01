use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use deadpool_postgres::Client;
use crate::models::country::Country;

#[derive(Deserialize)]
pub struct CountryPayload {
    pub name: String,
    pub iso: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub query: String,
}

pub async fn list_countries(db: web::Data<Client>) -> impl Responder {
    // Dummy placeholder
    HttpResponse::Ok().body("List of countries")
}

pub async fn create_country(
    db: web::Data<Client>,
    payload: web::Json<CountryPayload>,
) -> impl Responder {
    HttpResponse::Created().body(format!("Created country: {} ({})", payload.name, payload.iso))
}

pub async fn update_country(
    db: web::Data<Client>,
    id: web::Path<i32>,
    payload: web::Json<CountryPayload>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Updated country ID {}: {} ({})", id, payload.name, payload.iso))
}

pub async fn delete_country(
    db: web::Data<Client>,
    id: web::Path<i32>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Deleted country ID {}", id))
}

pub async fn search_countries(
    db: web::Data<Client>,
    query: web::Query<SearchQuery>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Search for: {}", query.query))
}
