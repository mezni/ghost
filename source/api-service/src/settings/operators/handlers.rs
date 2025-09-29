use actix_web::{web, HttpResponse, Scope};
use deadpool_postgres::Pool;

use crate::core::errors::AppError;
use crate::settings::operators::models::{NewOperator, UpdateOperator, OperatorResponse};
use crate::settings::operators::services::OperatorService;

/// GET /operators
pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let operators = OperatorService::get_all(&pool).await?;
    // Convert to response DTOs
    let resp: Vec<OperatorResponse> = operators.into_iter().map(|op| op.into()).collect();
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /operators/{id}
pub async fn get_by_id(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    if let Some(op) = OperatorService::get_by_id(&pool, id.into_inner()).await? {
        let resp: OperatorResponse = op.into();
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body("Operator not found"))
    }
}

/// POST /operators
pub async fn create(pool: web::Data<Pool>, payload: web::Json<NewOperator>) -> Result<HttpResponse, AppError> {
    let new_op = payload.into_inner();
    let op = OperatorService::create(&pool, new_op).await?;
    let resp: OperatorResponse = op.into();
    Ok(HttpResponse::Created().json(resp))
}

/// PUT /operators/{id}
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    payload: web::Json<UpdateOperator>,
) -> Result<HttpResponse, AppError> {
    let updated = OperatorService::update(&pool, id.into_inner(), payload.into_inner()).await?;
    let resp: OperatorResponse = updated.into();
    Ok(HttpResponse::Ok().json(resp))
}

/// DELETE /operators/{id}
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let deleted = OperatorService::delete(&pool, id.into_inner()).await?;
    if deleted == 0 {
        Ok(HttpResponse::NotFound().body("Operator not found"))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

/// Scope for /operators endpoints
pub fn scope() -> Scope {
    web::scope("/operators")
        .route("", web::get().to(get_all))
        .route("", web::post().to(create))
        .route("/{id}", web::get().to(get_by_id))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete))
}
