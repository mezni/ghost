use crate::auth::jwt::validate_jwt;
use crate::auth::models::{Claims, UserRole};
use crate::core::errors::AppError;
use actix_web::{Error, HttpMessage, dev::ServiceRequest};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub async fn validator(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
        let token = credentials.token();

        match validate_jwt(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                Ok(req)
            }
            Err(_) => {
                let config = req.app_data::<Config>().cloned().unwrap_or_default();
                Err((AuthenticationError::from(config).into(), req))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Role(pub UserRole);

impl Role {
    pub fn super_admin() -> Self {
        Self(UserRole::SuperAdmin)
    }

    pub fn admin() -> Self {
        Self(UserRole::Admin)
    }

    pub fn operator() -> Self {
        Self(UserRole::Operator)
    }

    pub fn viewer() -> Self {
        Self(UserRole::Viewer)
    }

    pub fn has_permission(&self, required_role: &UserRole) -> bool {
        match self.0 {
            UserRole::SuperAdmin => true,
            UserRole::Admin => matches!(
                required_role,
                UserRole::Admin | UserRole::Operator | UserRole::Viewer
            ),
            UserRole::Operator => matches!(required_role, UserRole::Operator | UserRole::Viewer),
            UserRole::Viewer => matches!(required_role, UserRole::Viewer),
        }
    }
}

// Helper function to extract user from request
pub fn get_current_user(req: &ServiceRequest) -> Option<Claims> {
    req.extensions().get::<Claims>().cloned()
}

// Helper function to check if user has required role
pub fn has_required_role(req: &ServiceRequest, required_role: UserRole) -> bool {
    if let Some(claims) = get_current_user(req) {
        let user_role = claims.role;
        match user_role {
            UserRole::SuperAdmin => true,
            UserRole::Admin => matches!(
                required_role,
                UserRole::Admin | UserRole::Operator | UserRole::Viewer
            ),
            UserRole::Operator => matches!(required_role, UserRole::Operator | UserRole::Viewer),
            UserRole::Viewer => matches!(required_role, UserRole::Viewer),
        }
    } else {
        false
    }
}
