use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::repo::UserRepo;
use crate::models::{CreateUser, UpdateUser};
use uuid::Uuid;
use crate::errors::AppError;
use serde_json::json;
use std::env;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("", web::get().to(list_users))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
            .route("/me", web::get().to(get_me))
    );
}

async fn create_user(repo: web::Data<UserRepo>, payload: web::Json<CreateUser>) -> Result<impl Responder, AppError> {
    let u = repo.create(payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(u))
}

async fn list_users(repo: web::Data<UserRepo>, query: web::Query<Pagination>) -> Result<impl Responder, AppError> {
    let users = repo.list(query.limit.unwrap_or(25), query.offset.unwrap_or(0)).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[derive(serde::Deserialize)]
struct Pagination {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn get_user(repo: web::Data<UserRepo>, path: web::Path<Uuid>) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let user = repo.get(id).await?;
    Ok(HttpResponse::Ok().json(user))
}

async fn update_user(repo: web::Data<UserRepo>, path: web::Path<Uuid>, payload: web::Json<UpdateUser>) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let updated = repo.update(id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

async fn delete_user(repo: web::Data<UserRepo>, path: web::Path<Uuid>) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    repo.delete(id).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// Get the current user from Keycloak token (calls userinfo endpoint)
async fn get_me(repo: web::Data<UserRepo>, req: HttpRequest) -> Result<impl Responder, AppError> {
    // Extract bearer token
    let header = req.headers().get("authorization").ok_or(AppError::AuthError("Missing authorization header".into()))?;
    let header_str = header.to_str().map_err(|_| AppError::AuthError("Invalid authorization header".into()))?;
    if !header_str.to_lowercase().starts_with("bearer ") {
        return Err(AppError::AuthError("Missing bearer token".into()));
    }
    let token = header_str[7..].trim();

    // Call Keycloak userinfo endpoint
    let kc_url = env::var("KEYCLOAK_URL").map_err(|_| AppError::AuthError("KEYCLOAK_URL not configured".into()))?;
    let realm = env::var("KEYCLOAK_REALM").map_err(|_| AppError::AuthError("KEYCLOAK_REALM not configured".into()))?;
    let userinfo = format!("{}/realms/{}/protocol/openid-connect/userinfo", kc_url.trim_end_matches('/'), realm);

    let client = reqwest::Client::new();
    let resp = client.get(&userinfo)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| AppError::AuthError(format!("failed to call userinfo: {}", e)))?;

    if !resp.status().is_success() {
        return Err(AppError::AuthError(format!("userinfo returned non-200: {}", resp.status())));
    }

    let j: serde_json::Value = resp.json().await.map_err(|e| AppError::AuthError(format!("invalid userinfo response: {}", e)))?;
    let sub = j.get("sub").and_then(|v| v.as_str()).ok_or(AppError::AuthError("userinfo missing sub".into()))?;

    // lookup user by keycloak id
    let user = repo.get_by_keycloak_id(sub).await?;
    Ok(HttpResponse::Ok().json(user))
}
