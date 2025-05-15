use actix_web::{post, web, HttpResponse, Responder};
use crate::{models::*, jwt, db, errors::AppError};

#[post("/login")]
pub async fn login(
    pool: web::Data<sqlx::PgPool>,
    data: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let user = db::find_user(&pool, &data.username).await?;
    if !db::verify_password(&data.password, &user.password_hash)? {
        return Err(AppError::unauthorized("Invalid credentials"));
    }

    let token = jwt::generate_token(&user)?;
    Ok(web::Json(LoginResponse {
        access_token: token,
        expires_in: 3600,
    }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
