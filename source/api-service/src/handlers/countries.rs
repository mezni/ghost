use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use deadpool_postgres::Pool;
use crate::models::country::Country;
use log::{info, error};

#[derive(Deserialize)]
pub struct CountryPayload {
    pub name: String,
    pub iso: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub query: String,
}

pub async fn list_countries(pool: web::Data<Pool>) -> impl Responder {
    info!("Listing all countries");

    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to get client from pool: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let stmt = match client
        .prepare("SELECT country_id, name, iso FROM countries ORDER BY name")
        .await
    {
        Ok(s) => s,
        Err(e) => {
            error!("Prepare failed: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let rows = match client.query(&stmt, &[]).await {
        Ok(r) => r,
        Err(e) => {
            error!("Query failed: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let countries: Vec<Country> = rows
        .iter()
        .map(|row| Country {
            id: row.get(0),
            name: row.get(1),
            iso: row.get(2),
        })
        .collect();

    HttpResponse::Ok().json(countries)
}

pub async fn create_country(
    pool: web::Data<Pool>,
    payload: web::Json<CountryPayload>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Pool error: {}", e)),
    };

    let stmt = match client
        .prepare("INSERT INTO countries (name, iso) VALUES ($1, $2) RETURNING country_id")
        .await
    {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Prepare failed: {}", e)),
    };

    let row = match client.query_one(&stmt, &[&payload.name, &payload.iso]).await {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Insert failed: {}", e)),
    };

    let id: i32 = row.get(0);
    HttpResponse::Created().json(Country {
        id,
        name: payload.name.clone(),
        iso: payload.iso.clone(),
    })
}

pub async fn update_country(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    payload: web::Json<CountryPayload>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Pool error: {}", e)),
    };

    let stmt = match client
        .prepare("UPDATE countries SET name = $1, iso = $2 WHERE country_id = $3")
        .await
    {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Prepare failed: {}", e)),
    };

    if let Err(e) = client
        .execute(&stmt, &[&payload.name, &payload.iso, &id.into_inner()])
        .await
    {
        return HttpResponse::InternalServerError().body(format!("Update failed: {}", e));
    }

    HttpResponse::Ok().body("Country updated")
}

pub async fn delete_country(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Pool error: {}", e)),
    };

    let stmt = match client
        .prepare("DELETE FROM countries WHERE country_id = $1")
        .await
    {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Prepare failed: {}", e)),
    };

    if let Err(e) = client.execute(&stmt, &[&id.into_inner()]).await {
        return HttpResponse::InternalServerError().body(format!("Delete failed: {}", e));
    }

    HttpResponse::Ok().body("Country deleted")
}

pub async fn search_countries(
    pool: web::Data<Pool>,
    query: web::Query<SearchQuery>,
) -> impl Responder {
    let pattern = format!("%{}%", query.query);

    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Pool error: {}", e)),
    };

    let stmt = match client
        .prepare("SELECT country_id, name, iso FROM countries WHERE name ILIKE $1 OR iso ILIKE $1")
        .await
    {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Prepare failed: {}", e)),
    };

    let rows = match client.query(&stmt, &[&pattern]).await {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Query failed: {}", e)),
    };

    let results: Vec<Country> = rows
        .iter()
        .map(|row| Country {
            id: row.get(0),
            name: row.get(1),
            iso: row.get(2),
        })
        .collect();

    HttpResponse::Ok().json(results)
}
