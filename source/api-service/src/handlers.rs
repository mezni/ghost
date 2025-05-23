use crate::models::{Country, NewCountry};
use crate::repositories::CountryRepository;
use actix_web::{HttpResponse, Responder, get, post, web};
use deadpool_postgres::Pool;

#[get("/countries")]
pub async fn list_countries(pool: web::Data<Pool>) -> impl Responder {
    match CountryRepository::list(&pool).await {
        Ok(countries) => HttpResponse::Ok().json(countries),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {e}")),
    }
}

#[post("/countries")]
pub async fn add_country(
    pool: web::Data<Pool>,
    new_country: web::Json<NewCountry>,
) -> impl Responder {
    match CountryRepository::insert(&pool, &new_country).await {
        Ok(country) => HttpResponse::Created().json(country),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {e}")),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(list_countries)
            .service(add_country),
    );
}
