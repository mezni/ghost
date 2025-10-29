use actix_web::{web, HttpResponse};

use crate::user_service::UserService;
use crate::user_model::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::core::error::Result;

pub async fn create_user(
    user_service: web::Data<UserService>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    let user = user_service.create_user(user_data.into_inner()).await?;
    Ok(HttpResponse::Created().json(UserResponse::from(user)))
}

pub async fn get_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<String>,
) -> Result<HttpResponse> {
    let user = user_service.get_user_by_id(&user_id).await?;
    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

pub async fn get_users(
    user_service: web::Data<UserService>,
) -> Result<HttpResponse> {
    let users = user_service.get_all_users().await?;
    let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    Ok(HttpResponse::Ok().json(responses))
}

pub async fn update_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<String>,
    user_data: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    let user = user_service.update_user(&user_id, user_data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

pub async fn delete_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<String>,
) -> Result<HttpResponse> {
    user_service.delete_user(&user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_user_by_username(
    user_service: web::Data<UserService>,
    username: web::Path<String>,
) -> Result<HttpResponse> {
    let user = user_service.get_user_by_username(&username).await?;
    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}