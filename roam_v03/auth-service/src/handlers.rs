use actix_web::{HttpResponse, Responder, web};
use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, SaltString};
use chrono::{Duration, Utc};
use deadpool_postgres::Pool;
use rand::rngs::OsRng;
use serde_json::json;

use crate::config::{Config, JwtConfig};
use crate::errors::AppError;
use crate::metrics::LOGIN_COUNTER;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, UserDTO};
use crate::repositories::{SessionRepository, UserRepository};
use crate::utils::{generate_jwt, verify_password};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout)),
    );
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("Auth service is healthy")
}

async fn register(
    db_pool: web::Data<Pool>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let client = db_pool.get().await?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)?
        .to_string();

    let user_repo = UserRepository::new(&client);
    let user = user_repo
        .create(&req.username, &req.email, &password_hash)
        .await?;

    Ok(HttpResponse::Created().json(UserDTO::from(user)))
}

async fn login(
    db_pool: web::Data<Pool>,
    jwt_config: web::Data<JwtConfig>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    LOGIN_COUNTER.inc();
    let client = db_pool.get().await?;
    let user_repo = UserRepository::new(&client);
    let user = user_repo.get_by_username(&req.username).await?;

    verify_password(&req.password, &user.password_hash)?;

    let jwt_secret = &jwt_config.secret;
    let expires_at = (Utc::now() + Duration::seconds(jwt_config.expires_in)).naive_utc();
    let expires_in_seconds = expires_at.timestamp() - Utc::now().timestamp();

    let token = generate_jwt(jwt_secret, user.id, expires_in_seconds)?;
    let session_repo = SessionRepository::new(&client);
    session_repo.create(user.id, &token, expires_at).await?;

    Ok(HttpResponse::Ok().json(AuthResponse { token, expires_at }))
}

async fn logout(
    db_pool: web::Data<Pool>,
    token: web::ReqData<String>, // assumed from middleware
) -> Result<HttpResponse, AppError> {
    let client = db_pool.get().await?;
    let session_repo = SessionRepository::new(&client);
    session_repo.delete_by_token(&token).await?;

    Ok(HttpResponse::Ok().json(json!({ "message": "Logged out successfully" })))
}
