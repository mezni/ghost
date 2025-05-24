use actix_web::{web, HttpResponse, Responder, get, post};
use crate::{
    errors::AppError,
    models::{RegisterRequest, LoginRequest, UserResponse, LoginResponse},
    repositories::UserRepository,
    utils::{hash_password, verify_password, generate_token}
};

#[get("/health")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "auth-service"
    }))
}

#[post("/register")]
async fn register(
    repo: web::Data<UserRepository>,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let user = repo.create_user(&form.into_inner()).await?;
    Ok(HttpResponse::Created().json(user))
}

#[post("/login")]
async fn login(
    repo: web::Data<UserRepository>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let creds = credentials.into_inner();
    
    // Get user from database
    let (user_id, hashed_pw) = repo.find_by_email(&creds.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;
    
    // Verify password
    if !verify_password(&creds.password, &hashed_pw)? {
        return Err(AppError::InvalidCredentials);
    }
    
    // Get user details
    let user = repo.get_user_by_id(user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;
    
    // Generate token
    let token = generate_token(user_id);
    
    Ok(HttpResponse::Ok().json(LoginResponse {
        token,
        user,
        expires_in: 3600, // 1 hour expiration
    }))
}

#[get("/users/{id}")]
async fn get_user(
    repo: web::Data<UserRepository>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user = repo.get_user_by_id(*id)
        .await?
        .ok_or(AppError::UserNotFound)?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(healthcheck)
            .service(register)
            .service(login)
            .service(get_user)
    );
}