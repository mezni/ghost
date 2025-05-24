use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use crate::dtos::*;
use crate::repositories::*;
use crate::errors::AppError;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/countries", web::post().to(create_country))
            .route("/countries", web::get().to(list_countries))
            .route("/operators", web::post().to(create_operator))
            .route("/operators", web::get().to(list_operators))
            .route("/plans", web::post().to(create_plan))
            .route("/plans", web::get().to(list_plans))
    );
}

async fn create_country(pool: web::Data<Pool>, item: web::Json<CreateCountryDto>) -> Result<impl Responder, AppError> {
    let client = pool.get().await?;
    let result = insert_country(&client, &item).await?;
    Ok(web::Json(result))
}

async fn list_countries(pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client = pool.get().await?;
    let result = get_countries(&client).await?;
    Ok(web::Json(result))
}

async fn create_operator(pool: web::Data<Pool>, item: web::Json<CreateOperatorDto>) -> Result<impl Responder, AppError> {
    let client = pool.get().await?;
    let result = insert_operator(&client, &item).await?;
    Ok(web::Json(result))
}

async fn list_operators(pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client = pool.get().await?;
    let result = get_operators(&client).await?;
    Ok(web::Json(result))
}

async fn create_plan(pool: web::Data<Pool>, item: web::Json<CreatePlanDto>) -> Result<impl Responder, AppError> {
    let client = pool.get().await?;
    let result = insert_plan(&client, &item).await?;
    Ok(web::Json(result))
}

async fn list_plans(pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client = pool.get().await?;
    let result = get_plans(&client).await?;
    Ok(web::Json(result))
}
