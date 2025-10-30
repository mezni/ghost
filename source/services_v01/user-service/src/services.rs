use std::sync::Arc;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::{
    errors::{AppError, Result},
    models::{GetUsersQuery, PaginationInfo, UpdateProfileRequest, UserEntity, UserListResponse},
    repositories::UserRepository,
};

#[async_trait::async_trait]
pub trait KeycloakClient: Send + Sync {
    async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<String>;
    async fn update_user_email(&self, user_id: &str, email: &str) -> Result<()>;
    async fn delete_user(&self, user_id: &str) -> Result<()>;
}

pub struct UserService<R, K> {
    user_repo: Arc<R>,
    keycloak_client: Arc<K>,
}

impl<R, K> UserService<R, K>
where
    R: UserRepository,
    K: KeycloakClient,
{
    pub fn new(user_repo: Arc<R>, keycloak_client: Arc<K>) -> Self {
        Self {
            user_repo,
            keycloak_client,
        }
    }

    #[instrument]
    pub async fn register_user(
        &self,
        register_data: &shared::models::RegisterRequest,
    ) -> Result<shared::models::User> {
        // Check if user already exists
        if self
            .user_repo
            .find_by_email(&register_data.email)
            .await
            .is_ok()
        {
            return Err(AppError::EmailTaken);
        }

        if self
            .user_repo
            .find_by_username(&register_data.username)
            .await
            .is_ok()
        {
            return Err(AppError::UsernameTaken);
        }

        // Create user in Keycloak
        let keycloak_user_id = self
            .keycloak_client
            .create_user(
                &register_data.username,
                &register_data.email,
                &register_data.password,
            )
            .await
            .map_err(|e| {
                error!("Failed to create user in Keycloak: {}", e);
                AppError::KeycloakUserCreation
            })?;

        // Create user in local database
        let user_entity = self
            .user_repo
            .create(
                &register_data.username,
                &register_data.email,
                &keycloak_user_id,
            )
            .await
            .map_err(|e| {
                error!("Failed to create user in database: {}", e);
                // Try to cleanup Keycloak user
                let _ = self.keycloak_client.delete_user(&keycloak_user_id).await;
                e
            })?;

        info!("User registered successfully: {}", user_entity.id);
        Ok(user_entity.into())
    }

    #[instrument]
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<shared::models::User> {
        let user_entity = self.user_repo.find_by_id(user_id).await?;
        Ok(user_entity.into())
    }

    #[instrument]
    pub async fn get_users(&self, query: &GetUsersQuery) -> Result<UserListResponse> {
        let users = self
            .user_repo
            .list(query.page.unwrap_or(1), query.limit.unwrap_or(50))
            .await?;

        let total = self.user_repo.count().await?;
        let limit = query.limit.unwrap_or(50);
        let page = query.page.unwrap_or(1);
        let total_pages = (total as f64 / limit as f64).ceil() as u32;

        Ok(UserListResponse {
            users: users.into_iter().map(|u| u.into()).collect(),
            pagination: PaginationInfo {
                page,
                limit,
                total,
                total_pages,
            },
        })
    }

    #[instrument]
    pub async fn update_user(
        &self,
        user_id: Uuid,
        update_data: &UpdateProfileRequest,
    ) -> Result<shared::models::User> {
        let user_entity = self.user_repo.update(user_id, update_data).await?;

        // Update in Keycloak if email changed
        if let Some(email) = &update_data.email {
            if let Err(e) = self
                .keycloak_client
                .update_user_email(&user_entity.keycloak_id, email)
                .await
            {
                error!("Failed to update user email in Keycloak: {}", e);
                // Continue anyway since the database update succeeded
            }
        }

        Ok(user_entity.into())
    }

    #[instrument]
    pub async fn delete_user(&self, user_id: Uuid) -> Result<()> {
        let user_entity = self.user_repo.find_by_id(user_id).await?;

        // Delete from Keycloak first
        self.keycloak_client
            .delete_user(&user_entity.keycloak_id)
            .await
            .map_err(|e| {
                error!("Failed to delete user from Keycloak: {}", e);
                AppError::KeycloakUserDeletion
            })?;

        // Delete from local database
        self.user_repo.delete(user_id).await?;

        info!("User deleted successfully: {}", user_id);
        Ok(())
    }

    #[instrument]
    pub async fn get_user_profile(&self, user_id: Uuid) -> Result<shared::models::User> {
        self.get_user_by_id(user_id).await
    }

    #[instrument]
    pub async fn update_user_profile(
        &self,
        user_id: Uuid,
        update_data: &UpdateProfileRequest,
    ) -> Result<shared::models::User> {
        self.update_user(user_id, update_data).await
    }
}
