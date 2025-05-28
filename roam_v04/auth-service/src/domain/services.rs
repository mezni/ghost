pub mod auth_service {
    use crate::{dto::auth_dto::{LoginRequest, AuthResponse}, repository::user_repository};

    pub async fn login_user(req: LoginRequest) -> Result<AuthResponse, crate::errors::AppError> {
        let user = user_repository::find_by_email(&req.email).await?;
        // Validate password and generate token
        Ok(AuthResponse::from(user))
    }

    pub async fn register_user(req: LoginRequest) -> Result<AuthResponse, crate::errors::AppError> {
        // Hash password and insert into DB
        let user = user_repository::create_user(req).await?;
        Ok(AuthResponse::from(user))
    }
}
