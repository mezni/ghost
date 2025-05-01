use actix_web::web;
use crate::handlers::countries::{
    list_countries,
    create_country,
    update_country,
    delete_country,
    search_countries,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(list_countries))
       .route("", web::post().to(create_country))
       .route("/{id}", web::put().to(update_country))
       .route("/{id}", web::delete().to(delete_country))
       .route("/search", web::get().to(search_countries));
}
