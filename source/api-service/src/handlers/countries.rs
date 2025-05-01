use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize};
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
    let stmt = db.prepare("SELECT country_id, name, iso FROM countries ORDER BY name").await.unwrap();
    let rows = db.query(&stmt, &[]).await.unwrap();

    let countries: Vec<Country> = rows.iter().map(|row| Country {
        id: row.get(0),
        name: row.get(1),
        iso: row.get(2),
    }).collect();

    HttpResponse::Ok().json(countries)
}

pub async fn create_country(
    db: web::Data<Client>,
    payload: web::Json<CountryPayload>,
) -> impl Responder {
    let stmt = db.prepare("INSERT INTO countries (name, iso) VALUES ($1, $2) RETURNING id").await.unwrap();
    let row = db.query_one(&stmt, &[&payload.name, &payload.iso]).await.unwrap();

    let id: i32 = row.get(0);
    HttpResponse::Created().json(Country {
        id,
        name: payload.name.clone(),
        iso: payload.iso.clone(),
    })
}

pub async fn update_country(
    db: web::Data<Client>,
    id: web::Path<i32>,
    payload: web::Json<CountryPayload>,
) -> impl Responder {
    let stmt = db.prepare("UPDATE countries SET name = $1, iso = $2 WHERE country_id = $3").await.unwrap();
    let _ = db.execute(&stmt, &[&payload.name, &payload.iso, &id.into_inner()]).await.unwrap();


    HttpResponse::Ok().body("Country updated")
}

pub async fn delete_country(
    db: web::Data<Client>,
    id: web::Path<i32>,
) -> impl Responder {
    let stmt = db.prepare("DELETE FROM countries WHERE country_id = $1").await.unwrap();
    let _ = db.execute(&stmt, &[&id.into_inner()]).await.unwrap();


    HttpResponse::Ok().body("Country deleted")
}

pub async fn search_countries(
    db: web::Data<Client>,
    query: web::Query<SearchQuery>,
) -> impl Responder {
    let pattern = format!("%{}%", query.query);
    let stmt = db.prepare("SELECT country_id, name, iso FROM countries WHERE name ILIKE $1 OR iso ILIKE $1").await.unwrap();
    let rows = db.query(&stmt, &[&pattern]).await.unwrap();

    let results: Vec<Country> = rows.iter().map(|row| Country {
        id: row.get(0),
        name: row.get(1),
        iso: row.get(2),
    }).collect();

    HttpResponse::Ok().json(results)
}
