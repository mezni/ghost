pub mod countries;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/countries").configure(countries::init));
}
