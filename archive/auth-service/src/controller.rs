use crate::model::{LoginInfo, LoginResponse};
use axum::Json;
use axum::http::StatusCode;

pub async fn login_handler(
    Json(login_info): Json<LoginInfo>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let username = &login_info.username;
    let password = &login_info.password;
    let is_valid = is_valid_user(username, password);
    if (is_valid) {
        let claims = Claims {
            sub: username.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp as usize,
        };
        let token = match encode(
            &Header::default,
            &claims,
            &EncodingKey::from_secret.as_ref(),
        ) {
            Ok(tok) => tok,
            Err(e) => {
                eprintln!("Error generating token {:}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        Ok(Json(LoginResponse { token }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub fn is_valid_user(username: &str, password: &str) -> bool {
    username != "" && password != ""
}

pub async fn get_info_handler(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {}
    }
    Err(StatusCode::UNAUTHORIZED)
}
