// users/service.rs
use crate::core::auth::Auth;
use crate::core::errors::AppError;
use crate::users::models::{CreateUser, User, UserDTO};
use crate::users::repositories::UserRepository;
use bcrypt::{DEFAULT_COST, hash, verify};
use deadpool_postgres::Pool;

pub struct UserService;

impl UserService {
    /// Register a new user
    pub async fn register(pool: &Pool, input: CreateUser) -> Result<UserDTO, AppError> {
        // Check if user with same email already exists
        if let Some(_) = UserRepository::get_by_email(pool, &input.email).await? {
            return Err(AppError::Other(format!(
                "Email {} already in use",
                input.email
            )));
        }

        // Optionally check username uniqueness
        if let Some(_) = UserRepository::get_by_email(pool, &input.username).await? {
            return Err(AppError::Other(format!(
                "Username {} already in use",
                input.username
            )));
        }

        // Create user
        let user = UserRepository::create(pool, input).await?;
        Ok(UserDTO::from(user))
    }

    /// Login a user and return JWT
    pub async fn login(pool: &Pool, email: &str, password: &str) -> Result<String, AppError> {
        let user = UserRepository::get_by_email(pool, email)
            .await?
            .ok_or_else(|| AppError::Other("Invalid email or password".into()))?;

        // Verify password
        if !verify(password, &user.password_hash)
            .map_err(|e| AppError::Other(format!("Password verification error: {}", e)))?
        {
            return Err(AppError::Other("Invalid email or password".into()));
        }

        // Optionally check if user is active
        if !user.is_valid {
            return Err(AppError::Other("User account is not active".into()));
        }

        // Generate JWT
        let token = Auth::create_jwt(&user.email);
        Ok(token)
    }

    /// Get user info by ID
    pub async fn get_user(pool: &Pool, user_id: uuid::Uuid) -> Result<UserDTO, AppError> {
        let user = UserRepository::get_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::Other("User not found".into()))?;
        Ok(UserDTO::from(user))
    }

    /// Update user
    pub async fn update_user(pool: &Pool, user: User) -> Result<UserDTO, AppError> {
        let updated_user = UserRepository::update(pool, &user).await?;
        Ok(UserDTO::from(updated_user))
    }

    /// Delete user
    pub async fn delete_user(pool: &Pool, user_id: uuid::Uuid) -> Result<(), AppError> {
        UserRepository::delete(pool, user_id).await
    }
}
