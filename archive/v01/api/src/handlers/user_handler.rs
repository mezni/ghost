use crate::errors::ApiError;
use crate::models::user::{NewUser, UpdateUser};
use crate::repositories::user_repository::UserRepository;
use actix_web::{web, HttpResponse};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    user_repo: web::Data<Arc<Mutex<UserRepository>>>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, ApiError> {
    let new_user = new_user.into_inner();
    new_user.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;
    
    let user = user_repo.lock().unwrap().create_user(new_user)?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn get_user(
    user_repo: web::Data<Arc<Mutex<UserRepository>>>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let user = user_repo.lock().unwrap().get_user_by_id(user_id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user(
    user_repo: web::Data<Arc<Mutex<UserRepository>>>,
    user_id: web::Path<Uuid>,
    update_data: web::Json<UpdateUser>,
) -> Result<HttpResponse, ApiError> {
    let update_data = update_data.into_inner();
    update_data.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;
    
    let user = user_repo.lock().unwrap().update_user(user_id.into_inner(), update_data)?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    user_repo: web::Data<Arc<Mutex<UserRepository>>>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    user_repo.lock().unwrap().delete_user(user_id.into_inner())?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn authenticate_user(
    user_repo: web::Data<Arc<Mutex<UserRepository>>>,
    credentials: web::Json<AuthRequest>,
) -> Result<HttpResponse, ApiError> {
    let user = user_repo.lock().unwrap().verify_password(
        &credentials.email, 
        &credentials.password
    )?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn get_user(
    user_repo: web::Data<UserRepository>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let user = user_repo.get_user_by_id(user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}