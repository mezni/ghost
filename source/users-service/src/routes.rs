use actix_web::web;

use crate::user_handler::{create_user, get_user, get_users, update_user, delete_user, get_user_by_username};
use crate::auth_handler::{login, refresh_token, logout};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(login))
                    .route("/refresh", web::post().to(refresh_token))
                    .route("/logout", web::post().to(logout))
            )
            .service(
                web::scope("/users")
                    .route("", web::post().to(create_user))
                    .route("", web::get().to(get_users))
                    .route("/{id}", web::get().to(get_user))
                    .route("/{id}", web::put().to(update_user))
                    .route("/{id}", web::delete().to(delete_user))
                    .route("/username/{username}", web::get().to(get_user_by_username))
            )
    );
}