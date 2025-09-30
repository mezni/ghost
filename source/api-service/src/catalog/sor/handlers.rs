use actix_web::{HttpResponse, Scope, web};
use deadpool_postgres::Pool;

use crate::catalog::sor::models::{NewSorPlan, SorPlanResponse, UpdateSorPlan};
use crate::catalog::sor::services::SorPlanService;
use crate::core::errors::AppError;

/// GET /sor
pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let plans = SorPlanService::get_all(&pool).await?;
    let resp: Vec<SorPlanResponse> = SorPlanService::to_response_vec(plans);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /sor/{id}
pub async fn get_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if let Some(plan) = SorPlanService::get_by_id(&pool, id.into_inner()).await? {
        let resp: SorPlanResponse = SorPlanService::to_response(plan);
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body("SOR Plan not found"))
    }
}

/// POST /sor
pub async fn create(
    pool: web::Data<Pool>,
    payload: web::Json<NewSorPlan>,
) -> Result<HttpResponse, AppError> {
    let new_plan = payload.into_inner();
    let plan = SorPlanService::create(&pool, new_plan).await?;
    let resp: SorPlanResponse = SorPlanService::to_response(plan);
    Ok(HttpResponse::Created().json(resp))
}

/// PUT /sor/{id}
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    payload: web::Json<UpdateSorPlan>,
) -> Result<HttpResponse, AppError> {
    let updated = SorPlanService::update(&pool, id.into_inner(), payload.into_inner()).await?;
    let resp: SorPlanResponse = SorPlanService::to_response(updated);
    Ok(HttpResponse::Ok().json(resp))
}

/// DELETE /sor/{id}
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let deleted = SorPlanService::delete(&pool, id.into_inner()).await?;
    if deleted == 0 {
        Ok(HttpResponse::NotFound().body("SOR Plan not found"))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

/// Scope for /sor endpoints
pub fn scope() -> Scope {
    web::scope("/sor")
        .route("", web::get().to(get_all))
        .route("", web::post().to(create))
        .route("/{id}", web::get().to(get_by_id))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete))
}
