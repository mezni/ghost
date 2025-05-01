pub mod countries;
pub mod carriers;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/countries").configure(countries::init));
    cfg.service(web::scope("/carriers").configure(carriers::init));
}
