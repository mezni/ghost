use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use crate::models::{Country, NewCountry};
use crate::repositories::CountryRepository;

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
    new_country: web::Json<NewCountry>
) -> impl Responder {
    match CountryRepository::insert(&pool, &new_country).await {
        Ok(country) => HttpResponse::Created().json(country),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {e}")),
    }
}

pub fn init_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_country)
       .service(list_countries);
}



#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use deadpool_postgres::Pool;
    use std::sync::Once;

    // Ensure logger and dotenv are only initialized once
    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            let _ = dotenvy::dotenv();
            crate::logger::Logger::init();
        });
    }

    async fn get_test_pool() -> Pool {
        setup();
        crate::db::get_pool().await.expect("Failed to get DB pool")
    }

    #[actix_web::test]
    async fn test_list_countries() {
        let pool = get_test_pool().await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(list_countries)
        ).await;

        let req = test::TestRequest::get().uri("/countries").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

    }
}
