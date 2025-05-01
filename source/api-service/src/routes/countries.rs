use actix_web::web;
use crate::handlers::countries::*;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(list))
       .route("", web::post().to(create))
       .route("/{id}", web::put().to(update))
       .route("/{id}", web::delete().to(delete))
       .route("/search", web::get().to(search));
}
