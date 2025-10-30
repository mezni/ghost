use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::sync::Arc;

use crate::{
    config::Config,
    errors::Result,
    repositories::{PostgresUserRepository, UserRepository},
};

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|e| crate::errors::AppError::Database(e))?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| crate::errors::AppError::Database(e))?;

        Ok(())
    }

    pub fn user_repository(&self) -> Arc<impl UserRepository> {
        Arc::new(PostgresUserRepository::new(self.pool.clone()))
    }

    pub async fn keycloak_client(
        &self,
        config: &Config,
    ) -> Result<Arc<impl crate::services::KeycloakClient>> {
        let client = KeycloakAdminClient::new(
            config.keycloak_url.clone(),
            config.keycloak_realm.clone(),
            config.keycloak_client_id.clone(),
            config.keycloak_client_secret.clone(),
            config.keycloak_admin_user.clone(),
            config.keycloak_admin_password.clone(),
        );

        Ok(Arc::new(client))
    }
}

// Keycloak client implementation
pub struct KeycloakAdminClient {
    client: reqwest::Client,
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    admin_user: String,
    admin_password: String,
    access_token: Option<String>,
}

impl KeycloakAdminClient {
    pub fn new(
        base_url: String,
        realm: String,
        client_id: String,
        client_secret: String,
        admin_user: String,
        admin_password: String,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            realm,
            client_id,
            client_secret,
            admin_user,
            admin_password,
            access_token: None,
        }
    }

    async fn get_access_token(&mut self) -> Result<String> {
        if let Some(token) = &self.access_token {
            return Ok(token.clone());
        }

        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let params = [
            ("grant_type", "password"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("username", &self.admin_user),
            ("password", &self.admin_password),
        ];

        let response = self
            .client
            .post(&token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?;

        let access_token = response["access_token"]
            .as_str()
            .ok_or_else(|| {
                crate::errors::AppError::Keycloak("No access token in response".to_string())
            })?
            .to_string();

        self.access_token = Some(access_token.clone());
        Ok(access_token)
    }
}

#[async_trait::async_trait]
impl crate::services::KeycloakClient for KeycloakAdminClient {
    async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<String> {
        let mut self_mut = self.clone_method();
        let token = self_mut.get_access_token().await?;

        let user_url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);

        let user_data = serde_json::json!({
            "username": username,
            "email": email,
            "enabled": true,
            "emailVerified": true,
            "credentials": [{
                "type": "password",
                "value": password,
                "temporary": false
            }]
        });

        let response = self
            .client
            .post(&user_url)
            .bearer_auth(&token)
            .json(&user_data)
            .send()
            .await
            .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?;

        if response.status().is_success() {
            if let Some(location) = response.headers().get("Location") {
                let location_str = location
                    .to_str()
                    .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?;
                let user_id = location_str.split('/').last().ok_or_else(|| {
                    crate::errors::AppError::Keycloak("Invalid location header".to_string())
                })?;
                return Ok(user_id.to_string());
            }
        }

        let error_text = response
            .text()
            .await
            .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?;
        Err(crate::errors::AppError::Keycloak(format!(
            "Failed to create user: {}",
            error_text
        )))
    }

    async fn update_user_email(&self, user_id: &str, email: &str) -> Result<()> {
        let mut self_mut = self.clone_method();
        let token = self_mut.get_access_token().await?;

        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, user_id
        );

        let update_data = serde_json::json!({
            "email": email
        });

        let response = self
            .client
            .put(&user_url)
            .bearer_auth(&token)
            .json(&update_data)
            .send()
            .await
            .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?;
            Err(crate::errors::AppError::Keycloak(format!(
                "Failed to update user email: {}",
                error_text
            )))
        }
    }

    async fn delete_user(&self, user_id: &str) -> Result<()> {
        let mut self_mut = self.clone_method();
        let token = self_mut.get_access_token().await?;

        let user_url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, user_id
        );

        let response = self
            .client
            .delete(&user_url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .map_err(|e| crate::errors::AppError::ExternalService(e.to_string()))?;
            Err(crate::errors::AppError::Keycloak(format!(
                "Failed to delete user: {}",
                error_text
            )))
        }
    }
}

// Helper to work around &mut self requirements
impl KeycloakAdminClient {
    fn clone_method(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            realm: self.realm.clone(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            admin_user: self.admin_user.clone(),
            admin_password: self.admin_password.clone(),
            access_token: self.access_token.clone(),
        }
    }
}
