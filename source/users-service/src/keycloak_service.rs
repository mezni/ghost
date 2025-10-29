use reqwest::Client;
use serde_json::json;

use crate::core::config::Config;
use crate::core::error::{Result, AppError};
use crate::auth_model::{KeycloakTokenResponse, LoginResponse, KeycloakUserInfo};

#[derive(Clone)]
pub struct KeycloakService {
    client: Client,
    config: Config,
}

impl KeycloakService {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.keycloak_url, self.config.keycloak_realm
        );

        println!("Attempting login to: {}", token_url);

        let params = [
            ("client_id", self.config.keycloak_client_id.as_str()),
            ("client_secret", self.config.keycloak_client_secret.as_str()),
            ("username", username),
            ("password", password),
            ("grant_type", "password"),
        ];

        let response = self.client
            .post(&token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Keycloak request error: {}", e);
                AppError::KeycloakError(e.to_string())
            })?;

        println!("Keycloak response status: {}", response.status());

        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            eprintln!("Keycloak error response: {}", error_body);
            return Err(AppError::AuthError("Invalid credentials".to_string()));
        }

        let token_response: KeycloakTokenResponse = response
            .json()
            .await
            .map_err(|e| {
                eprintln!("Keycloak JSON parse error: {}", e);
                AppError::KeycloakError(e.to_string())
            })?;

        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
        })
    }

    // ... rest of the methods remain similar but update URLs
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<LoginResponse> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.keycloak_url, self.config.keycloak_realm
        );

        let params = [
            ("client_id", self.config.keycloak_client_id.as_str()),
            ("client_secret", self.config.keycloak_client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];

        let response = self.client
            .post(&token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::AuthError("Invalid refresh token".to_string()));
        }

        let token_response: KeycloakTokenResponse = response
            .json()
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
        })
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<()> {
        let logout_url = format!(
            "{}/realms/{}/protocol/openid-connect/logout",
            self.config.keycloak_url, self.config.keycloak_realm
        );

        let params = [
            ("client_id", self.config.keycloak_client_id.as_str()),
            ("client_secret", self.config.keycloak_client_secret.as_str()),
            ("refresh_token", refresh_token),
        ];

        let response = self.client
            .post(&logout_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::AuthError("Logout failed".to_string()));
        }

        Ok(())
    }

    pub async fn get_user_info(&self, access_token: &str) -> Result<KeycloakUserInfo> {
        let userinfo_url = format!(
            "{}/realms/{}/protocol/openid-connect/userinfo",
            self.config.keycloak_url, self.config.keycloak_realm
        );

        let response = self.client
            .get(&userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::AuthError("Invalid access token".to_string()));
        }

        let user_info: KeycloakUserInfo = response
            .json()
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        Ok(user_info)
    }

    pub async fn create_user_in_keycloak(
        &self,
        username: &str,
        email: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
    ) -> Result<String> {
        let admin_token = self.get_admin_token().await?;
        
        let create_user_url = format!(
            "{}/admin/realms/{}/users",
            self.config.keycloak_url, self.config.keycloak_realm
        );

        let user_data = json!({
            "username": username,
            "email": email,
            "firstName": first_name,
            "lastName": last_name,
            "enabled": true,
            "credentials": [{
                "type": "password",
                "value": password,
                "temporary": false
            }]
        });

        let response = self.client
            .post(&create_user_url)
            .bearer_auth(&admin_token)
            .json(&user_data)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::KeycloakError("Failed to create user in Keycloak".to_string()));
        }

        // Extract user ID from location header
        if let Some(location) = response.headers().get("location") {
            if let Ok(location_str) = location.to_str() {
                if let Some(user_id) = location_str.split('/').last() {
                    return Ok(user_id.to_string());
                }
            }
        }

        Err(AppError::KeycloakError("Failed to get user ID from Keycloak response".to_string()))
    }

    async fn get_admin_token(&self) -> Result<String> {
        let token_url = format!(
            "{}/realms/master/protocol/openid-connect/token",
            self.config.keycloak_url
        );

        // Note: In production, use proper admin credentials from config
        let params = [
            ("client_id", "admin-cli"),
            ("username", "admin"),
            ("password", "admin123"),
            ("grant_type", "password"),
        ];

        let response = self.client
            .post(&token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::AuthError("Failed to get admin token".to_string()));
        }

        let token_response: KeycloakTokenResponse = response
            .json()
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        Ok(token_response.access_token)
    }
}