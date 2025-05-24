use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{LoginRequest, LoginResponse, RegisterRequest};
use crate::repositories::{authenticate_user, create_user};
use actix_web::{HttpResponse, Responder, post, web};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

/// Claims for JWT payload (simplified)
#[derive(Debug, Serialize)]
struct Claims {
    sub: i32,
    role: String,
    exp: usize,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<DbPool>,
    form: web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    let user = create_user(&pool, form.into_inner()).await?;
    Ok(HttpResponse::Created().json(&user))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let user = authenticate_user(&pool, &form).await?;

    // Generate JWT token expiration (24h)
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        role: user.role.to_string(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::Other(e.to_string()))?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        token,
        user_id: user.id,
        role: user.role,
    }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").service(register).service(login));
}
