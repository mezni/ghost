use crate::auth::models::{
    AuthResponse, LoginRequest, RegisterRequest, User, UserResponse, UserRole,
};
use crate::auth::{jwt, password};
use crate::core::errors::AppError;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use validator::Validate;

#[derive(Clone)]
pub struct AuthService {
    pool: PgPool,
    permission_cache: Arc<RwLock<HashMap<uuid::Uuid, (Vec<String>, Instant)>>>,
    user_cache: Arc<RwLock<HashMap<uuid::Uuid, (User, Instant)>>>,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            permission_cache: Arc::new(RwLock::new(HashMap::new())),
            user_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn login(&self, login_data: &LoginRequest) -> Result<AuthResponse, AppError> {
        // Validate input
        if let Err(validation_errors) = login_data.validate() {
            return Err(AppError::BadRequest(validation_errors.to_string()));
        }

        // Use LOWER() for case-insensitive matching with index
        let user_row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, 
                role::text as role,
                is_active, created_at, updated_at
            FROM users 
            WHERE LOWER(username) = LOWER($1) AND is_active = true
            "#,
        )
        .bind(&login_data.username)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        let user_row = user_row
            .ok_or_else(|| AppError::Unauthorized("Invalid username or password".to_string()))?;

        // Extract user data from row
        let user = User {
            id: user_row.get("id"),
            username: user_row.get("username"),
            email: user_row.get("email"),
            password_hash: user_row.get("password_hash"),
            role: user_row
                .get::<String, _>("role")
                .parse()
                .map_err(|_| AppError::Internal("Invalid role in database".to_string()))?,
            is_active: user_row.get("is_active"),
            created_at: user_row.get("created_at"),
            updated_at: user_row.get("updated_at"),
        };

        // Verify password
        if !password::verify_password(&login_data.password, &user.password_hash)
            .map_err(|e| AppError::Internal(format!("Password verification error: {}", e)))?
        {
            return Err(AppError::Unauthorized(
                "Invalid username or password".to_string(),
            ));
        }

        // Create JWT token
        let token = jwt::create_jwt(&user)
            .map_err(|e| AppError::Internal(format!("Token creation error: {}", e)))?;

        // Get user permissions with caching
        let permissions = self.get_user_permissions_cached(user.id).await?;

        // Create UserResponse by cloning individual fields to avoid moving
        let user_response = UserResponse {
            id: user.id,
            username: user.username.clone(), // Clone the String
            email: user.email.clone(),       // Clone the String
            role: user.role,                 // Copy the enum (implements Copy)
            is_active: user.is_active,
        };

        // Cache the user data for future token validations
        self.cache_user(user).await; // Use the original user, no clone needed

        Ok(AuthResponse {
            success: true,
            message: "Login successful".to_string(),
            token: Some(token),
            user: Some(user_response),
            permissions,
        })
    }

    pub async fn register(
        &self,
        register_data: &RegisterRequest,
    ) -> Result<AuthResponse, AppError> {
        // Validate input
        if let Err(validation_errors) = register_data.validate() {
            return Err(AppError::BadRequest(validation_errors.to_string()));
        }

        // Use LOWER() for case-insensitive email/username checks
        let exists_row = sqlx::query(
            "SELECT COUNT(*) as count FROM users WHERE LOWER(username) = LOWER($1) OR LOWER(email) = LOWER($2)"
        )
        .bind(&register_data.username)
        .bind(&register_data.email)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        let count: i64 = exists_row.get("count");
        if count > 0 {
            return Err(AppError::BadRequest(
                "Username or email already exists".to_string(),
            ));
        }

        // Hash password
        let password_hash = password::hash_password(&register_data.password)
            .map_err(|e| AppError::Internal(format!("Password hashing error: {}", e)))?;

        let role = register_data.role.clone().unwrap_or(UserRole::Viewer);

        // Create user
        let user_row = sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash, role) 
            VALUES ($1, $2, $3, $4) 
            RETURNING 
                id, username, email, password_hash, 
                role::text as role,
                is_active, created_at, updated_at
            "#,
        )
        .bind(&register_data.username)
        .bind(&register_data.email)
        .bind(&password_hash)
        .bind(role.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create user: {}", e)))?;

        // Extract user data
        let user = User {
            id: user_row.get("id"),
            username: user_row.get("username"),
            email: user_row.get("email"),
            password_hash: user_row.get("password_hash"),
            role: user_row
                .get::<String, _>("role")
                .parse()
                .map_err(|_| AppError::Internal("Invalid role in database".to_string()))?,
            is_active: user_row.get("is_active"),
            created_at: user_row.get("created_at"),
            updated_at: user_row.get("updated_at"),
        };

        // Create JWT token
        let token = jwt::create_jwt(&user)
            .map_err(|e| AppError::Internal(format!("Token creation error: {}", e)))?;

        // Get user permissions with caching
        let permissions = self.get_user_permissions_cached(user.id).await?;

        // Create UserResponse by cloning individual fields
        let user_response = UserResponse {
            id: user.id,
            username: user.username.clone(), // Clone the String
            email: user.email.clone(),       // Clone the String
            role: user.role,                 // Copy the enum
            is_active: user.is_active,
        };

        // Cache the new user
        self.cache_user(user).await; // Use the original user, no clone needed

        Ok(AuthResponse {
            success: true,
            message: "Registration successful".to_string(),
            token: Some(token),
            user: Some(user_response),
            permissions,
        })
    }

    // Cached version of get_user_permissions
    pub async fn get_user_permissions_cached(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<String>, AppError> {
        // Check cache first
        {
            let cache = self.permission_cache.read().await;
            if let Some((permissions, timestamp)) = cache.get(&user_id) {
                if timestamp.elapsed() < Duration::from_secs(300) {
                    // 5 minute cache
                    return Ok(permissions.clone());
                }
            }
        }

        // Cache miss - query database
        let permissions = self.get_user_permissions_db(user_id).await?;

        // Update cache
        {
            let mut cache = self.permission_cache.write().await;
            cache.insert(user_id, (permissions.clone(), Instant::now()));
        }

        Ok(permissions)
    }

    // Direct database query for permissions
    async fn get_user_permissions_db(&self, user_id: uuid::Uuid) -> Result<Vec<String>, AppError> {
        let permission_rows = sqlx::query(
            "SELECT permission_key FROM user_permissions WHERE user_id = $1 AND granted = true",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        let permissions = permission_rows
            .into_iter()
            .map(|row| row.get("permission_key"))
            .collect();

        Ok(permissions)
    }

    pub async fn get_user_by_id(&self, user_id: uuid::Uuid) -> Result<Option<User>, AppError> {
        // Check cache first
        {
            let cache = self.user_cache.read().await;
            if let Some((user, timestamp)) = cache.get(&user_id) {
                if timestamp.elapsed() < Duration::from_secs(60) {
                    // 1 minute cache
                    return Ok(Some(user.clone()));
                }
            }
        }

        // Cache miss - query database
        let user_row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, 
                role::text as role,
                is_active, created_at, updated_at
            FROM users 
            WHERE id = $1 AND is_active = true
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Sqlx)?;

        let user = match user_row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    username: row.get("username"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role: row
                        .get::<String, _>("role")
                        .parse()
                        .map_err(|_| AppError::Internal("Invalid role in database".to_string()))?,
                    is_active: row.get("is_active"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Some(user)
            }
            None => None,
        };

        // Cache the result if user exists - clone the Option content
        if let Some(ref user) = user {
            self.cache_user(user.clone()).await;
        }

        Ok(user)
    }

    // Helper method to cache user
    async fn cache_user(&self, user: User) {
        let mut cache = self.user_cache.write().await;
        cache.insert(user.id, (user, Instant::now()));
    }

    pub async fn validate_token(&self, token: &str) -> Result<User, AppError> {
        let claims = jwt::validate_jwt(token)
            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

        let user = self
            .get_user_by_id(claims.sub)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        Ok(user)
    }

    // Cache management methods
    pub async fn clear_user_cache(&self, user_id: Option<uuid::Uuid>) {
        let mut cache = self.user_cache.write().await;
        if let Some(user_id) = user_id {
            cache.remove(&user_id);
        } else {
            cache.clear();
        }
    }

    pub async fn clear_permission_cache(&self, user_id: Option<uuid::Uuid>) {
        let mut cache = self.permission_cache.write().await;
        if let Some(user_id) = user_id {
            cache.remove(&user_id);
        } else {
            cache.clear();
        }
    }

    // Add this method to expose pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
