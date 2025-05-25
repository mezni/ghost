// handlers.rs
use crate::models::{User, LoginRequest, RegisterRequest};
use crate::repositories::{UserRepository, SessionRepository};
use crate::utils::hash_password;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn register(
    user_repo: web::Data<UserRepository>,
    register_request: web::Json<RegisterRequest>,
) -> impl Responder {
    let user = User {
        id: 0,
        username: register_request.username.clone(),
        email: register_request.email.clone(),
        password_hash: hash_password(&register_request.password),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    match user_repo.create_user(&user).await {
        Ok(_) => HttpResponse::Created().json(json!({ "message": "User created successfully" })),
        Err(_) => HttpResponse::InternalServerError().json(json!({ "message": "Failed to create user" })),
    }
}

pub async fn login(
    user_repo: web::Data<UserRepository>,
    session_repo: web::Data<SessionRepository>,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    match user_repo.get_user_by_username(&login_request.username).await {
        Ok(user) => {
            if user.password_hash == hash_password(&login_request.password) {
                let session = session_repo.create_session(&user.id).await.unwrap();
                HttpResponse::Ok().json(json!({ "token": session.token }))
            } else {
                HttpResponse::Unauthorized().json(json!({ "message": "Invalid credentials" }))
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(json!({ "message": "Invalid credentials" })),
    }
}

pub async fn logout(
    session_repo: web::Data<SessionRepository>,
    token: web::Header<actix_web::http::header::Authorization>,
) -> impl Responder {
    match session_repo.delete_session(token.token()).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Logged out successfully" })),
        Err(_) => HttpResponse::InternalServerError().json(json!({ "message": "Failed to log out" })),
    }
}