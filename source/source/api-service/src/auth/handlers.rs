use crate::auth::models::{LoginRequest, RegisterRequest, UpdateRoleRequest, UserRole};
use crate::auth::service::AuthService;
use crate::core::errors::AppError;
use actix_web::{HttpResponse, get, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[post("/login")]
pub async fn login(
    auth_service: web::Data<AuthService>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let response = auth_service.login(&login_data).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/register")]
pub async fn register(
    auth_service: web::Data<AuthService>,
    register_data: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let response = auth_service.register(&register_data).await?;
    Ok(HttpResponse::Created().json(response))
}

#[get("/me")]
pub async fn get_current_user_info(
    auth_service: web::Data<AuthService>,
    bearer: BearerAuth,
) -> Result<HttpResponse, AppError> {
    let token = bearer.token();
    let user = auth_service.validate_token(token).await?;

    let user_response = crate::auth::models::UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        role: user.role,
        is_active: user.is_active,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "user": user_response
    })))
}

#[get("/permissions")]
pub async fn get_user_permissions(
    auth_service: web::Data<AuthService>,
    bearer: BearerAuth,
) -> Result<HttpResponse, AppError> {
    let token = bearer.token();
    let user = auth_service.validate_token(token).await?;

    // Use the cached method instead of the private DB method
    let permissions = auth_service.get_user_permissions_cached(user.id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "permissions": permissions
    })))
}

// Simple role check function for handlers
fn has_required_role(user_role: &UserRole, required_role: UserRole) -> bool {
    match user_role {
        UserRole::SuperAdmin => true,
        UserRole::Admin => matches!(
            required_role,
            UserRole::Admin | UserRole::Operator | UserRole::Viewer
        ),
        UserRole::Operator => matches!(required_role, UserRole::Operator | UserRole::Viewer),
        UserRole::Viewer => matches!(required_role, UserRole::Viewer),
    }
}

// Admin-only endpoints
#[post("/users/role")]
pub async fn update_user_role(
    auth_service: web::Data<AuthService>,
    bearer: BearerAuth,
    update_data: web::Json<UpdateRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let token = bearer.token();
    let current_user = auth_service.validate_token(token).await?;

    // Check if current user has admin permissions
    if !has_required_role(&current_user.role, UserRole::Admin) {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }

    // Check if trying to assign super_admin role (only super_admin can do this)
    if update_data.role == UserRole::SuperAdmin {
        if !has_required_role(&current_user.role, UserRole::SuperAdmin) {
            return Err(AppError::Forbidden(
                "Only super admin can assign super admin role".to_string(),
            ));
        }
    }

    let pool = auth_service.pool();
    let result =
        sqlx::query("UPDATE users SET role = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
            .bind(update_data.role.to_string())
            .bind(update_data.user_id)
            .execute(pool)
            .await
            .map_err(AppError::Sqlx)?;

    if result.rows_affected() > 0 {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "User role updated successfully"
        })))
    } else {
        Err(AppError::NotFound("User not found".to_string()))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(register)
            .service(
                web::scope("")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        crate::auth::middleware::AuthMiddleware::validator,
                    ))
                    .service(get_current_user_info)
                    .service(get_user_permissions)
                    .service(update_user_role),
            ),
    );
}
