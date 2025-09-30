use actix_web::{HttpResponse, Scope, web};
use deadpool_postgres::Pool;

use crate::core::errors::AppError;
use crate::catalog::operators::models::{NewOperator, OperatorResponse, UpdateOperator};
use crate::catalog::operators::services::OperatorService;

/// GET /operators
pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let operators = OperatorService::get_all(&pool).await?;
    let resp: Vec<OperatorResponse> = operators.into_iter().map(|op| op.into()).collect();
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /operators/{id}
pub async fn get_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if let Some(op) = OperatorService::get_by_id(&pool, id.into_inner()).await? {
        let resp: OperatorResponse = op.into();
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body("Operator not found"))
    }
}

/// GET /operators/by-country/{country_id}
pub async fn get_by_country_id(
    pool: web::Data<Pool>,
    country_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let ops = OperatorService::get_by_country_id(&pool, country_id.into_inner()).await?;
    let resp: Vec<OperatorResponse> = ops.into_iter().map(|op| op.into()).collect();
    Ok(HttpResponse::Ok().json(resp))
}

/// POST /operators
pub async fn create(
    pool: web::Data<Pool>,
    payload: web::Json<NewOperator>,
) -> Result<HttpResponse, AppError> {
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
    match OperatorService::update(&pool, id.into_inner(), payload.into_inner()).await? {
        Some(updated) => {
            let resp: OperatorResponse = updated.into();
            Ok(HttpResponse::Ok().json(resp))
        }
        None => Ok(HttpResponse::NotFound().body("Operator not found")),
    }
}

/// DELETE /operators/{id}
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    OperatorService::delete(&pool, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// Scope for /operators endpoints
pub fn scope() -> Scope {
    web::scope("/operators")
        .route("", web::get().to(get_all))
        .route("", web::post().to(create))
        .route("/{id}", web::get().to(get_by_id))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete))
        .route("/by-country/{country_id}", web::get().to(get_by_country_id))
}
