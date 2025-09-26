// users/handlers.rs
use crate::core::errors::AppError;
use crate::users::models::{CreateUser, User, UserDTO};
use crate::users::services::UserService;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use deadpool_postgres::Pool;
use serde::Deserialize;
use uuid::Uuid;

/// POST /api/v1/register
#[post("/register")]
pub async fn register(
    pool: web::Data<Pool>,
    payload: web::Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    let user = UserService::register(&pool, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// POST /api/v1/login
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<Pool>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let token = UserService::login(&pool, &payload.email, &payload.password).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}

/// GET /api/v1/users/{id}
#[get("/users/{id}")]
pub async fn get_user(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_id = path.into_inner();
    let user = UserService::get_user(&pool, user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// PUT /api/v1/users/{id}
#[put("/users/{id}")]
pub async fn update_user(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
    payload: web::Json<User>,
) -> Result<impl Responder, AppError> {
    let mut user = payload.into_inner();
    user.id = path.into_inner(); // ensure ID matches URL
    let updated = UserService::update_user(&pool, user).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/// DELETE /api/v1/users/{id}
#[delete("/users/{id}")]
pub async fn delete_user(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_id = path.into_inner();
    UserService::delete_user(&pool, user_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "deleted" })))
}
