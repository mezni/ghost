use std::convert::Infallible;
use std::sync::Arc;
use tracing::{info, instrument};
use uuid::Uuid;
use validator::Validate;
use warp::{Filter, Rejection, Reply};

use crate::{
    errors::AppError,
    models::{DeleteUserResponse, GetUsersQuery, UserListResponse},
    services::UserService,
};

type UserServiceType =
    Arc<UserService<dyn crate::repositories::UserRepository, dyn crate::services::KeycloakClient>>;

pub fn create_routes(
    user_service: UserServiceType,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let user_service = Arc::new(user_service);

    let health_route = warp::path!("health")
        .and(warp::get())
        .and_then(health_handler);

    let register_route = warp::path!("api" / "users" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_user_service(user_service.clone()))
        .and_then(register_handler);

    let get_users_route = warp::path!("api" / "users")
        .and(warp::get())
        .and(warp::query::<GetUsersQuery>())
        .and(require_auth())
        .and(with_user_service(user_service.clone()))
        .and_then(get_users_handler);

    let get_user_route = warp::path!("api" / "users" / Uuid)
        .and(warp::get())
        .and(require_auth())
        .and(with_user_service(user_service.clone()))
        .and_then(get_user_handler);

    let update_user_route = warp::path!("api" / "users" / Uuid)
        .and(warp::put())
        .and(warp::body::json())
        .and(require_auth())
        .and(with_user_service(user_service.clone()))
        .and_then(update_user_handler);

    let delete_user_route = warp::path!("api" / "users" / Uuid)
        .and(warp::delete())
        .and(require_auth_with_roles(vec!["admin".to_string()]))
        .and(with_user_service(user_service.clone()))
        .and_then(delete_user_handler);

    let get_profile_route = warp::path!("api" / "users" / "me" / "profile")
        .and(warp::get())
        .and(require_auth())
        .and(with_user_service(user_service.clone()))
        .and_then(get_profile_handler);

    let update_profile_route = warp::path!("api" / "users" / "me" / "profile")
        .and(warp::put())
        .and(warp::body::json())
        .and(require_auth())
        .and(with_user_service(user_service))
        .and_then(update_profile_handler);

    let public_routes = health_route.or(register_route);

    let protected_routes = get_users_route
        .or(get_user_route)
        .or(update_user_route)
        .or(delete_user_route)
        .or(get_profile_route)
        .or(update_profile_route);

    public_routes
        .or(protected_routes)
        .with(shared::middleware::cors().build())
        .with(warp::log("user_service"))
        .recover(crate::errors::handle_rejection)
}

#[instrument]
async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&serde_json::json!({
        "status": "healthy",
        "service": "user-service",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

#[instrument]
async fn register_handler(
    register_data: shared::models::RegisterRequest,
    user_service: UserServiceType,
) -> Result<impl Reply, Rejection> {
    // Validate input using shared validation
    if let Err(validation_errors) = register_data.validate() {
        return Err(warp::reject::custom(AppError::Validation(
            validation_errors.to_string(),
        )));
    }

    let user = user_service.register_user(&register_data).await?;

    Ok(warp::reply::json(&shared::models::ApiResponse::success(
        user,
    )))
}

#[instrument]
async fn get_users_handler(
    query: GetUsersQuery,
    _current_user: shared::models::User,
    user_service: UserServiceType,
) -> Result<impl Reply, Rejection> {
    // Validate query
    if let Err(validation_errors) = query.validate() {
        return Err(warp::reject::custom(AppError::Validation(
            validation_errors.to_string(),
        )));
    }

    let response = user_service.get_users(&query).await?;

    Ok(warp::reply::json(&shared::models::ApiResponse::success(
        response,
    )))
}

#[instrument]
async fn get_user_handler(
    user_id: Uuid,
    _current_user: shared::models::User,
    user_service: UserServiceType,
) -> Result<impl Reply, Rejection> {
    let user = user_service.get_user_by_id(user_id).await?;
    Ok(warp::reply::json(&shared::models::ApiResponse::success(
        user,
    )))
}

#[instrument]
async fn update_user_handler(
    user_id: Uuid,
    update_data: shared::models::UpdateUserRequest,
    current_user: shared::models::User,
    user_service: UserServiceType,
) -> Result<impl Reply, Rejection> {
    // Validate input
    if let Err(validation_errors) = update_data.validate() {
        return Err(warp::reject::custom(AppError::Validation(
            validation_errors.to_string(),
        )));
    }

    // Users can only update their own profile unless they're admin
    if user_id != current_user.id && !current_user.roles.contains(&"admin".to_string()) {
        return Err(warp::reject::custom(AppError::Forbidden));
    }

    let user = user_service.update_user(user_id, &update_data).await?;
    Ok(warp::reply::json(&shared::models::ApiResponse::success(
        user,
    )))
}

#[instrument]
async fn delete_user_handler(
    user_id: Uuid,
    _current_user: shared::models::User,
    user_service: UserServiceType,
) -> Result<impl Reply, Rejection> {
    user_service.delete_user(user_id).await?;

    let response = DeleteUserResponse {
        message: "User deleted successfully".to_string(),
        user_id,
    };

    Ok(warp::reply::json(&shared::models::ApiResponse::success(
        response,
    )))
}

#[instrument]
async fn get_profile_handler(
    current_user: shared::models::User,
    user_service: UserServiceType,
) -> Result<impl Reply, Rejection> {
    let user = user_service.get_user_profile(current_user.id).await?;
    Ok(warp::reply::json(&shared::models::ApiResponse::success(
        user,
    )))
}

#[instrument]
async fn update_profile_handler(
    update_data: shared::models::UpdateUserRequest,
    current_user: shared::models::User,
    user_service: UserServiceType,
) -> Result<impl Reply, Rejection> {
    // Validate input
    if let Err(validation_errors) = update_data.validate() {
        return Err(warp::reject::custom(AppError::Validation(
            validation_errors.to_string(),
        )));
    }

    let user = user_service
        .update_user_profile(current_user.id, &update_data)
        .await?;
    Ok(warp::reply::json(&shared::models::ApiResponse::success(
        user,
    )))
}

// Utility functions
fn with_user_service(
    user_service: UserServiceType,
) -> impl Filter<Extract = (UserServiceType,), Error = Infallible> + Clone {
    warp::any().map(move || user_service.clone())
}

fn require_auth() -> impl Filter<Extract = (shared::models::User,), Error = Rejection> + Clone {
    warp::header::<String>("authorization")
        .and_then(|token: String| async move { validate_token(&token).await })
}

fn require_auth_with_roles(
    required_roles: Vec<String>,
) -> impl Filter<Extract = (shared::models::User,), Error = Rejection> + Clone {
    require_auth().and_then(move |user: shared::models::User| {
        let required_roles = required_roles.clone();
        async move {
            for role in &required_roles {
                if user.roles.contains(role) {
                    return Ok(user);
                }
            }
            Err(warp::reject::custom(AppError::Forbidden))
        }
    })
}

async fn validate_token(token: &str) -> Result<shared::models::User, Rejection> {
    if !token.starts_with("Bearer ") {
        return Err(warp::reject::custom(AppError::Unauthorized));
    }

    // Mock token validation - in production, call auth service
    Ok(shared::models::User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        roles: vec!["user".to_string()],
        created_at: chrono::Utc::now(),
    })
}
