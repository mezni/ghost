use actix_web::{web, HttpResponse};

use crate::keycloak_service::KeycloakService;
use crate::auth_model::LoginRequest;
use crate::core::error::Result;

pub async fn login(
    keycloak_service: web::Data<KeycloakService>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let tokens = keycloak_service.login(&login_data.username, &login_data.password).await?;
    Ok(HttpResponse::Ok().json(tokens))
}

pub async fn refresh_token(
    keycloak_service: web::Data<KeycloakService>,
    refresh_token: web::Json<String>,
) -> Result<HttpResponse> {
    let tokens = keycloak_service.refresh_token(&refresh_token).await?;
    Ok(HttpResponse::Ok().json(tokens))
}

pub async fn logout(
    keycloak_service: web::Data<KeycloakService>,
    refresh_token: web::Json<String>,
) -> Result<HttpResponse> {
    keycloak_service.logout(&refresh_token).await?;
    Ok(HttpResponse::Ok().json("Logged out successfully"))
}