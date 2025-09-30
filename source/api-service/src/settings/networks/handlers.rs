use actix_web::{HttpResponse, Scope, web};
use deadpool_postgres::Pool;

use crate::core::errors::AppError;
use crate::settings::networks::models::{NetworkResponse, NewNetwork, UpdateNetwork};
use crate::settings::networks::services::NetworkService;

/// GET /networks
pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let networks = NetworkService::get_all(&pool).await?;
    // Convert to response DTOs
    let resp: Vec<NetworkResponse> = networks.into_iter().map(|n| n.into()).collect();
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /networks/{id}
pub async fn get_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if let Some(net) = NetworkService::get_by_id(&pool, id.into_inner()).await? {
        let resp: NetworkResponse = net.into();
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body("Network not found"))
    }
}

/// POST /networks
pub async fn create(
    pool: web::Data<Pool>,
    payload: web::Json<NewNetwork>,
) -> Result<HttpResponse, AppError> {
    let new_net = payload.into_inner();
    let net = NetworkService::create(&pool, new_net).await?;
    let resp: NetworkResponse = net.into();
    Ok(HttpResponse::Created().json(resp))
}

/// PUT /networks/{id}
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    payload: web::Json<UpdateNetwork>,
) -> Result<HttpResponse, AppError> {
    let updated = NetworkService::update(&pool, id.into_inner(), payload.into_inner()).await?;
    let resp: NetworkResponse = updated.into();
    Ok(HttpResponse::Ok().json(resp))
}

/// DELETE /networks/{id}
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let deleted = NetworkService::delete(&pool, id.into_inner()).await?;
    if deleted == 0 {
        Ok(HttpResponse::NotFound().body("Network not found"))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

/// Scope for /networks endpoints
pub fn scope() -> Scope {
    web::scope("/networks")
        .route("", web::get().to(get_all))
        .route("", web::post().to(create))
        .route("/{id}", web::get().to(get_by_id))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete))
}
